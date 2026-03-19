# External Integrations

**Analysis Date:** 2026-03-19

## APIs & External Services

**GitHub REST API:**
- Used by: `mcp-servers/github/` — token-compressed GitHub MCP server
- Endpoint: `https://api.github.com` (hardcoded in `mcp-servers/github/src/github/client.rs`)
- SDK/Client: `reqwest` 0.12 (direct HTTP) with `gh` CLI as fallback
- Auth: Bearer token via cascade — `GITHUB_TOKEN` env → `GH_TOKEN` env → `gh auth token` subprocess
- Token resolution: `mcp-servers/github/src/auth.rs`
- Tools exposed: issues, PRs, repos, commits, branches, releases, code search, users, files, tags, teams, labels, actions — see `mcp-servers/github/src/tools/`
- Token compression: responses filtered/compressed before returning to reduce Claude context usage; configured via `KP_GITHUB_MAX_BODY`, `KP_GITHUB_TIME_CUTOFF`, `KP_GITHUB_STRIP_URLS` env vars

**AI Model Runtimes (Claude Code / Gemini CLI / Codex):**
- GSD commands (`gsd/commands/gsd/*.md`) spawn sub-agents using the host AI runtime
- Sequential thinking server detects host client type at startup via env vars: `CLAUDE_CODE_VERSION`, `CLAUDE_AGENT_SDK` → `claude-code`; `GEMINI_CLI`, `GOOGLE_CLI` → `gemini-cli`; `TALENTOS_AGENT` → `talentos`
- Detection logic: `mcp-servers/sequential-thinking/src/server.rs` `detect_client_type()`
- Model profiles for sequential thinking tuned per model family: Gemini Flash, DeepSeek, Grok, Claude, Llama/Nemotron — `mcp-servers/sequential-thinking/src/profiles.rs`

**Brave Search (optional):**
- Referenced in GSD config as `brave_search: false` (default off)
- Config key: `brave_search` in `.planning/config.json`
- Not directly implemented in this repo — expected to be provided by the host AI runtime as a tool

## Data Storage

**Databases:**
- None — no database dependencies in either Rust server or Node.js code

**File Storage (local):**
- Sequential thinking JSONL logs: `var/sequential_thinking_logs/{session_id}.jsonl` (relative to project root, written by `mcp-servers/sequential-thinking/src/logging.rs`)
- Agent outcome log: `~/.kinderpowers/agent_outcomes.jsonl` (written by `hooks/agent-outcome-logger.py`)
- GSD planning state: `.planning/` directory tree — `STATE.md`, `ROADMAP.md`, `config.json`, `phases/*/`, `codebase/` — managed by `gsd/bin/lib/*.cjs`
- Hook output log: `~/.kinderpowers/` directory created by `setup.sh`

**Caching:**
- None

## Authentication & Identity

**GitHub Auth:**
- Provider: GitHub token (PAT or `gh` CLI managed token)
- Cascade: `GITHUB_TOKEN` env var → `GH_TOKEN` env var → `gh auth token` subprocess call
- Implementation: `mcp-servers/github/src/auth.rs`
- No OAuth flow — token is resolved once at process startup and held in memory for the server lifetime

**AI Runtime Auth:**
- Managed entirely by the host AI client (Claude Code, Gemini CLI, Codex) — kinderpowers does not handle AI API keys directly

## Monitoring & Observability

**Structured Logging (Rust):**
- Both MCP servers use `tracing` crate with `tracing-subscriber`
- All trace output goes to stderr (MCP protocol uses stdout for JSON-RPC)
- Log level controlled by `KP_SEQTHINK_LOG_LEVEL` (sequential-thinking) and `KP_GITHUB_LOG_LEVEL` (github)
- Format: human-readable with env filter

**Agent Outcome Telemetry:**
- `hooks/agent-outcome-logger.py` — PostToolUse hook that fires after every `Agent` tool call
- Records: agent name, model, description, session ID, bead ID, project path, output size, output preview, variant tag
- Stored: `~/.kinderpowers/agent_outcomes.jsonl` (JSONL append)
- Variant A/B tagging: `variant:` marker in agent prompt enables comparison tracking
- Registered in Claude Code `settings.json` under `hooks.PostToolUse`

**Sequential Thinking Session Logs:**
- Per-session JSONL files at `var/sequential_thinking_logs/{session_id}.jsonl`
- Each record includes: timestamp, sessionId, projectPath, clientType, modelId, profile, all thought fields
- Session ID sourced from `CLAUDE_SESSION_ID` env var or UUID v4 fallback
- Disabled via `DISABLE_THOUGHT_LOGGING=true` env var (used in tests)

## CI/CD & Deployment

**Hosting:**
- Not a deployed service — installed as a local Claude Code plugin
- Install path: `~/.claude/plugins/kinderpowers/` (via `git clone`)
- GSD runtime symlinked: `~/.claude/get-shit-done/ -> {plugin_root}/gsd/`

**CI Pipeline:**
- Not detected in this repo — no `.github/workflows/` or CI config files present

**Distribution:**
- Pre-compiled Rust binaries distributed in `mcp-servers/bin/linux-x86_64/` and `mcp-servers/bin/macos-arm64/`
- Node.js GSD published to npm as `get-shit-done-cc` (version 1.26.0, package: `gsd/package.json`)
- npm install command in `gsd/bin/install.js`

## Environment Configuration

**Required env vars (GitHub MCP server):**
- At least one of: `GITHUB_TOKEN`, `GH_TOKEN`, or `gh` CLI installed and authenticated

**Optional env vars (sequential-thinking server):**
- `SEQUENTIAL_THINKING_MODEL` - Used for profile selection; falls back to `"unknown"`
- `SEQUENTIAL_THINKING_PROFILES` - Custom profiles JSON file path
- `KP_SEQTHINK_LOG_LEVEL` - Log verbosity

**Optional env vars (GitHub MCP server):**
- `KP_GITHUB_MAX_BODY`, `KP_GITHUB_TIME_CUTOFF`, `KP_GITHUB_STRIP_URLS`, `KP_GITHUB_FORMAT`
- `KP_GITHUB_BASE_URL` - Can override GitHub API base URL (used in `mcp-servers/github/src/github/client.rs` for test mocking via `wiremock`)

**Secrets location:**
- GitHub tokens in shell environment (`~/.zshrc` or equivalent); never committed

## Webhooks & Callbacks

**Incoming:**
- None — both MCP servers communicate via stdio JSON-RPC only (no HTTP listener)

**Outgoing:**
- `mcp-servers/github/` makes outbound HTTPS calls to `https://api.github.com` via `reqwest`
- `hooks/agent-outcome-logger.py` writes to local filesystem only (no outbound network calls)

## MCP Protocol Transport

Both Rust servers use stdio transport exclusively:
- `rmcp::transport::io::stdio()` — reads JSON-RPC from stdin, writes responses to stdout
- Client connects by spawning the server binary as a subprocess
- Server registration config expected in Claude Code's MCP server configuration (not included in this repo — user-configured)

---

*Integration audit: 2026-03-19*
