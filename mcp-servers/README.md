# MCP servers

Two Rust MCP servers ship with the combined plugin. Each server's source and
pre-built binaries live in its own repository, mounted here as a submodule.
This directory holds the platform wrappers, install/upgrade helpers, and consumer
tests. The combined plugin's `plugin.json` points at those wrappers.

| Path | Repo | MCP server name | Wrapper |
| --- | --- | --- | --- |
| `stepwise/` | [jw409/kp-stepwise](https://github.com/jw409/kp-stepwise) | `kp-stepwise` | `bin/kp-stepwise` |
| `github/` | [jw409/kp-github](https://github.com/jw409/kp-github) | `kp-github` | `bin/kp-github-mcp` |

`bin/<name>` detects the platform and executes
`../<server>/bin/<platform>/<binary>` inside the submodule. Both the submodule
checkout and the executable for the current platform must be present.

## The combined plugin requires submodules at runtime

`plugin.json` points each server at `${CLAUDE_PLUGIN_ROOT}/mcp-servers/bin/<name>`,
and that wrapper resolves into the submodule. For a manual checkout, initialize
the pinned commits:

```sh
git submodule update --init -- mcp-servers/github mcp-servers/stepwise
```

Both repositories must be reachable by the installer. The standalone `kp-github`
marketplace entry instead loads `jw409/kp-github` directly and runs its
`bin/kp-github-mcp` wrapper; it does not use this repository's submodule pointer.

Keep source and both platform binaries in the same reviewed server revision.
Before updating a submodule pointer, verify the build revision and artifacts;
merely storing binaries beside source does not establish that they match.

## Installing and upgrading

`./mcp-servers/install.sh` initializes the pinned submodules and prefers their
Linux x86_64 or macOS arm64 pre-built binaries. It registers stable wrappers for
pre-built installs, and requires no Rust toolchain on those platforms. If a
pre-built binary is missing, that server is built with `cargo build --locked
--release`. `--build` builds both servers from source and uses their
`target/release` paths. A failed build stops installation even if an older binary
is present.

Existing Claude MCP registrations are preserved, including account/profile
environment settings. Missing registrations are added. Use `--no-register` to
prepare binaries and print registration commands without invoking the MCP host.
For a new kp-github registration, selected `KP_GITHUB_ACCOUNT` and
`KP_GITHUB_EVENTS_CONFIG` values are retained in its MCP environment. Tokens and
other shell environment values are not copied into that registration.

`./mcp-servers/upgrade.sh` fast-forwards this checkout from `origin/main`, then
updates submodules to the recorded commits and prepares the binaries. It never
uses `--remote` or `--force`, and failures stop the upgrade. If a submodule already
has a `target/release` binary, both source targets are rebuilt to keep prior
source installations current; `--build` also requests that behavior explicitly.
Upgrade preserves MCP registrations. Restart the MCP host afterward.

## GitHub developer events

The optional `github_events` client registers, lists, removes, reads/waits for,
and explicitly acknowledges PR approval, merge, and CI failure listeners. Reads
preserve event provenance and opaque ack tokens. Production use requires the
operator's deployed consumer API, verified identity mapping, and published
configuration. A configured local profile alone does not prove live delivery.

Each Meshly developer uses their own existing GitHub CLI login and Google login.
The operator supplies one configuration bundle or an exact readable GCS object;
developers do not need another GitHub App or its signing key. Run the upstream
helper from this checkout:

```sh
python3 mcp-servers/github/scripts/github_events_setup.py setup \
  --gcs-object gs://YOUR-CONFIG-BUCKET/YOUR-KNOWN-OBJECT \
  --github-account jw409 \
  --google-account developer@meshly.ai
```

The GCS path is a placeholder for the operator's actual object. Setup reads that
known object without listing the bucket. Copy/paste via `--bundle` is also
supported; valid cached configuration is reused until explicitly refreshed or
changed. See the pinned upstream [setup and bundle contract](github/docs/developer-events-setup.md),
[account selection](github/docs/account-selection.md), and
[proposed receiver API](github/docs/listener-api.md).

Example MCP server configuration, using absolute paths for your machine:

```json
{
  "mcpServers": {
    "kp-github": {
      "command": "/absolute/path/to/kinderpowers/mcp-servers/bin/kp-github-mcp",
      "args": [],
      "env": {
        "KP_GITHUB_ACCOUNT": "jw409",
        "KP_GITHUB_EVENTS_CONFIG": "/home/you/.config/kp-github/events.json"
      }
    }
  }
}
```

These selectors do not change machine-wide GitHub or Google defaults. Existing
GitHub tools use the selected developer's credential; event requests use the
profile's Google identity. The receiver independently verifies access. The
consumer endpoint differs from GitHub's webhook ingress, and a human gcloud
token's accepted audience must come from the receiver operator. The bundle
contains configuration, never App keys, webhook secrets, or personal tokens.
Open source adopters operate their own app and receiver unless Meshly explicitly
onboards their organization.

## Working on a server

```sh
git submodule update --init -- mcp-servers/stepwise  # or mcp-servers/github
cd mcp-servers/stepwise && cargo test
```

Both repositories are public and `.gitmodules` records HTTPS URLs.

## Releasing

Build and review artifacts in each server's own repository, using its build
workflow. For kp-github, retain the Linux x86_64 and macOS arm64 binaries from the
same source revision, validate the packaged MCP tool surface, and publish a commit
containing both refreshed executable files. Bump `mcp-servers/github` to that
exact published commit; a source-only change does not update packaged installs.
See the upstream [binary workflow](https://github.com/jw409/kp-github/blob/main/.github/workflows/build-binaries.yml).

This repository tests the consumer wrappers/installers plus the pinned upstream
Rust and Python suites. Run the isolated shell integration tests locally with:

```sh
python3 -m unittest discover -s mcp-servers/tests -p 'test_*.py'
```

Those tests use fake Git, cargo, and MCP host commands; they do not register real
servers, fetch repositories, or validate deployment of an event receiver.
