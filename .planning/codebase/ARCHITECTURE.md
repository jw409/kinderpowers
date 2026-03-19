# Architecture

**Analysis Date:** 2026-03-19

## Pattern Overview

**Overall:** Layered plugin-first agent operating system

**Key Characteristics:**
- Skills are markdown documents injected into agent context via Claude Code hooks — not code, not tools
- GSD lifecycle engine is a separate sub-repository embedded at `gsd/` with its own Node.js runtime and CLI
- Commands are markdown prompt templates that orchestrate spawned subagents; the orchestrator stays lean while subagents do the heavy work
- MCP servers are compiled Rust binaries that expose JSON-RPC tools over stdio
- Enforcement is opt-in via hookify rules (external system), not built into the core

## Layers

**L1 — Skills (Prompt Context Layer):**
- Purpose: Inject behavioral patterns into agent context at session start or on-demand
- Location: `skills/*/SKILL.md`
- Contains: YAML frontmatter (`name`, `description`), markdown guidance
- Depends on: Claude Code Skill tool / SessionStart hook injection
- Used by: Any agent, auto-triggered by hook or explicit `Skill()` tool invocation
- Discovery mechanism: `lib/skills-core.js` — `findSkillsInDir()` walks `skills/` recursively looking for `SKILL.md` files; personal skills at `~/.claude/skills/` shadow kinderpowers skills

**L2 — Agents (Specialized Worker Layer):**
- Purpose: Focused subagents with scoped tool access and single-task mandates
- Location: `agents/*.md` (6 kinderpowers agents), `gsd/agents/*.md` (16 GSD lifecycle agents)
- Contains: YAML frontmatter with `name`, `description`, `model`, `tools`, optional `color`; system prompt body
- Depends on: Commands that spawn them via `Task(subagent_type="...")`
- Used by: Orchestrator commands; model selection resolved by `gsd/bin/lib/model-profiles.cjs`

**L3 — Commands (Orchestrator Layer):**
- Purpose: Entry points that parse user intent, bootstrap context, and spawn agent pipelines
- Location: `commands/gsd/*.md` (42 commands)
- Contains: YAML frontmatter (`name`, `argument-hint`, `allowed-tools`), `<objective>`, `<execution_context>`, `<context>`, `<process>` XML blocks
- Depends on: Workflow files via `@${CLAUDE_PLUGIN_ROOT}/gsd/workflows/` reference; `gsd-tools.cjs` CLI for state management
- Used by: Human via `/gsd:<command>` slash invocation

**L4 — Workflows (Step-by-Step Orchestration Specs):**
- Purpose: Detailed machine-readable execution recipes consumed by command orchestrators
- Location: `gsd/workflows/*.md` (43 workflow files), `gsd/get-shit-done/workflows/` (upstream copy)
- Contains: `<purpose>`, `<process>`, numbered `<step>` blocks with embedded bash, agent spawn patterns, routing logic
- Depends on: `gsd-tools.cjs` init commands that return JSON context; agent type names from the catalog
- Used by: Commands reference workflow files via `<execution_context>` blocks

**L5 — GSD Tools CLI (Runtime Engine Layer):**
- Purpose: Centralizes all filesystem operations, state parsing, phase management, model resolution
- Location: `gsd/bin/gsd-tools.cjs` (CLI router), `gsd/bin/lib/*.cjs` (14 modules)
- Contains: CommonJS modules for state, phase, roadmap, config, template, verify, milestone, init, frontmatter, model-profiles
- Depends on: Node.js >=20, `.planning/` directory structure in the target project
- Used by: Workflows and commands via `node "${CLAUDE_PLUGIN_ROOT}/gsd/bin/gsd-tools.cjs" <command>`
- Large payload escape hatch: outputs `@file:/tmp/gsd-*.json` when JSON > 50KB to avoid bash buffer limits

**L6 — MCP Servers (External Tool Layer):**
- Purpose: Compiled Rust servers providing tools beyond Claude's built-ins
- Location: `mcp-servers/sequential-thinking/src/` and `mcp-servers/github/src/`
- Contains: Rust crates with JSON-RPC over stdio; `rmcp` crate for protocol
- Depends on: GitHub token env var; optional `KP_SEQTHINK_LOG_LEVEL` / `KP_GITHUB_LOG_LEVEL`
- Used by: Agent tool calls when MCP server is registered in Claude settings

