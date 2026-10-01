# Codebase Structure

**Analysis Date:** 2026-03-19

## Directory Layout

```
kp-sequential-thinking/          # Kinderpowers plugin root
├── skills/                      # 27 skill definitions (L1–L3)
│   └── <skill-name>/
│       └── SKILL.md
├── agents/                      # 22 agent definitions
│   ├── <name>.md                # 6 kinderpowers agents
│   └── (gsd agents in gsd/agents/)
├── commands/
│   └── gsd/                     # 42 GSD slash commands
│       └── <command>.md
├── gsd/                         # GSD lifecycle engine (embedded sub-repo, v1.26.0)
│   ├── bin/
│   │   ├── gsd-tools.cjs        # CLI router (~130 subcommands)
│   │   └── lib/                 # 14 CommonJS modules
│   ├── agents/                  # 16 GSD-specific agent definitions
│   ├── commands/
│   │   └── gsd/                 # GSD command definitions (upstream copy)
│   ├── hooks/                   # 4 hook scripts (update checker, statusline, etc.)
│   ├── workflows/               # 43 workflow execution specs
│   ├── templates/               # Document templates (PLAN.md, STATE.md, etc.)
│   ├── references/              # Reference docs (model-profiles, git-integration, etc.)
│   ├── get-shit-done/           # Upstream GSD copy (reference only)
│   ├── scripts/                 # build-hooks.js, run-tests.cjs
│   ├── package.json             # Node package (get-shit-done-cc v1.26.0)
│   └── VERSION
├── mcp-servers/                 # 2 optional Rust MCP servers
│   ├── sequential-thinking/
│   │   └── src/                 # main.rs, server.rs, thinking.rs, profiles.rs, logging.rs
│   ├── github/
│   │   └── src/                 # main.rs, server.rs, auth.rs, tools/, github/, query/
│   └── bin/                     # Pre-compiled binaries (linux-x86_64, macos-arm64)
├── hooks/                       # SessionStart + outcome logging
│   ├── hooks.json               # Hook registration spec
│   ├── session-start            # Bash: injects using-kinderpowers at session start
│   ├── run-hook.cmd             # Windows shim
│   └── agent-outcome-logger.py  # PostToolUse: logs Agent completions to JSONL
├── hookify-rules/               # 3 enforcement rules (opt-in via hookify)
│   ├── brainstorm-before-build.local.md
│   ├── discovery-before-creation.local.md
│   └── verification-required.local.md
├── lib/                         # Shared JS for hook infrastructure
│   └── skills-core.js           # Skill discovery, resolution, frontmatter parsing
├── specs/
│   └── toon-format/             # TOON output format spec
├── tests/                       # Test suite
├── docs/                        # User-facing documentation
├── KINDERPOWERS.xml             # Machine-readable manifest (v6.1.0)
├── setup.sh                     # Post-install symlink wiring (idempotent)
└── scanner.py                   # Compulsion-language scanner for skill files
```

## Directory Purposes

**`skills/`:**
- Purpose: Behavioral pattern library — one directory per skill, each containing `SKILL.md`
- Contains: YAML frontmatter (name, description) + markdown guidance body
- Key files: `skills/using-kinderpowers/SKILL.md` (bootstrapped at session start), `skills/brainstorming/SKILL.md`, `skills/systematic-debugging/SKILL.md`, `skills/metathinking/SKILL.md`
- Naming: kebab-case directory names matching skill name field

**`agents/`:**
- Purpose: The 6 non-GSD specialized agents (code-reviewer, multi-perspective-review, strategic-planner, quality-gate, team-coordinator, research-extractor)
- Contains: Markdown with YAML frontmatter (`name`, `description`, `model`, `tools`) + system prompt body
- Key files: `agents/multi-perspective-review.md` (spawns 2–7 lens subagents), `agents/team-coordinator.md` (parallel work orchestration)

**`commands/gsd/`:**
- Purpose: User-facing slash commands — each is `/gsd:<filename>` invoked via Claude Code
- Contains: Command definitions with `<objective>`, `<execution_context>`, `<context>`, `<process>` XML blocks
- Key files: `commands/gsd/autonomous.md`, `commands/gsd/plan-phase.md`, `commands/gsd/execute-phase.md`, `commands/gsd/new-project.md`, `commands/gsd/verify-work.md`, `commands/gsd/map-codebase.md`

**`gsd/`:**
- Purpose: Complete GSD lifecycle engine (get-shit-done v1.26.0) — manages project planning state
- Contains: Node.js CLI, workflow specs, templates, agent definitions, references
- Key files: `gsd/bin/gsd-tools.cjs` (CLI), `gsd/bin/lib/model-profiles.cjs` (agent model map), `gsd/bin/lib/init.cjs` (workflow bootstrapping), `gsd/bin/lib/state.cjs` (STATE.md operations)

