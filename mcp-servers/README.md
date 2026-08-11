# MCP servers

Two Rust MCP servers ship with the plugin. Each one's **source and its
pre-built binaries both live in its own repo**, mounted here as a submodule.
This directory holds only the platform wrappers that `plugin.json` points at.

| Path | Repo | MCP server name | Wrapper |
| --- | --- | --- | --- |
| `stepwise/` | [jw409/kp-stepwise](https://github.com/jw409/kp-stepwise) | `kp-stepwise` | `bin/kp-stepwise` |
| `github/` | [jw409/kp-github](https://github.com/jw409/kp-github) | `kp-github` | `bin/kp-github-mcp` |

`bin/<name>` detects the platform and execs
`../<server>/bin/<platform>/<binary>` inside the submodule. If the submodule
isn't checked out it says so and exits 1, rather than failing on a missing path.

## Consequence: the submodules are required to run, not just to build

`plugin.json` points each server at `${CLAUDE_PLUGIN_ROOT}/mcp-servers/bin/<name>`,
and that wrapper now resolves into the submodule. **A plugin install that skips
submodules gets two dead MCP servers.** Claude Code clones marketplaces with
`--recurse-submodules`, so a normal install is fine — but both server repos have
to be reachable by whoever is installing.

Keeping each binary next to the source that produced it is the point: one repo,
one server, one release. Nothing has to be copied across a repo boundary to cut
a version, and a binary can never drift from the source it claims to be built
from.

## Working on a server

```sh
git submodule update --init mcp-servers/stepwise   # or mcp-servers/github
cd mcp-servers/stepwise && cargo test
```

Both repos are public and `.gitmodules` records HTTPS URLs, so this needs no
credentials.

## Releasing

Binaries are rebuilt by each server repo's own `build-binaries.yml` on a `v*`
tag, which commits the refreshed `bin/` back to that repo's `main`. Then bump
the submodule pointer here. This repo no longer builds server binaries — the
former `build-mcp-servers.yml` was removed in favour of the per-repo workflows.

`install.sh` and `upgrade.sh` build out of these same paths and need no changes.
