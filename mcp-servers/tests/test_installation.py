"""Exercise the real shell entrypoints with isolated fake executables.

These tests never fetch Git repositories, compile Rust, contact GitHub, or change
an MCP host registration. Executable routing and argument/environment preservation
are tested by actually invoking the shipped wrappers and installer scripts.
"""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SERVERS = {"github": "kp-github-mcp", "stepwise": "kp-stepwise"}

FAKE_TOOL = r'''
import json, os, sys
from pathlib import Path
name = Path(sys.argv[0]).name
args = sys.argv[1:]
record = {"command":name,"args":args,"cwd":str(Path.cwd()),
          "account":os.environ.get("KP_GITHUB_ACCOUNT"),
          "profile":os.environ.get("KP_GITHUB_EVENTS_CONFIG"),
          "rust_stack":os.environ.get("RUST_MIN_STACK")}
with open(os.environ["FAKE_LOG"], "a") as log:
    log.write(json.dumps(record) + "\n")
if name == "uname":
    print(os.environ.get("FAKE_OS", "Linux") if args == ["-s"] else os.environ.get("FAKE_ARCH", "x86_64"))
elif name == "git":
    failure = os.environ.get("FAKE_GIT_FAIL")
    if failure and failure in args:
        print("synthetic git failure", file=sys.stderr)
        sys.exit(12)
elif name == "cargo":
    if os.environ.get("FAKE_CARGO_FAIL") == Path.cwd().name:
        print("synthetic cargo failure", file=sys.stderr)
        sys.exit(23)
    if not os.environ.get("FAKE_CARGO_NO_OUTPUT"):
        binary = "kp-github-mcp" if Path.cwd().name == "github" else "kp-stepwise"
        output = Path("target/release") / binary
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text("#!/bin/sh\nexit 0\n")
        output.chmod(0o755)
elif name == "claude":
    if args[:2] == ["mcp", "get"]:
        sys.exit(0 if args[2] in os.environ.get("FAKE_REGISTERED", "").split(",") else 1)
    if args[:2] == ["mcp", "add"]:
        sys.exit(17 if os.environ.get("FAKE_CLAUDE_ADD_FAIL") == args[2] else 0)
    raise SystemExit("Unexpected Claude command: " + repr(args))
else:
    raise SystemExit("Unexpected fake tool: " + name)
'''


class InstallationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="kp-consumer-")
        self.addCleanup(self.temp.cleanup)
        # Spaces in the checkout and profile paths catch quoting regressions.
        self.root = Path(self.temp.name) / "checkout with spaces"
        self.mcp = self.root / "mcp-servers"
        (self.mcp / "bin").mkdir(parents=True)
        for script in ("install.sh", "upgrade.sh"):
            shutil.copy2(ROOT / "mcp-servers" / script, self.mcp / script)
        for repo, binary in SERVERS.items():
            (self.mcp / repo).mkdir()
            shutil.copy2(ROOT / "mcp-servers/bin" / binary, self.mcp / "bin" / binary)
        self.commands = Path(self.temp.name) / "commands"
        self.commands.mkdir()
        self.log = Path(self.temp.name) / "calls.jsonl"
        for name in ("git", "cargo", "claude", "uname"):
            path = self.commands / name
            path.write_text(f"#!{sys.executable}\n" + FAKE_TOOL)
            path.chmod(0o755)
        (self.commands / "dirname").symlink_to(shutil.which("dirname"))
        self.env = {
            "PATH": str(self.commands), "FAKE_LOG": str(self.log),
            "KP_GITHUB_ACCOUNT": "jw409",
            "KP_GITHUB_EVENTS_CONFIG": "/private profile/events.json",
        }

    def calls(self, command=None):
        records = [json.loads(line) for line in self.log.read_text().splitlines()] if self.log.exists() else []
        return [r for r in records if r["command"] == command] if command else records

    def run_script(self, script, *args):
        return subprocess.run([str(self.mcp / script), *args], env=self.env,
                              cwd=self.temp.name, text=True, capture_output=True, timeout=10)

    def prebuilts(self):
        for repo, binary in SERVERS.items():
            for platform in ("linux-x86_64", "macos-arm64"):
                path = self.mcp / repo / "bin" / platform / binary
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(f"#!{sys.executable}\nimport json, os, sys\n"
                    f"print(json.dumps({{'platform': {platform!r}, 'binary': {binary!r}, "
                    "'argv': sys.argv[1:], 'account': os.environ.get('KP_GITHUB_ACCOUNT'), "
                    "'profile': os.environ.get('KP_GITHUB_EVENTS_CONFIG')}))\n")
                path.chmod(0o755)

    def assert_pinned_submodules(self):
        calls = [call for call in self.calls("git") if "submodule" in call["args"]]
        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0]["args"], ["-C", str(self.root), "submodule", "update", "--init",
                                          "--", "mcp-servers/github", "mcp-servers/stepwise"])

    def test_linux_and_macos_wrappers_preserve_arguments_and_profile(self):
        self.prebuilts()
        for system, arch, platform in (("Linux", "x86_64", "linux-x86_64"), ("Darwin", "arm64", "macos-arm64")):
            with self.subTest(platform=platform):
                self.env.update(FAKE_OS=system, FAKE_ARCH=arch)
                result = self.run_script("bin/kp-github-mcp", "argument with spaces", "literal-$HOME")
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(json.loads(result.stdout), {
                    "platform": platform, "binary": "kp-github-mcp",
                    "argv": ["argument with spaces", "literal-$HOME"],
                    "account": "jw409", "profile": "/private profile/events.json"})

    def test_missing_submodule_binary_does_not_fall_back_to_another_binary(self):
        result = self.run_script("bin/kp-github-mcp")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("kp-github-mcp", result.stderr)

    def test_installer_uses_submodule_prebuilts_and_registers_stable_wrappers(self):
        self.prebuilts()
        self.env.update(GH_TOKEN="synthetic-gh-token", GITHUB_TOKEN="synthetic-legacy-token")
        (self.commands / "cargo").unlink()  # No Rust toolchain is required.
        result = self.run_script("install.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assert_pinned_submodules()
        adds = [call for call in self.calls("claude") if call["args"][1] == "add"]
        self.assertEqual([call["args"] for call in adds], [
            ["mcp", "add", "kp-github", "--transport", "stdio",
             "--env", "KP_GITHUB_ACCOUNT=jw409",
             "--env", "KP_GITHUB_EVENTS_CONFIG=/private profile/events.json",
             "--", str(self.mcp / "bin/kp-github-mcp")],
            ["mcp", "add", "kp-stepwise", "--transport", "stdio", "--", str(self.mcp / "bin/kp-stepwise")]])
        self.assertTrue(all(call["account"] == "jw409" for call in adds))

    def test_installer_preserves_existing_registration_environment(self):
        self.prebuilts()
        self.env["FAKE_REGISTERED"] = "kp-github,kp-stepwise"
        result = self.run_script("install.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(all(call["args"][1] == "get" for call in self.calls("claude")))
        self.assertIn("registration preserved", result.stdout)

    def test_no_register_mode_never_invokes_host_cli(self):
        self.prebuilts()
        result = self.run_script("install.sh", "--no-register")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.calls("claude"), [])
        self.assertIn("claude mcp add kp-github", result.stdout)

    def test_missing_one_prebuilt_builds_only_that_server(self):
        self.prebuilts()
        (self.mcp / "github/bin/linux-x86_64/kp-github-mcp").unlink()
        result = self.run_script("install.sh", "--no-register")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual([Path(c["cwd"]).name for c in self.calls("cargo")], ["github"])
        self.assertEqual(self.calls("cargo")[0]["args"], ["build", "--locked", "--release"])
        self.assertEqual(self.calls("cargo")[0]["rust_stack"], "16777216")

    def test_failed_source_build_does_not_accept_stale_binary_or_register(self):
        stale = self.mcp / "github/target/release/kp-github-mcp"
        stale.parent.mkdir(parents=True)
        stale.write_text("stale binary")
        stale.chmod(0o755)
        self.env["FAKE_CARGO_FAIL"] = "github"
        result = self.run_script("install.sh", "--build")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(stale.read_text(), "stale binary")
        self.assertEqual(self.calls("claude"), [])
        self.assertIn("build failed", result.stderr)

    def test_success_without_output_is_not_reported_as_success(self):
        self.env["FAKE_CARGO_NO_OUTPUT"] = "1"
        result = self.run_script("install.sh", "--build")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("without producing an executable", result.stderr)
        self.assertEqual(self.calls("claude"), [])

    def test_submodule_failure_stops_before_build_or_registration(self):
        self.prebuilts()
        self.env["FAKE_GIT_FAIL"] = "submodule"
        result = self.run_script("install.sh")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.calls("cargo"), [])
        self.assertEqual(self.calls("claude"), [])

    def test_upgrade_updates_pins_even_without_new_parent_commit_and_keeps_host_config(self):
        self.prebuilts()
        result = self.run_script("upgrade.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.calls("git")[0]["args"], ["-C", str(self.root), "pull", "--ff-only", "origin", "main"])
        self.assert_pinned_submodules()
        self.assertEqual(self.calls("cargo"), [])
        self.assertEqual(self.calls("claude"), [])

    def test_upgrade_failed_pull_stops_before_submodule_update(self):
        self.prebuilts()
        self.env["FAKE_GIT_FAIL"] = "pull"
        result = self.run_script("upgrade.sh")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(self.calls("git")), 1)
        self.assertEqual(self.calls("claude"), [])

    def test_upgrade_rebuilds_existing_source_installation(self):
        self.prebuilts()
        stale = self.mcp / "github/target/release/kp-github-mcp"
        stale.parent.mkdir(parents=True)
        stale.write_text("old source build")
        result = self.run_script("upgrade.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual([Path(c["cwd"]).name for c in self.calls("cargo")], ["github", "stepwise"])
        self.assertNotEqual(stale.read_text(), "old source build")
        self.assertEqual(self.calls("claude"), [])

    def test_upgrade_propagates_build_failure(self):
        self.prebuilts()
        self.env["FAKE_CARGO_FAIL"] = "stepwise"
        result = self.run_script("upgrade.sh", "--build")
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("Pinned binaries prepared", result.stdout)
        self.assertEqual(self.calls("claude"), [])

    def test_registration_failure_is_not_hidden(self):
        self.prebuilts()
        self.env["FAKE_CLAUDE_ADD_FAIL"] = "kp-github"
        result = self.run_script("install.sh")
        self.assertEqual(result.returncode, 17)
        self.assertNotIn("Restart your MCP host", result.stdout)


if __name__ == "__main__":
    unittest.main()
