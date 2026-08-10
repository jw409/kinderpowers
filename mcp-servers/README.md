# MCP servers

Two Rust MCP servers ship with the plugin. Their **sources live in their own
repos**, mounted here as submodules; their **pre-built binaries stay in this
repo**, committed under `bin/`.

| Path | Repo | MCP server name | Binary |
| --- | --- | --- | --- |
| `stepwise/` | [jw409/kp-stepwise](https://github.com/jw409/kp-stepwise) | `kp-stepwise` | `bin/kp-stepwise` |
| `github/` | [jw409/kp-github](https://github.com/jw409/kp-github) | `kp-github` | `bin/kp-github-mcp` |

## Why the binaries stay here

`.claude-plugin/plugin.json` points each server at
`${CLAUDE_PLUGIN_ROOT}/mcp-servers/bin/<name>`, and Claude Code installs a
plugin by cloning its marketplace. Keeping the binaries in this repo means an
install needs **no submodule access at all** — the submodules are a *build-time*
dependency, not a runtime one. Only someone rebuilding a server needs them
checked out.

That split is deliberate: it decouples "can I run the plugin" from "can I read
the server sources", so the two repos can have their own visibility and release
cadence without the plugin install depending on either.

## Working on a server

```sh
git submodule update --init mcp-servers/stepwise   # or mcp-servers/github
cd mcp-servers/stepwise && cargo test
```

**While either source repo is private,** `.gitmodules` records an HTTPS URL that
an anonymous clone cannot reach, so the init above fails with `Repository not
found`. Override the URL locally (this does not touch `.gitmodules`):

```sh
git submodule init mcp-servers/stepwise
git config submodule."mcp-servers/stepwise".url git@github.com:jw409/kp-stepwise.git
git submodule update mcp-servers/stepwise
```

The HTTPS URL is kept as the recorded one because it is what works for everyone
once the repos are public. Nothing about a plugin *install* depends on this —
only rebuilding does.

Commit and push source changes **in the submodule repo**; then bump the pointer
here in a separate commit. `install.sh` and `upgrade.sh` build out of these same
paths and need no changes — they prefer `bin/` and fall back to `cargo build`.

Binaries are refreshed by `.github/workflows/build-mcp-servers.yml` on an
`mcp-v*` tag, which commits the rebuilt `bin/` back to `main`.
