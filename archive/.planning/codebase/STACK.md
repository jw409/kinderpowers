# Technology Stack

**Analysis Date:** 2026-03-19

## Languages

**Primary:**
- Rust (2021 edition) - Two MCP servers: `mcp-servers/sequential-thinking/` and `mcp-servers/github/`
- JavaScript (CommonJS / ES modules) - GSD lifecycle engine: `gsd/bin/lib/*.cjs`, `gsd/hooks/*.js`, `gsd/bin/install.js`

**Secondary:**
- Python 3.x - Hook scripts: `hooks/agent-outcome-logger.py`; scanner: `scanner.py`, `test_scanner.py`
- Markdown / XML - Skills (`skills/*/`), agents (`agents/*.md`), KINDERPOWERS manifest (`KINDERPOWERS.xml`)
- Bash - Setup and install scripts: `setup.sh`, `mcp-servers/sequential-thinking/install.sh`, `mcp-servers/github/install.sh`, `mcp-servers/bin/upgrade.sh`

## Runtime

**Environment:**
- Node.js >=20.0.0 (required by `gsd/package.json` engines field)
- Tokio async runtime (Rust) - both MCP servers use `#[tokio::main]`
- Python 3.x (stdlib only for hooks — no pip dependencies)

**Package Manager:**
- npm (Node.js) - `gsd/package-lock.json` present; lockfile committed
- cargo (Rust) - `mcp-servers/sequential-thinking/Cargo.lock` and `mcp-servers/github/Cargo.lock` present; both lockfiles committed

## Frameworks

**Core (Rust MCP servers):**
- `rmcp` 0.16 - MCP (Model Context Protocol) server framework; used by both servers with `features = ["server", "macros", "transport-io"]`
- `tokio` 1.x (full) - Async runtime for both servers
- `schemars` 1.x - JSON Schema generation for MCP tool parameter types
- `serde` 1.x + `serde_json` 1.x - Serialization for all JSON-RPC traffic

**Testing (Rust):**
- Built-in `#[test]` / `#[tokio::test]` - inline unit tests in source files
- `tempfile` 3.x - Temporary file fixtures (`sequential-thinking` dev dep)
- `insta` 1.x - Snapshot testing (`github` dev dep)
- `wiremock` 0.6 - HTTP mock server for GitHub API tests (`github` dev dep)
- `proptest` 1.x - Property-based testing (`github` dev dep)

**Testing (Node.js):**
- Custom test runner at `gsd/scripts/run-tests.cjs` — no Jest/Vitest
- `c8` 11.x - Code coverage (dev dep; threshold: 70% lines)

**Build/Dev (Node.js):**
- `esbuild` 0.24 - Used in `gsd/scripts/build-hooks.js` to bundle hook scripts into `gsd/hooks/dist/`

## Key Dependencies

**Critical (Rust — sequential-thinking):**
- `rmcp` 0.16 - Entire MCP server surface; `mcp-servers/sequential-thinking/src/server.rs`
- `uuid` 1.x (v4 feature) - Session ID generation in `mcp-servers/sequential-thinking/src/logging.rs`
- `regex` 1.x - Model pattern matching for tuning profiles in `mcp-servers/sequential-thinking/src/profiles.rs`
- `chrono` 0.4 - Timestamp generation in persistent JSONL logger
- `tracing` 0.1 + `tracing-subscriber` 0.3 - Structured logging to stderr (MCP uses stdout for JSON-RPC)

**Critical (Rust — github):**
- `rmcp` 0.16 - MCP server surface; `mcp-servers/github/src/server.rs`
- `reqwest` 0.12 (rustls-tls-webpki-roots + json) - GitHub REST API HTTP client; `mcp-servers/github/src/github/client.rs`
- `rustls` 0.23 (aws_lc_rs feature) - TLS; forced over `ring` to avoid C compiler requirement on WSL2
- `base64` 0.22 - Decoding file content from GitHub API responses; `mcp-servers/github/src/compress.rs`
- `thiserror` 2.x - Typed error enums in auth and client modules

**Critical (Node.js GSD):**
- Zero runtime npm dependencies — all GSD logic uses Node.js stdlib only (`fs`, `path`, `child_process`, `os`)
- `c8` 11.x + `esbuild` 0.24 are dev-only

## Configuration

**Environment (MCP servers):**
- `KP_SEQTHINK_LOG_LEVEL` - Log level for sequential-thinking server (default: `info`)
- `KP_GITHUB_LOG_LEVEL` - Log level for github server (default: `warn`)
- `SEQUENTIAL_THINKING_MODEL` - Model ID passed to sequential-thinking for profile selection
- `SEQUENTIAL_THINKING_PROFILES` - Optional path override for tuning profiles JSON
- `GITHUB_TOKEN` / `GH_TOKEN` - GitHub auth (cascade: GITHUB_TOKEN → GH_TOKEN → `gh auth token`)
- `KP_GITHUB_MAX_BODY` - Max response body chars (default: 500)
- `KP_GITHUB_TIME_CUTOFF` - Time cutoff for filtering old data in days (default: 30)
- `KP_GITHUB_STRIP_URLS` - Whether to strip URLs from responses (default: true)
- `KP_GITHUB_FORMAT` - Output format override: json/table/text/auto
- `DISABLE_THOUGHT_LOGGING` - Disables JSONL thought persistence (used in tests)
- Client detection: `CLAUDE_CODE_VERSION`, `CLAUDE_AGENT_SDK`, `GEMINI_CLI`, `GOOGLE_CLI`, `TALENTOS_AGENT`

**Environment (GSD / hooks):**
- `CLAUDE_SESSION_ID`, `CLAUDE_BEAD_ID`, `CLAUDE_PROJECT_DIR` - Injected by Claude Code; used by `hooks/agent-outcome-logger.py`
- `CLAUDE_PLUGIN_ROOT` - Plugin root path for hook command in `hooks/hooks.json`

**GSD Project Config:**
- Per-project config at `.planning/config.json` — valid keys: `model_profile`, `commit_docs`, `search_gitignored`, `branching_strategy`, `brave_search`, `workflow.*`, `git.*`, `planning.*`
- Default model profile: `balanced`
- Profiles: `quality`, `balanced`, `budget`

**Build:**
- `gsd/scripts/build-hooks.js` — esbuild bundles hooks to `gsd/hooks/dist/`; runs as `prepublishOnly`
- Rust: `[profile.release]` — `opt-level = 2`, `strip = true` for both servers

## Platform Requirements

**Development:**
- Rust toolchain (tested with rustc 1.94.0)
- Node.js >=20 with npm
- Python 3.x (stdlib only)
- `gh` CLI recommended for GitHub auth fallback
- WSL2 note: `aws-lc-rs` forced over `ring` in github server due to WSL2 C compiler incompatibility

**Production:**
- Pre-compiled binaries in `mcp-servers/bin/linux-x86_64/` and `mcp-servers/bin/macos-arm64/`
- Installed into `~/.claude/plugins/kinderpowers/` via `setup.sh`
- GSD runtime symlinked to `~/.claude/get-shit-done/`
- Agent outcome log at `~/.kinderpowers/agent_outcomes.jsonl`
- Thought session logs at `var/sequential_thinking_logs/{session_id}.jsonl`

---

*Stack analysis: 2026-03-19*