**`gsd/bin/lib/`:**
- Purpose: CommonJS modules behind `gsd-tools.cjs` — never imported directly by workflows
- Key modules:
  - `core.cjs` — shared utils, `loadConfig()`, `output()`, `error()`, path helpers
  - `state.cjs` — `STATE.md` CRUD (reads both `**Field:**` bold format and plain `Field:` format)
  - `phase.cjs` — phase directory operations, decimal phase sorting
  - `roadmap.cjs` — `ROADMAP.md` parsing, phase section extraction, current-milestone scoping
  - `config.cjs` — `.planning/config.json` CRUD with dot-notation key paths
  - `model-profiles.cjs` — `MODEL_PROFILES` constant, model alias resolution
  - `init.cjs` — compound init commands that pre-compute all context for each workflow type
  - `verify.cjs` — SUMMARY.md validation, plan structure checks, artifact existence
  - `frontmatter.cjs` — YAML frontmatter parse/write for all `.planning/` documents
  - `template.cjs` — scaffold new PLAN.md, SUMMARY.md, VERIFICATION.md, CONTEXT.md
  - `milestone.cjs` — milestone archive, create `MILESTONES.md`

**`gsd/workflows/`:**
- Purpose: Step-by-step execution specs referenced by commands via `<execution_context>` blocks
- Contains: XML documents with `<purpose>`, `<process>`, `<step>` elements + embedded bash
- Key files: `gsd/workflows/execute-phase.md`, `gsd/workflows/plan-phase.md`, `gsd/workflows/autonomous.md`, `gsd/workflows/new-project.md`, `gsd/workflows/verify-work.md`

**`gsd/agents/`:**
- Purpose: The 16 GSD lifecycle worker agents used exclusively by GSD commands
- Key files: `gsd/agents/gsd-executor.md`, `gsd/agents/gsd-planner.md`, `gsd/agents/gsd-verifier.md`, `gsd/agents/gsd-phase-researcher.md`, `gsd/agents/gsd-codebase-mapper.md`

**`gsd/templates/`:**
- Purpose: Document templates used by `gsd-tools.cjs template fill` commands
- Key files: `gsd/templates/state.md`, `gsd/templates/roadmap.md`, `gsd/templates/context.md`, `gsd/templates/summary.md`, `gsd/templates/codebase/` (7 codebase analysis templates)

**`gsd/references/`:**
- Purpose: Reference docs loaded via `<execution_context>` — not templates, but behavioral specs
- Key files: `gsd/references/model-profiles.md`, `gsd/references/planning-config.md`, `gsd/references/verification-patterns.md`, `gsd/references/ui-brand.md`

**`mcp-servers/sequential-thinking/`:**
- Purpose: Rust MCP server providing enhanced sequential thinking with branching, confidence tracking, and per-model tuning profiles
- Key files: `src/thinking.rs` (ThinkingEngine + ThoughtData), `src/profiles.rs` (TuningProfile + default_profiles()), `src/server.rs` (MCP tool handler), `src/logging.rs` (PersistentLogger)

**`mcp-servers/github/`:**
- Purpose: Rust MCP server wrapping the GitHub REST API with authentication, compression, and formatting
- Key files: `src/tools/` (one module per resource type), `src/github/` (client, types), `src/auth.rs`, `src/server.rs`

**`mcp-servers/bin/`:**
- Purpose: Pre-compiled binaries for distribution (no build step required for end users)
- Contains: `linux-x86_64/`, `macos-arm64/` — compiled binaries committed to repo
- Generated: Yes (build output)
- Committed: Yes

**`hooks/`:**
- Purpose: Runtime hook infrastructure — session injection + outcome telemetry
- Key files: `hooks/hooks.json` (Claude hook registration), `hooks/session-start` (bash injection script), `hooks/agent-outcome-logger.py` (JSONL logging)

**`hookify-rules/`:**
- Purpose: Opt-in enforcement rules for the hookify plugin (separate install)
- Contains: `.local.md` files with YAML rule specs — `setup.sh` links these into hookify's rules directory
- Note: All 3 rules are `enabled: false` by default — users must enable explicitly

**`lib/`:**
- Purpose: Shared JavaScript for hook/session infrastructure
- Key files: `lib/skills-core.js` — the only JS library in kinderpowers proper (separate from gsd/bin/lib/)

**`.planning/codebase/`:**
- Purpose: Generated codebase analysis documents (output of `/gsd:map-codebase`)
- Generated: Yes (by gsd-codebase-mapper agents)
- Committed: Yes (per `commit_docs` config)

## Key File Locations

**Entry Points:**
- `hooks/session-start` — SessionStart hook that bootstraps skill context
- `commands/gsd/*.md` — all 42 user-facing slash commands
- `gsd/bin/gsd-tools.cjs` — CLI runtime engine

**Configuration:**
- `gsd/bin/lib/model-profiles.cjs` — model assignments per profile
- `gsd/bin/lib/config.cjs` — config schema and defaults
- `.planning/config.json` — per-project runtime config (in target project, not here)
- `KINDERPOWERS.xml` — plugin manifest (machine + human readable)
- `.claude-plugin/plugin.json` — Claude Code plugin registration