**L7 — Enforcement Layer (Hookify Rules):**
- Purpose: Pre/post-write behavioral nudges that detect compulsion language, missing brainstorm, unverified completion
- Location: `hookify-rules/*.local.md` (3 rules)
- Contains: YAML frontmatter (`name`, `enabled`, `event`, `action`, `conditions`)
- Depends on: External `hookify` plugin being installed; `setup.sh` links rules into hookify's rules dir
- Used by: hookify intercepts write/stop events and evaluates conditions against transcript

## Data Flow

**Session Bootstrap Flow:**
1. Claude Code session starts → `hooks.json` fires `SessionStart` matcher
2. `hooks/session-start` bash script runs → reads `skills/using-kinderpowers/SKILL.md`
3. Script JSON-escapes content → outputs `hookSpecificOutput.additionalContext` (dual-shape for Cursor/Claude compat)
4. `using-kinderpowers` content is injected into agent context as `<EXTREMELY_IMPORTANT>` block
5. Agent now knows how to use the Skill tool to discover remaining skills

**GSD Command Execution Flow:**
1. User invokes `/gsd:execute-phase 3`
2. Command markdown loads `<execution_context>` file references (workflow + ui-brand)
3. Orchestrator runs `node gsd-tools.cjs init execute-phase 3` → returns JSON with all phase context
4. JSON `@file:` path pattern handled: if output starts with `@file:`, read the tmp file
5. Orchestrator parses: phase dir, plan list, model assignments from `model_profiles.cjs`
6. Plans grouped into dependency waves
7. For each wave: `Task(subagent_type="gsd-executor", ...)` spawns fresh agent with `<files_to_read>` block
8. Subagent executes plan tasks, commits atomically, writes `SUMMARY.md`
9. After all waves: `Task(subagent_type="gsd-verifier", ...)` validates phase completeness
10. Orchestrator updates `STATE.md` via `gsd-tools.cjs state patch`

**New Project Setup Flow:**
1. `/gsd:new-project` → loads `workflows/new-project.md`
2. `gsd-tools.cjs init new-project` bootstraps config
3. Orchestrator questions user (via `AskUserQuestion`), then:
   - Spawns 4 parallel `gsd-project-researcher` agents for domain research
   - Spawns `gsd-research-synthesizer` to merge into `SUMMARY.md`
   - Spawns `gsd-roadmapper` to produce `ROADMAP.md`
