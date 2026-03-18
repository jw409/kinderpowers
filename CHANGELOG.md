# Changelog

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

## [5.1.0] — 2025-03-17

- Added adversarial-review, architecture, beads, requirements, retrospective skills
- Hub-and-spoke discovery via find-skills
- Scanner improvements

## [5.0.0] — 2025-03-08

- Initial kinderpowers release forked from superpowers v4.3.1
- 20 skills, 1 agent (code-reviewer), compulsion language scanner
