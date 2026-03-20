# Changelog

## [6.2.0] — 2026-03-20

### Philosophy: Caller Controls Everything

6 phases executed in parallel via GSD, dogfooding kinderpowers on itself. 31 requirements, 16 plans, ~25 agents spawned.

### Added

**Phase 1: Sequential Thinking Spawn Hints**
- `spawn_candidate` hint with SpawnMeta (branch_points, recommended_depth, recommended_model)
- Enhanced MergeSummary with BranchOutcome (per-branch finalConfidence, doneReason) and convergenceSignal (converged/diverged/mixed/insufficient)
- `spawn_strategy` parameter in metathinking skill (none/convergent/divergent/hierarchical)
- 12 new Rust tests (123 total, 104 unit + 19 integration)

**Phase 2: Agent Collapse (16 → 8)**
- `gsd-researcher` replaces phase-researcher + project-researcher + research-synthesizer (mode: phase/project/synthesize)
- `gsd-verifier` replaces verifier + integration-checker + plan-checker + nyquist-auditor (mode: goal-backward/integration/plan-quality/coverage)
- `gsd-ui` replaces ui-researcher + ui-auditor + ui-checker (mode: spec/audit/validate)
- `gsd-planner` absorbs roadmapper via scope parameter (phase/milestone/project)
- 10 old agents replaced with thin alias stubs (backward compatible)
- 12 workflow files + 3 command files + model-profiles.cjs migrated
- Net reduction: 4,429 lines

**Phase 3: Beads Integration**
- `beadsAvailable()` cached check + 6 bead helper functions in core.cjs
- `bead` subcommands in gsd-tools.cjs (available, create, update, close, show)
- Bead lifecycle wired into 5 GSD workflows: new-project (epic), plan-phase (task), execute-phase (in_progress), verify (evidence), ship (close with PR)
- Graceful degradation: zero errors when beads not installed

**Phase 4: Agent Parameterization**
- code-reviewer: focus, pedanticness, scope
- research-extractor: mode, depth, output
- team-coordinator: worker_count, worker_model, isolation, coordination
- gsd-debugger: method, max_hypotheses, checkpoint_frequency, escalation

**Phase 5: Skill Parameterization**
- systematic-debugging: depth, hypothesis_count, reproduce_first
- brainstorming: breadth, mode, time_box
- test-driven-development: strictness, coverage_target, test_style
- verification-before-completion: evidence_bar, auto_run, check_types
- adversarial-review: intensity, min_findings, focus
- subagent-driven-development: worker_model, review_between, parallelism

**Phase 6: Team Mode**
- GSD workflows use TeamCreate + named Agent spawns instead of Task(background)
- map-codebase: TeamCreate("gsd-mapping") with 4 named mapper agents
- execute-phase: per-wave TeamCreate with named executor agents
- new-project: TeamCreate("gsd-research") with 4 researchers + synthesizer
- 4 agents gain Team Communication sections (mapper, executor, researcher, verifier)
- Fallback to Task(background) when TeamCreate unavailable
- Graceful degradation: SendMessage calls skipped silently when not in team

### Changed

- All version references bumped 6.1.0 → 6.2.0

## [6.1.0] — 2026-03-19

### Philosophy: Caller Controls

Skills and agents now expose parameters with sensible defaults. The caller tunes via their prompt.
The system parses what it recognizes and applies defaults for the rest. One-size-fits-all is the anti-pattern.

### Added

**kp-sequential-thinking MCP server**
- Hints system: non-prescriptive observations (linear_chain, premature_confidence, merge_available, etc.)
- Branch merge: `continuation_mode: "merge"` with `merge_branches` to synthesize insights (GoT-inspired)
- Dunning-Kruger detection: flags confidence > 80% at layer 1 as "observation"
- 106 tests (87 unit + 19 integration)

**Parameterized agents** — all accept caller-controlled parameters:
- `multi-perspective-review`: 5 modes (council, troll-test, adversarial, gentle, focused), lens_count 2-7, pedanticness slider
- `gsd-codebase-mapper`: depth slider, JSONL index output, expiry, LSP/AST probe step, multi-repo support
- `strategic-planner`: depth, approaches count, phase granularity, output format
- `quality-gate`: strictness slider, evidence types, min checks, security scan toggle
- `gsd-executor`: caution level, commit style, parallelism, deviation handling, test frequency