4. Creates: `.planning/PROJECT.md`, `.planning/config.json`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`

**Skill Resolution Flow:**
1. Agent calls `Skill("brainstorming")`
2. `lib/skills-core.js` `resolveSkillPath()` checks personal skills at `~/.claude/skills/brainstorming/SKILL.md` first
3. Falls back to kinderpowers `skills/brainstorming/SKILL.md`
4. `stripFrontmatter()` removes YAML header, returns content body
5. `checkForUpdates()` runs `git fetch` with 3s timeout to detect upstream changes (non-blocking)

**State Management:**
- `.planning/STATE.md` — living project memory, markdown with bold-field and frontmatter parseable by `state.cjs`
- `.planning/config.json` — workflow configuration (model profile, git strategy, feature flags)
- `.planning/ROADMAP.md` — phase inventory, parsed by `roadmap.cjs` using current-milestone extraction
- Phase directories at `.planning/phases/{padded-number}-{slug}/` — contain `CONTEXT.md`, `PLAN-*.md`, `SUMMARY-*.md`

## Key Abstractions

**Skill:**
- Purpose: A named, discoverable behavioral pattern for agents
- Examples: `skills/brainstorming/SKILL.md`, `skills/systematic-debugging/SKILL.md`
- Pattern: Directory named after skill, contains one `SKILL.md` with YAML frontmatter + markdown body. Personal skills in `~/.claude/skills/` shadow installed skills by same name.

**Agent:**
- Purpose: A specialized subagent role with constrained tools and a clear mandate
- Examples: `agents/gsd-executor.md`, `agents/gsd-planner.md`, `agents/quality-gate.md`
- Pattern: Markdown file with YAML frontmatter (`name`, `model`, `tools`) + system prompt. Spawned via `Task(subagent_type="<name>")`. Model resolved at runtime from model-profiles based on project config.

**Command:**
- Purpose: A user-facing slash command that orchestrates a workflow
- Examples: `commands/gsd/plan-phase.md`, `commands/gsd/autonomous.md`
- Pattern: YAML frontmatter with `allowed-tools`, XML `<objective>` + `<execution_context>` + `<process>`. Orchestrators stay lean (15% context budget); subagents get 100% fresh context.

**Workflow:**
- Purpose: Step-by-step machine spec consumed by a command's orchestrator
- Examples: `gsd/workflows/execute-phase.md`, `gsd/workflows/plan-phase.md`
- Pattern: XML `<purpose>` + `<process>` with named `<step>` elements. Embedded bash for `gsd-tools.cjs` calls, agent spawn blocks with `<files_to_read>`, routing logic.

**Phase:**
- Purpose: A unit of project work with plans, context, and completion tracking
- Pattern: Directory `{padded-number}-{slug}` under `.planning/phases/`. Integer phases = planned milestone work; decimal phases (2.1) = inserted urgencies. Tracked in `ROADMAP.md` and `STATE.md`.

**Model Profile:**
- Purpose: Per-project quality/cost tradeoff for all agent model assignments
- Examples: `quality` (opus everywhere), `balanced` (default), `budget` (sonnet/haiku)
- Location: `gsd/bin/lib/model-profiles.cjs` — single source of truth for `MODEL_PROFILES` map

## Entry Points

**SessionStart Hook:**
- Location: `hooks/session-start`
- Triggers: Claude Code session start, resume, clear, compact (regex: `startup|resume|clear|compact`)
- Responsibilities: Injects `using-kinderpowers` skill content as `<EXTREMELY_IMPORTANT>` context block

**GSD Command Invocations:**
- Location: `commands/gsd/<command>.md`
- Triggers: User types `/gsd:<command>` in Claude Code
- Responsibilities: Parse arguments, load workflow, bootstrap with `gsd-tools.cjs`, orchestrate agents

**GSD Tools CLI:**
- Location: `gsd/bin/gsd-tools.cjs`
- Triggers: `node gsd-tools.cjs <subcommand>` from within workflow bash blocks
- Responsibilities: All filesystem state management — read config, find phases, parse roadmap, update state, resolve models, scaffold templates

**MCP Server Entry (sequential-thinking):**
- Location: `mcp-servers/sequential-thinking/src/main.rs`
- Triggers: Claude Code MCP tool call `sequential_thinking`
- Responsibilities: Maintains `ThinkingEngine` with thought history, branching, confidence tracking; serves `TuningProfile` per model; persists logs via `PersistentLogger`

**MCP Server Entry (github):**
- Location: `mcp-servers/github/src/main.rs`
- Triggers: Claude Code MCP tool calls for GitHub operations
- Responsibilities: GitHub API operations (issues, PRs, repos, files, actions, teams, releases) with compression and auth resolution

**Agent Outcome Logger:**
- Location: `hooks/agent-outcome-logger.py`
- Triggers: `PostToolUse` hook when `tool_name == "Agent"` completes
- Responsibilities: Appends structured JSONL to `~/.kinderpowers/agent_outcomes.jsonl`; extracts `variant:` tag for A/B tracking

## Error Handling

**Strategy:** Fail-fast with informative messages; workflows include explicit routing for known failure states; subagents handle their own errors before reporting back to orchestrator.

**Patterns:**
- `gsd-tools.cjs` calls `error()` which writes to stderr and `process.exit(1)` — caught by workflow bash blocks
- Missing `.planning/` directory → error with specific remediation command (`/gsd:new-project`)
- Missing phase → fallback to auto-detect next unplanned phase
- MCP server: `anyhow::Result` propagation, tracing to stderr (stdout reserved for JSON-RPC)
- Agent spawn fallback: if `Task()` subagent API unavailable, workflows specify sequential inline fallback

## Cross-Cutting Concerns

**Skill shadowing:** Personal skills at `~/.claude/skills/<name>/SKILL.md` override installed skills. Prefix `superpowers:` forces the installed version.

**Plugin namespace:** Commands are registered under `kinderpowers:gsd:*` namespace automatically by plugin system; NOT symlinked into `~/.claude/commands/` (avoids duplicates).

**GSD runtime path:** `setup.sh` symlinks `gsd/` to `~/.claude/get-shit-done`. Workflows reference `${CLAUDE_PLUGIN_ROOT}/gsd/` (plugin) and `~/.claude/get-shit-done/` (runtime) — both resolve to the same directory.

**Large payload routing:** `gsd-tools.cjs` `output()` writes to tmpfile and returns `@file:/tmp/gsd-*.json` when JSON > 50KB. Callers check for `@file:` prefix and read the file.

**Model profile resolution:** All agent model names (opus/sonnet/haiku) are aliases resolved at runtime from `MODEL_PROFILES` in `model-profiles.cjs` based on `.planning/config.json`'s `model_profile` field.

**Compulsion language enforcement:** `scanner.py` — detects patterns like "MUST" without escape, "not negotiable", "not optional" in skill files. Run in CI as `python scanner.py --check`. Skills use "strongly recommended" with explicit skip costs instead.

## Verified Layer Boundaries

**Skills ↔ GSD:** Skills layer has no dependency on GSD. Skills are pure markdown. GSD agents may invoke skills via `Skill()` tool but this is a runtime reference, not a compile-time dependency.

**Commands ↔ Agents:** Commands only reference agent type names (strings). Agent definitions live in `agents/*.md`. No command file imports an agent file.

**Workflows ↔ gsd-tools.cjs:** Workflows call gsd-tools via bash subprocess. No direct JS import. JSON output is the entire interface contract.

**MCP Servers ↔ Everything Else:** MCP servers are independently compiled Rust binaries. Zero shared code with skills, agents, commands, or gsd-tools. Connected only via Claude Code's MCP registration in `claude_desktop_config.json`.

**lib/skills-core.js ↔ hooks:** `skills-core.js` is a pure JS module used only by hook/session infrastructure. Not imported by gsd-tools.cjs or any workflow.

## Module Coupling

**`gsd/bin/gsd-tools.cjs`** (CLI router) imports:
- `lib/core.cjs` — shared utilities, config loader, output helpers
- `lib/state.cjs` — STATE.md CRUD
- `lib/phase.cjs` — phase directory operations
- `lib/roadmap.cjs` — ROADMAP.md parsing
- `lib/verify.cjs` — verification suite
- `lib/config.cjs` — `.planning/config.json` CRUD (imports `model-profiles.cjs`)
- `lib/template.cjs` — template scaffolding
- `lib/milestone.cjs` — milestone archive operations
- `lib/commands.cjs` — misc atomic commands
- `lib/init.cjs` — compound init commands (imports core, config, phase, roadmap)
- `lib/frontmatter.cjs` — YAML frontmatter parse/write
- `lib/profile-pipeline.cjs` — user profiling data pipeline
- `lib/profile-output.cjs` — user profile output formatter
- `lib/model-profiles.cjs` — `MODEL_PROFILES` constant, profile resolution

**`lib/core.cjs`** is the root dependency — imported by all other lib modules. Circular imports avoided: `core.cjs` does not import any other lib module.

**`lib/state.cjs`** imports `lib/frontmatter.cjs` (only). **`lib/phase.cjs`** imports `lib/core.cjs` + `lib/frontmatter.cjs` + `lib/state.cjs`. **`lib/init.cjs`** imports all operational modules (hub-and-spoke from core outward).

## Export Surface

**`lib/skills-core.js`** (ES module, used by hook infrastructure):
- `extractFrontmatter(filePath)` → `{name, description}`
- `findSkillsInDir(dir, sourceType, maxDepth)` → `Array<{path, skillFile, name, description, sourceType}>`
- `resolveSkillPath(skillName, superpowersDir, personalDir)` → `{skillFile, sourceType, skillPath} | null`
- `checkForUpdates(repoDir)` → `boolean`
- `stripFrontmatter(content)` → `string`

**`lib/model-profiles.cjs`** (CommonJS):
- `MODEL_PROFILES` — map of agent names to `{quality, balanced, budget}` model aliases
- `VALID_PROFILES` — `['quality', 'balanced', 'budget']`
- `getAgentToModelMapForProfile(normalizedProfile)` → `{agentName: modelAlias}`
- `formatAgentToModelMapAsTable(agentToModelMap)` → formatted string

**`gsd/bin/gsd-tools.cjs`** (CLI, not imported):
- Exposes ~60 subcommands via argv routing
- Primary interface: stdout JSON (or `@file:` path for large payloads)
- Error interface: stderr + exit 1

**MCP `sequential-thinking` server** (Rust, rmcp crate):
- Exposes one MCP tool: `sequential_thinking`
- Parameters: `SequentialThinkingParams` (thought, thought_number, total_thoughts, next_thought_needed, branching fields, continuation_mode, confidence, etc.)
- Resources: `/thinking/history`, `/thinking/branches`, `/thinking/summary` (MCP resource URIs)

**MCP `github` server** (Rust):
- Exposes GitHub API tools for: issues, PRs, repos, files, commits, branches, tags, releases, actions, teams, code search, user
- Auth: `GITHUB_TOKEN` env var via `auth::resolve_token()`

---

*Architecture analysis: 2026-03-19*