**Core Logic:**
- `gsd/bin/lib/core.cjs` — root utilities shared by all gsd-tools modules
- `gsd/bin/lib/init.cjs` — compound init commands that prepare workflow context
- `lib/skills-core.js` — skill discovery and shadowing resolution

**Testing:**
- `gsd/tests/` — GSD-specific tests (run via `node scripts/run-tests.cjs`)
- `tests/` — top-level tests (kinderpowers-specific)
- `scanner.py` + `test_scanner.py` — scanner unit tests
- `gsd/scripts/run-tests.cjs` — test runner script

## Naming Conventions

**Files:**
- Skills: `skills/<kebab-name>/SKILL.md` — UPPERCASE `SKILL.md` always
- Agents: `agents/<kebab-name>.md` — lowercase kebab filename matching `name:` in frontmatter
- Commands: `commands/gsd/<kebab-name>.md` — matches `/gsd:<kebab-name>` invocation
- GSD lib modules: `<domain>.cjs` — all CommonJS, all lowercase

**Directories:**
- Skills: `skills/<kebab-name>/` — matches skill name exactly
- Phase dirs: `.planning/phases/{padded-number}-{slug}/` e.g. `01-setup`, `02-api`, `10-polish`
- Decimal phases: `02.1-hotfix` (sorted numerically between integer phases)

**YAML Frontmatter:**
- Skills: `name:` (matches dir name), `description:` (trigger condition format: "Use when...")
- Agents: `name:`, `description:`, `model:`, `tools:`, optional `color:`
- Commands: `name:` (includes namespace e.g. `gsd:plan-phase`), `description:`, `argument-hint:`, `allowed-tools:`
- Plans: frontmatter includes `phase:`, `plan:`, `wave:`, optional `gap_closure: true`

**Rust MCP servers:** Module files named after their domain (`thinking.rs`, `profiles.rs`, `auth.rs`, `compress.rs`). Tool submodules in `src/tools/<resource>.rs`.

## Where to Add New Code

**New Skill:**
- Create: `skills/<name>/SKILL.md` with YAML frontmatter (`name`, `description`) + markdown body
- Declare level (L1–L3) in description based on complexity
- Run: `python scanner.py skills/<name>/SKILL.md` to check for compulsion language
- Note: No registration required — `findSkillsInDir()` discovers automatically

**New Agent:**
- Create: `agents/<name>.md` with frontmatter + system prompt
- Register in `KINDERPOWERS.xml` under `<agent-catalog>`
- Model default goes in `gsd/bin/lib/model-profiles.cjs` `MODEL_PROFILES` if GSD lifecycle agent

**New GSD Command:**
- Create: `commands/gsd/<name>.md` with XML structure matching existing commands
- Create matching workflow: `gsd/workflows/<name>.md`
- Add init command to `gsd/bin/lib/init.cjs` if new context patterns needed
- Reference workflow via `<execution_context>@${CLAUDE_PLUGIN_ROOT}/gsd/workflows/<name>.md</execution_context>`

**New gsd-tools subcommand:**
- Add implementation in appropriate `gsd/bin/lib/<domain>.cjs` module
- Export function from module
- Add routing case in `gsd/bin/gsd-tools.cjs` CLI router
- Add to command reference comment block at top of `gsd-tools.cjs`

**New Hookify Rule:**
- Create: `hookify-rules/<name>.local.md` with YAML frontmatter (`name`, `enabled`, `event`, `action`, `conditions`)
- Set `enabled: false` by default (agency-preserving)
- Rules auto-linked by `setup.sh` if hookify is installed

**New MCP Server Tool:**
- Add tool handler in `mcp-servers/<server>/src/tools/<resource>.rs`
- Register in `mcp-servers/<server>/src/server.rs` tool router
- Rebuild binary and add to `mcp-servers/bin/`

## Special Directories

**`gsd/get-shit-done/`:**
- Purpose: Upstream GSD reference copy (mirrors `glittercowboy/get-shit-done`)
- Generated: No (checked in)
- Committed: Yes
- Note: `setup.sh` symlinks `gsd/` to `~/.claude/get-shit-done` — the entire `gsd/` subtree acts as the runtime, not just `get-shit-done/`

**`.claude-plugin/`:**
- Purpose: Claude Code plugin registration files (`plugin.json`, `marketplace.json`)
- Generated: No
- Committed: Yes

**`.planning/`:**
- Purpose: GSD project planning state (created by `/gsd:new-project` in the target project)
- Not present in this repo root (kinderpowers itself is the plugin, not a project being planned)
- Generated: Yes (by GSD commands)
- Committed: Yes (per `commit_docs` default)

**`mcp-servers/bin/`:**
- Purpose: Pre-compiled MCP server binaries for immediate use without Rust toolchain
- Generated: Yes (build artifacts)
- Committed: Yes (for distribution)

---

*Structure analysis: 2026-03-19*