**Codebase mapper intelligence sources**
- LSP tool added (documentSymbol, incomingCalls, outgoingCalls for real call graphs)
- Probe step discovers available tools: LSP, ast.parse, pyright, tsc, ck, ZMCPTools
- JSONL sidecar output with expiry dates for search index ingestion

**Recommended tools section** in KINDERPOWERS.xml
- beads, beads-viewer, BeaconBay/ck, ultimate-bug-scanner

**Paper citations** in KINDERPOWERS.xml references section
- ToT (Yao 2023), GoT (Besta 2023), AGoT (Singh 2025), DeepConf (Zhao 2025)
- Cycles of Thought (Belem 2024), FaR (Qiu 2024), D-K in LLMs (2025)
- Brenner "Third Alternative" pattern

**CI** — GitHub Actions workflow for MCP server tests and GSD tools verification

**TOON format** — added as submodule at `specs/toon-format`

### Changed

- `metathinking` skill: removed mandatory/enforcement language, aligned with hints philosophy
- `KINDERPOWERS.xml`: expanded agent catalog with tools, when-to-use, output format
- `gsd-tools.cjs`: fixed MODULE_NOT_FOUND when invoked from different cwd (__dirname-relative requires)
- `install.sh` / `upgrade.sh` / README: fixed kp-seqthink → kp-sequential-thinking naming
- `marketplace.json`: synced version to 6.0.0 → 6.1.0
- `plugin.json`: added optionalComponents for MCP servers, toon-format to incorporates

## [6.0.0] — 2026-03-18

Kinderpowers v6.0: An operating system for AI agents.

### Added

**Lifecycle Engine (GSD)**
- Integrated [get-shit-done](https://github.com/davidjbauer/get-shit-done) v1.26.0 as the lifecycle engine
- 44 workflow definitions in `gsd/workflows/`
- 42 slash commands in `commands/gsd/` covering the full development lifecycle
- 16 GSD agents for autonomous project delivery
- `gsd-tools.cjs` runtime with state management, model resolution, and phase tracking

**New Skills**
- `strategic-planning` — discovery-before-creation, investigative vs implementation modes
- `metathinking` — sequential thinking with mandatory branching, confidence tracking, Brenner pattern
- `research-extraction` — harvest → extract → analyze → rank → verify pipeline with 3 routing modes
- `dispatching-to-runtimes` — prompt structuring for Gemini, GPT, and local model dispatch
- `team-orchestration` — Claude Code teams patterns, worker sizing, file domain separation, INJECT patterns
- `remembering-conversations` — conversation history search (moved from marketplace-only to repo)

**New Agents**
- `strategic-planner` — goal → discovery → phased plan
- `quality-gate` — adversarial verification, refuses to pass without evidence
- `team-coordinator` — orchestrates parallel Claude Code agent teams
- `research-extractor` — routes between idea extraction, usage evaluation, and deep integration

**Enforcement (Hookify Rules)**
- `verification-required` — blocks completion without verification evidence
- `discovery-before-creation` — warns before creating new files without searching
- `brainstorm-before-build` — warns before writing 100+ lines without design discussion
- All rules ship disabled — users opt-in

**Infrastructure**
- `setup.sh` — post-install script for symlinks and hookify rule installation
- `KINDERPOWERS.xml` — machine-readable manifest for AI agent consumption
- Progression model: L1 Coding Assistant → L2 Agentic Worker → L3 Team Orchestrator → L4 Dark Factory

### Enhanced

- `writing-plans` — added discovery-before-creation and extend-over-duplicate strategies
- `executing-plans` — added bead claim protocol, verify-before-assuming, parallel patterns, explicit-instructions mode
- `verification-before-completion` — added deep inspection checklist and agent delegation verification
- `plugin.json` — updated to v6.0.0 with expanded keywords

### Credits

- [superpowers](https://github.com/obra/superpowers) by Jesse Vincent — craft philosophy, skill format, scanner, hook system
- [get-shit-done](https://github.com/davidjbauer/get-shit-done) by Davíd Braun — lifecycle engine, commands, agents, workflows
- [hookify](https://github.com/QuantGeekDev/hookify) by Diego Perez — enforcement rule format
- [jw409](https://github.com/jw409) — progression model, agency-preserving philosophy, council mode, new skills and agents

## [5.1.0] — 2025-03-17

- Added adversarial-review, architecture, beads, requirements, retrospective skills
- Hub-and-spoke discovery via find-skills
- Scanner improvements

## [5.0.0] — 2025-03-08

- Initial kinderpowers release forked from superpowers v4.3.1
- 20 skills, 1 agent (code-reviewer), compulsion language scanner
