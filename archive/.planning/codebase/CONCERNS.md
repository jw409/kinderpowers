# Codebase Concerns

**Analysis Date:** 2026-03-19

---

## Tech Debt

### GSD Vendored at v1.26.0 — Upstream is 101 Commits Ahead

- Issue: GSD is vendored in `gsd/` at v1.26.0 (pinned in `plugin.json` and `gsd/VERSION`). The upstream `davidjbauer/get-shit-done` has continued development. A dual-copy structure exists: `gsd/bin/` (KP-modified) and `gsd/get-shit-done/bin/` (upstream baseline) — all 12 library files differ between them.
- Files: `gsd/VERSION`, `gsd/bin/gsd-tools.cjs`, `gsd/get-shit-done/bin/gsd-tools.cjs`, `.claude-plugin/plugin.json`
- Impact: Missing 101 upstream commits. Known upstream-only features visible in diff: `commit --no-verify` flag, `commit-to-subrepo` multi-repo routing, `audit-uat`, `fast`, `plant-seed`, `pr-branch`, `review-backlog`, `review`, `thread` commands (8 commands present in `gsd/commands/gsd/` but not in `commands/gsd/`), and `uat.cjs` module. KP agents reference `${CLAUDE_PLUGIN_ROOT}/gsd/bin/gsd-tools.cjs` — if upstream fixes are needed they must be manually merged.
- Fix approach: Evaluate upstream changelog from v1.26.0 forward. Cherry-pick desired fixes to `gsd/bin/`. Update `commands/gsd/` to add the 8 missing command stubs. Remove `gsd/get-shit-done/` duplicate directory once diff is reconciled — it serves no runtime purpose and bloats the repo.

---

### Duplicate GSD Binary Tree: `gsd/bin/` vs `gsd/get-shit-done/bin/`

- Issue: Two full copies of gsd-tools.cjs and all lib/*.cjs files exist: `gsd/bin/lib/` (12 files) and `gsd/get-shit-done/bin/lib/` (13 files, adds `uat.cjs`). All 12 shared files differ. `gsd/get-shit-done/` is the upstream snapshot; `gsd/bin/` is the KP-modified fork.
- Files: `gsd/bin/gsd-tools.cjs`, `gsd/get-shit-done/bin/gsd-tools.cjs` and all corresponding `lib/` files
- Impact: Confusion about which binary is authoritative. Agents use `${CLAUDE_PLUGIN_ROOT}/gsd/bin/gsd-tools.cjs`. The `gsd/get-shit-done/` tree is used by some upstream workflow references (`@~/.claude/get-shit-done/...`) which differ from KP's `${CLAUDE_PLUGIN_ROOT}/gsd/...` path pattern.
- Fix approach: Delete `gsd/get-shit-done/` after verifying all KP agent/workflow references use the `${CLAUDE_PLUGIN_ROOT}/gsd/` path. Run a scan for `~/.claude/get-shit-done` references in KP files to ensure none remain.

---

### Command Divergence: KP `commands/gsd/` Missing 8 Upstream Commands

- Issue: The `gsd/commands/gsd/` directory contains 8 commands that are not present in the root `commands/gsd/` directory: `add-backlog.md`, `audit-uat.md`, `fast.md`, `plant-seed.md`, `pr-branch.md`, `review-backlog.md`, `review.md`, `thread.md`.
- Files: `commands/gsd/` (42 files) vs `gsd/commands/gsd/` (50 files)
- Impact: Users invoking `/gsd:review` or `/gsd:fast` at the KP level get no command. The upstream workflow for `audit-uat` is referenced in `gsd/get-shit-done/workflows/audit-uat.md` but has no KP command entry point.
- Fix approach: Copy or symlink the 8 missing command files from `gsd/commands/gsd/` to `commands/gsd/`. Or establish a single authoritative `commands/gsd/` and remove the duplicate at `gsd/commands/gsd/`.

---

### 16 GSD Agent Types — Consolidation Opportunity

- Issue: KP ships 22 agents total (16 gsd-prefixed + 6 KP-native). The 16 gsd agents are documented in `gsd/docs/AGENTS.md` as 10 distinct roles but implemented as 16 files, with notable clustering in the UI domain (gsd-ui-researcher, gsd-ui-checker, gsd-ui-auditor) and research domain (gsd-project-researcher, gsd-phase-researcher, gsd-research-synthesizer). The three UI agents do strictly separate things (spec creation, contract validation, retroactive audit) but differ only in their trigger context and output file.
- Files: `agents/gsd-ui-researcher.md`, `agents/gsd-ui-checker.md`, `agents/gsd-ui-auditor.md`, `agents/gsd-project-researcher.md`, `agents/gsd-phase-researcher.md`, `agents/gsd-research-synthesizer.md`
- Impact: Agent selection surface is broad. `gsd/docs/AGENTS.md` says 15 agents but the directory contains 16 gsd-prefixed agents (gsd-user-profiler was added in v1.26.0 and is not fully reflected in the doc). The catalog becomes harder to maintain as each agent diverges independently.
- Fix approach: Consolidate the three UI agents into a single parameterized `gsd-ui-specialist` with a `mode` parameter (spec, check, audit). Apply the same pattern to research agents — a single `gsd-researcher` with `scope=project|phase|synthesis`. This aligns with the "caller controls" philosophy already present in multi-perspective-review.md and quality-gate.md.

---

### 17/22 Agents Lack Parameterization

- Issue: Only 5 of 22 agents have caller-controllable parameters: `gsd-codebase-mapper` (`<parameters>` block), `gsd-executor` (`<parameters>` block), `multi-perspective-review` (`## Parameters (caller controls)`), `quality-gate` (`## Parameters (caller controls)`), `strategic-planner` (`## Parameters (caller controls)`). The remaining 17 agents have fixed behavior with no tuning surface.
- Files: `agents/gsd-debugger.md`, `agents/gsd-integration-checker.md`, `agents/gsd-nyquist-auditor.md`, `agents/gsd-phase-researcher.md`, `agents/gsd-plan-checker.md`, `agents/gsd-planner.md`, `agents/gsd-project-researcher.md`, `agents/gsd-research-synthesizer.md`, `agents/gsd-roadmapper.md`, `agents/gsd-ui-auditor.md`, `agents/gsd-ui-checker.md`, `agents/gsd-ui-researcher.md`, `agents/gsd-user-profiler.md`, `agents/gsd-verifier.md`, `agents/code-reviewer.md`, `agents/research-extractor.md`, `agents/team-coordinator.md`
- Impact: Callers cannot tune depth, strictness, or output format. The CHANGELOG.md v6.1.0 specifically added parameterization as a design goal ("one-size-fits-all is the anti-pattern") but only 5 agents were updated.
- Fix approach: Add `## Parameters (caller controls)` sections to the remaining 17 agents. Priority order: `gsd-verifier` (add strictness/scope), `gsd-planner` (add plan depth/tdd-mode), `gsd-debugger` (add investigation depth/hypothesis count), `code-reviewer` (add pedanticness/scope).

---

### Beads Integration Absent from GSD Workflows

- Issue: The `skills/beads/SKILL.md` documents a complete bead protocol including session close sequence (`bd sync` → `git commit` → `bd sync` → `git push`). Zero GSD commands or workflows reference beads. `commands/gsd/execute-phase.md`, `commands/gsd/plan-phase.md`, `commands/gsd/verify-work.md` — none instruct the executor or planner to create or update beads for phase work.
- Files: `skills/beads/SKILL.md`, `commands/gsd/execute-phase.md`, `commands/gsd/plan-phase.md`, `commands/gsd/verify-work.md`, `commands/gsd/pause-work.md`
- Impact: GSD uses `.planning/STATE.md` and `WAITING.json` for session continuity. This is orthogonal to beads but does not replace it. Multi-session GSD work has no bead tracking, which means context recovery after compaction falls back to re-reading all `.planning/` files rather than running `bd ready`.
- Fix approach: Add a "Bead Integration" section to `commands/gsd/pause-work.md` and `commands/gsd/resume-work.md` that runs `bd ready` and `bd create` for active phases. Add bead claim/close calls to `execute-phase.md` workflow completion step.

---

### Scanner.py Has False Positive Problem with Legitimate Protocol Language

- Issue: `scanner.py` flags `MUST` and `NEVER` as medium severity without distinguishing between compulsion language in skills (bad) and protocol specification in workflow files (legitimate). Running the scanner against the full repo produces 540 findings with 535 medium — the vast majority are GSD workflow specification language like "Every task MUST include these fields" and "NEVER include phases for:". The scanner cannot distinguish intent.
- Files: `scanner.py`, `gsd/workflows/plan-phase.md:437`, `gsd/workflows/execute-plan.md:140`, `gsd/get-shit-done/workflows/plan-phase.md:437`
- Impact: 4 HIGH findings exist in vendored GSD workflow files (`gsd/workflows/plan-phase.md`, `gsd/workflows/execute-plan.md` and their `get-shit-done/` duplicates). The CI `--check` mode exits 1 on these if run against `gsd/`. The scanner is intended for `skills/` not for `gsd/` workflow specifications.
- Fix approach: Add a `--exclude-dirs` argument to `scanner.py` or a `.scannerignore` file. Exclude `gsd/` from CI scans. The HIGH findings in `gsd/workflows/` are protocol language, not compulsion language in the philosophical sense the scanner was designed to detect.

---

### Duplicate Workflow Files: `gsd/workflows/` vs `gsd/get-shit-done/workflows/`

- Issue: KP ships two full copies of all GSD workflow definitions. `gsd/workflows/` has 44 files; `gsd/get-shit-done/workflows/` has 47 files (adds `diagnose-issues.md`, `discovery-phase.md`, `resume-project.md`, `transition.md`, `verify-phase.md`). These diverge in the same way as the bin/ directories.
- Files: `gsd/workflows/` (44 .md files), `gsd/get-shit-done/workflows/` (47 .md files)
- Impact: Agents referencing `@${CLAUDE_PLUGIN_ROOT}/gsd/workflows/execute-phase.md` get different content than `@~/.claude/get-shit-done/workflows/execute-phase.md`. The KP-modified versions omit 5 upstream workflow files. Runtime paths differ.
- Fix approach: Same as bin/ consolidation — delete `gsd/get-shit-done/workflows/` and add the 5 missing workflows to `gsd/workflows/`.

---

### Self-Optimizing Pipeline: Design Complete but Not Implemented

- Issue: `mcp-servers/sequential-thinking/SELF_OPTIMIZING_PIPELINE.md` documents a 4-phase feedback loop (scavenger → PRIM → CMA-ES → profile update). Phase 1 (plumbing) and Phase 4 (CMA-ES optimization) are not implemented. Profiles are hard-coded in `src/profiles.rs` via `default_profiles()`, not loaded from an external JSON file.
- Files: `mcp-servers/sequential-thinking/SELF_OPTIMIZING_PIPELINE.md`, `mcp-servers/sequential-thinking/src/profiles.rs`
- Impact: Per-model profile parameters (`branching_threshold`, `confidence_threshold`, `default_explore_count`) are static. Session logs accumulate in `var/sequential_thinking_logs/` but are never fed back. The `SEQUENTIAL_THINKING_PROFILES_PATH` env var referenced in the design doc has no implementation. The CMA-ES optimizer referenced from `game1-cght` (closed epic) never connects to the Rust server.
- Fix approach: Implement Upgrade 1 from the design doc: add a `load_profiles()` function in `src/profiles.rs` that reads from `$SEQUENTIAL_THINKING_PROFILES_PATH` if set, falling back to `default_profiles()`. This enables external tuning without code changes.

---

## Known Bugs

### HIGH: GSD Workflow Files Fail Scanner CI Check

- Symptoms: Running `scanner.py --check --dir gsd/` exits with code 1 due to 4 HIGH-severity findings in `gsd/workflows/plan-phase.md` and `gsd/workflows/execute-plan.md` (plus their `get-shit-done/` duplicates).
- Files: `gsd/workflows/plan-phase.md:437`, `gsd/workflows/execute-plan.md:140`, `gsd/get-shit-done/workflows/plan-phase.md:437`, `gsd/get-shit-done/workflows/execute-plan.md:140`
- Trigger: Run `uv run python scanner.py --check --dir gsd/` — exits 1.
- Workaround: Exclude `gsd/` from scanner CI or run only against `skills/` and `agents/`.

---

### Agent Count Discrepancy in gsd/docs/AGENTS.md

- Symptoms: `gsd/docs/AGENTS.md` states "All 15 specialized agents" but the `gsd/agents/` directory contains 16 agents (gsd-user-profiler was added in v1.26.0 and is documented later in the file but not counted in the header or the category table).
- Files: `gsd/docs/AGENTS.md:3`, `gsd/agents/gsd-user-profiler.md`
- Trigger: `ls gsd/agents/*.md | wc -l` returns 16; docs says 15.
- Workaround: Reference the directory listing rather than the doc count.

---

## Security Considerations

### research-extractor Uses WebFetch Tool

- Risk: `agents/research-extractor.md` declares `tools: Read, Grep, Glob, Bash, WebSearch, WebFetch`. WebFetch allows arbitrary URL fetching. The agent has `model: opus` and broad tool access. No URL allowlist or content validation is specified.
- Files: `agents/research-extractor.md:6`
- Current mitigation: None — agent relies on model judgment for URL safety.
- Recommendations: Add a `## Scope` section defining which URL patterns are acceptable for research tasks. Consider removing WebFetch in favor of WebSearch-only, using `gsd-phase-researcher` pattern (which also uses WebFetch but is narrower in scope).

---

### MCP Binary Path Relies on `${CLAUDE_PLUGIN_ROOT}` Env Var

- Risk: `plugin.json` registers MCP server binaries as `${CLAUDE_PLUGIN_ROOT}/mcp-servers/bin/kp-github-mcp` and `${CLAUDE_PLUGIN_ROOT}/mcp-servers/bin/kp-sequential-thinking`. If `CLAUDE_PLUGIN_ROOT` is not set or is wrong, MCP servers silently fail to start.
- Files: `.claude-plugin/plugin.json:41-48`, `mcp-servers/install.sh`
- Current mitigation: `mcp-servers/install.sh` builds and registers the binaries. No startup validation exists.
- Recommendations: Add a health check in `gsd/hooks/gsd-workflow-guard.js` that verifies MCP server registration at session start. The `gsd/hooks/gsd-check-update.js` hook is the right place for this.

---

### set_var in Tests Causes Env Pollution Between Tests

- Risk: `mcp-servers/sequential-thinking/src/server.rs` tests use `std::env::set_var("DISABLE_THOUGHT_LOGGING", "true")` and `std::env::set_var("TALENTOS_AGENT", "1")` without cleanup in parallel test runs. This is noted in a comment ("env var tests must be serialized to avoid races") but the test for `detect_client_type_all_branches` does clean up.
- Files: `mcp-servers/sequential-thinking/src/server.rs:399`, `mcp-servers/sequential-thinking/src/server.rs:586-588`, `mcp-servers/sequential-thinking/src/server.rs:641-675`
- Current mitigation: The `detect_client_type_all_branches` test uses a single consolidated test function to avoid parallel races. The `DISABLE_THOUGHT_LOGGING` set at line 399 is never removed.
- Recommendations: Wrap env mutations in `unsafe { std::env::set_var }` calls using `serial_test` crate or move logging disable to a test fixture that restores state.

---

## Performance Bottlenecks

### PersistentLogger Writes to Disk on Every Thought

- Problem: `mcp-servers/sequential-thinking/src/logging.rs` opens and appends to a JSONL file on every `persist()` call (fire-and-forget). No batching or buffering — each thought triggers a filesystem write.
- Files: `mcp-servers/sequential-thinking/src/logging.rs`
- Cause: The `persist()` method uses `OpenOptions::append(true)` and immediately writes. No write buffer or async channel is used.
- Improvement path: Buffer writes in a `Vec<String>` and flush on session end or at intervals. Alternatively, use a channel-based background writer that batches. The current approach is acceptable for thinking sessions with < 50 thoughts; it becomes a bottleneck for long sessions with `explore_count=5` generating 200+ thoughts.

---

## Fragile Areas

### `gsd/bin/gsd-tools.cjs` Module Resolution

- Files: `gsd/bin/gsd-tools.cjs`, `gsd/bin/lib/*.cjs`
- Why fragile: The CHANGELOG for v6.1.0 notes "fixed MODULE_NOT_FOUND when invoked from different cwd (__dirname-relative requires)". This fix exists in `gsd/bin/` but not verified in `gsd/get-shit-done/bin/`. Agents invoke `node "${CLAUDE_PLUGIN_ROOT}/gsd/bin/gsd-tools.cjs"` from arbitrary working directories. If the `__dirname` fix is incomplete, commands fail silently.
- Safe modification: Always test `gsd-tools.cjs` commands from a different working directory than the bin/ directory after any changes.
- Test coverage: `gsd/scripts/run-tests.cjs` exists but test scope is not verified to include cross-cwd invocation.

---

### Scanner Regex for `MUST`/`NEVER` Has Lookahead Scope Issue

- Files: `scanner.py:86-96`
- Why fragile: The `MUST` pattern is `\bMUST\b(?!.*\b(unless|except|if|when|consider)\b)`. The negative lookahead `.*` matches only to end of the current line. A `MUST` on line N with an escape clause on line N+1 is incorrectly flagged as compulsion language. The scanner operates line-by-line.
- Safe modification: The `scan_file` function splits on `\n` and checks each line independently. Any multi-line escape clauses will not be detected.
- Test coverage: `test_scanner.py` tests single-line escape cases only. No multi-line escape test exists.

---

### Agent Files Differ Between `agents/` and `gsd/agents/`

- Files: `agents/gsd-codebase-mapper.md`, `agents/gsd-executor.md`, `agents/gsd-debugger.md`, `agents/gsd-phase-researcher.md`, `agents/gsd-plan-checker.md`, `agents/gsd-planner.md`, `agents/gsd-project-researcher.md`, `agents/gsd-research-synthesizer.md`, `agents/gsd-roadmapper.md`, `agents/gsd-ui-researcher.md`, `agents/gsd-verifier.md`
- Why fragile: KP ships parameterized agents in `agents/` (the versions users invoke via Claude plugin). The `gsd/agents/` directory contains the upstream GSD versions. 11 of 16 shared agents differ. KP's `agents/gsd-codebase-mapper.md` adds 5 parameters, LSP tool, JSONL output, and ~400 lines of exploration guidance that upstream lacks. If someone updates `gsd/agents/` expecting changes to take effect at runtime, nothing changes — runtime uses `agents/`.
- Safe modification: Only modify `agents/*.md` for runtime behavior changes. Treat `gsd/agents/` as upstream reference only. Never sync from `gsd/agents/` back to `agents/` without reviewing the diff.
- Test coverage: No automated test verifies that `agents/` versions are used at runtime over `gsd/agents/` versions.

---

## Scaling Limits

### Sequential Thinking Engine is Single-Threaded (Mutex)

- Current capacity: One concurrent thinking session per process instance.
- Limit: `SeqThinkServer.engine` is wrapped in `std::sync::Mutex<ThinkingEngine>`. All tool calls serialize on the lock. A single long-running tool call blocks all subsequent calls.
- Scaling path: Thought history is session-scoped. Multiple independent sessions should use separate `ThinkingEngine` instances. Replace `Mutex<ThinkingEngine>` with `DashMap<SessionId, ThinkingEngine>` if multi-session concurrency is needed. Current single-session MCP usage is not blocked by this.

---

## Dependencies at Risk

### `rmcp` at v0.16 — Rapidly Evolving MCP SDK

- Risk: Both MCP servers pin `rmcp = { version = "0.16", ... }`. The MCP protocol and `rmcp` SDK are actively evolving. The `#[rmcp::tool_handler]` and `#[rmcp::tool_router]` macros generate code that may break on minor version bumps.
- Files: `mcp-servers/sequential-thinking/Cargo.toml:13`, `mcp-servers/github/Cargo.toml:7`
- Impact: A breaking change in `rmcp` requires rebuilding both servers. Users who upgrade Rust dependencies via `cargo update` may get build failures.
- Migration plan: Lock to an exact version (`rmcp = "=0.16.X"`) in Cargo.lock or periodically test against the latest minor version. Add a CI step that runs `cargo test` on both MCP servers.

---

### No `reload_profiles` MCP Tool — Profile Updates Require Restart

- Risk: Profile changes (e.g., after CMA-ES optimization or manual tuning) require restarting the MCP server. The `SELF_OPTIMIZING_PIPELINE.md` doc explicitly calls for a `reload_profiles` tool in Phase 1 but it was not implemented.
- Files: `mcp-servers/sequential-thinking/src/server.rs`, `mcp-servers/sequential-thinking/SELF_OPTIMIZING_PIPELINE.md:48`
- Impact: External learning pipeline cannot hot-update profiles. Any profile optimization requires a server restart, losing in-session state.
- Migration plan: Implement the `reload_profiles` tool as described in the design doc — reads from `$SEQUENTIAL_THINKING_PROFILES_PATH` and calls `engine.update_profile()`. This is a small change (~30 lines).

---

## Missing Critical Features

### No Beads Integration in GSD Lifecycle

- Problem: GSD manages phase work state in `.planning/STATE.md`, `.planning/HANDOFF.json`, and `WAITING.json`. The `skills/beads/SKILL.md` skill documents `bd ready` as the canonical context recovery mechanism after compaction. These systems are completely independent. An agent running GSD commands after context compaction has no bead-based recovery path.
- Blocks: Multi-session GSD project work where phases span many conversations. Context recovery after compaction falls back to reading all `.planning/` files from scratch.

---

### `etc/sequential_thinking_profiles.json` Does Not Exist

- Problem: The `SELF_OPTIMIZING_PIPELINE.md` design specifies profiles should be loadable from `etc/sequential_thinking_profiles.json`. This file does not exist. All profiles are compiled in to `src/profiles.rs`. External profile customization (e.g., tuning the Nemotron profile) requires editing Rust source and rebuilding.
- Blocks: External profile injection, the CMA-ES optimization loop, and user-configurable model tuning without code changes.

---

### GSD `gsd/docs/AGENTS.md` Is Stale

- Problem: `gsd/docs/AGENTS.md` documents 15 agents but 16 exist. `gsd-user-profiler` was added in v1.26.0 and is listed later in the file (line 332) but the header, category table, and count are wrong. Additionally, the 6 KP-native agents (`multi-perspective-review`, `quality-gate`, `team-coordinator`, `research-extractor`, `strategic-planner`, `code-reviewer`) are not documented anywhere in `gsd/docs/`.
- Blocks: Accurate discovery of available agents via `KINDERPOWERS.xml` or documentation.

---

## Test Coverage Gaps

### Scanner Multi-Line Escape Clauses Not Tested

- What's not tested: `scanner.py`'s MUST/NEVER patterns operate line-by-line. No test verifies that an escape clause on the following line prevents a finding.
- Files: `test_scanner.py`, `scanner.py:86-96`
- Risk: False positives on legitimate multi-line guidance; false negatives if compulsion language spans lines.
- Priority: Low

---

### KP-Native Agents Have No Automated Tests

- What's not tested: `agents/multi-perspective-review.md`, `agents/quality-gate.md`, `agents/strategic-planner.md`, `agents/team-coordinator.md`, `agents/research-extractor.md`, `agents/code-reviewer.md` — all 6 KP-native agents have no test coverage in `tests/` (integration tests exist only for skill-triggering patterns, not agent behavior).
- Files: `tests/claude-code/`, `tests/skill-triggering/`, `agents/*.md`
- Risk: Agent parameter parsing, mode routing, and output contracts are untested. A broken parameter in `multi-perspective-review` would only be caught in production use.
- Priority: Medium

---

### GSD Tool CJS Tests Do Not Cover Cross-CWD Invocation

- What's not tested: `gsd/scripts/run-tests.cjs` exists but the MODULE_NOT_FOUND fix (v6.1.0 changelog) is not explicitly tested by invoking `gsd-tools.cjs` from a non-bin working directory.
- Files: `gsd/scripts/run-tests.cjs`, `gsd/bin/gsd-tools.cjs`
- Risk: Regression of the MODULE_NOT_FOUND bug silently breaks all agent tool calls.
- Priority: High

---

## Type Errors

**Type checker:** Not available (Rust project — `cargo check` could be run but was not executed in this analysis. Python files have no type annotations beyond dataclasses.)

**Scanner.py typing:** `scanner.py` uses `@dataclass` for `Finding` but no `mypy` or `pyright` configuration exists. The `Iterator[Finding]` return type annotations are informal.

---

## Dead Code (LSP-verified)

LSP not available for Rust source in this context. Statically identified:

| File | Symbol | Type | References | Verdict |
|------|--------|------|-----------|---------|
| `mcp-servers/sequential-thinking/src/thinking.rs:106` | `model_id` field | struct field | 0 (marked `#[allow(dead_code)]`) | Intentionally dead — future analytics |
| `mcp-servers/sequential-thinking/src/thinking.rs:108` | `client_type` field | struct field | 0 (marked `#[allow(dead_code)]`) | Intentionally dead — future analytics |
| `mcp-servers/sequential-thinking/src/thinking.rs:141` | `profile()` method | pub fn | 0 (marked `#[allow(dead_code)]`) | Intentionally dead — future public API |
| `gsd/get-shit-done/bin/lib/uat.cjs` | entire module | cjs | Not referenced in KP commands | Upstream-only feature, no KP entry point |

---

## Phantom Config

| Config key | Defined in | Referenced by | Verdict |
|-----------|-----------|--------------|---------|
| `SEQUENTIAL_THINKING_MODEL` | `mcp-servers/sequential-thinking/src/server.rs:328` env read | No external config file | Read at runtime only — no documentation of valid values |
| `SEQUENTIAL_THINKING_PROFILES_PATH` | `SELF_OPTIMIZING_PIPELINE.md` design doc | 0 code paths | Phantom — design intent only, never implemented |
| `RUST_MIN_STACK` | `mcp-servers/install.sh:6`, `mcp-servers/upgrade.sh:8` | Not read by Rust code — set for cargo build env | Cargo workaround only |

---

## Layer Violations

LSP not available for cross-file analysis. Pattern-based finding:

| From | Imports | Stated rule | Observation |
|------|---------|-------------|-----------|
| `agents/*.md` (KP layer) | References `${CLAUDE_PLUGIN_ROOT}/gsd/bin/gsd-tools.cjs` | Agents should be self-contained | Tight coupling to vendored GSD toolchain path — breaks if GSD path changes |
| `gsd/agents/*.md` (GSD upstream layer) | References `$HOME/.claude/get-shit-done/bin/gsd-tools.cjs` | Upstream agents use install path | Path convention differs from KP agents — both layers coexist without clear precedence |

---

*Concerns audit: 2026-03-19*
*Intelligence sources: grep, AST (Python), static analysis (Rust source reading), scanner.py execution*
*Confidence: medium — LSP/cargo-check not run; findings from source reading and tool execution*
