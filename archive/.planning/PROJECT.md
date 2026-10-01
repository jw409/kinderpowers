# kinderpowers v6.2

## What This Is

An operating system for AI agents — skills, lifecycle engine, MCP servers, and enforcement. v6.2 evolves the GSD engine from rigid agent types to parameterized, hints-based tools where the caller controls depth, mode, and strategy.

## Core Value

Every agent and skill is a parameterized canvas with sensible defaults. The server hints, the caller decides. One-size-fits-all is the anti-pattern.

## Requirements

### Validated

- ✓ 27 skills with auto-injection via hooks — existing
- ✓ 22 agents (5 kinderpowers + 16 GSD + 1 multi-perspective-review) — existing
- ✓ 42 GSD slash commands — existing
- ✓ 2 Rust MCP servers (kp-github-mcp, kp-sequential-thinking) — existing
- ✓ Hints system in sequential thinking (6 hint types) — v6.1.0
- ✓ Branch merge capability — v6.1.0
- ✓ Dunning-Kruger detection — v6.1.0
- ✓ LSP-first codebase mapper — v6.1.0
- ✓ Parameterized: multi-perspective-review, codebase-mapper, strategic-planner, quality-gate, gsd-executor — v6.1.0
- ✓ Paper citations in KINDERPOWERS.xml — v6.1.0

### Active

**#2 — Sequential thinking + subagent orchestration**
- [ ] SPAWN-01: Server surfaces `spawn_candidate` hint when explore/branch detected
- [ ] SPAWN-02: Enhanced mergeSummary with branch outcomes and convergenceSignal
- [ ] SPAWN-03: Metathinking skill gains spawn_strategy parameter (none/convergent/divergent/hierarchical)
- [ ] SPAWN-04: Tests for spawn hint generation and enhanced merge

**#4 — Collapse 16 GSD agents into ~8 parameterized**
- [ ] COLLAPSE-01: gsd-researcher replaces phase-researcher + project-researcher + research-synthesizer
- [ ] COLLAPSE-02: gsd-verifier replaces verifier + integration-checker + plan-checker + nyquist-auditor
- [ ] COLLAPSE-03: gsd-ui replaces ui-researcher + ui-auditor + ui-checker
- [ ] COLLAPSE-04: gsd-planner absorbs roadmapper (scope=phase vs scope=project)
- [ ] COLLAPSE-05: Old agent names kept as aliases during transition
- [ ] COLLAPSE-06: Workflow files updated to use new parameterized agents

**#5 — Beads integration with GSD**
- [ ] BEADS-01: /gsd:new-project creates epic bead
- [ ] BEADS-02: /gsd:plan-phase creates task bead as child of epic
- [ ] BEADS-03: /gsd:execute-phase marks bead in_progress
- [ ] BEADS-04: /gsd:verify-work attaches evidence to bead
- [ ] BEADS-05: /gsd:ship closes bead with PR link
- [ ] BEADS-06: Graceful degradation when beads not installed

**#6 — Parameterize remaining agents and skills**
- [ ] PARAM-01: code-reviewer (focus, pedanticness, scope)
- [ ] PARAM-02: research-extractor (mode, depth, output)
- [ ] PARAM-03: team-coordinator (worker_count, worker_model, isolation, coordination)
- [ ] PARAM-04: gsd-debugger (method, max_hypotheses, checkpoint_frequency)
- [ ] PARAM-05: systematic-debugging skill (depth, hypothesis_count, reproduce_first)
- [ ] PARAM-06: brainstorming skill (breadth, mode, time_box)
- [ ] PARAM-07: test-driven-development skill (strictness, coverage_target, test_style)
- [ ] PARAM-08: verification-before-completion skill (evidence_bar, auto_run, check_types)
- [ ] PARAM-09: adversarial-review skill (intensity, min_findings, focus)
- [ ] PARAM-10: subagent-driven-development skill (worker_model, review_between, parallelism)

### Out of Scope

- Prebuilt binary releases (GitHub Actions) — separate infrastructure work
- GSD upstream wholesale sync — #3 is evolution, not sync
- Learning pipeline (scavenger/teacher) — that's TalentOS, not kinderpowers
- Party mode / inter-agent communication — future issue, not v6.2

## Context

kinderpowers v6.1.0 established the philosophy: server hints, caller controls. v6.2 propagates this across the entire system. The brownfield mapper now uses LSP-first intelligence. The concerns mapper found 4 HIGH items and 17 structured findings.

Codebase: 27 skills, 22 agents, 42 commands, 2 MCP servers (606 Rust tests), 116 tracked GSD files.

## Constraints

- **No breaking changes**: old agent names must work as aliases
- **Beads optional**: GSD must work without beads installed
- **Test coverage**: every new feature needs tests (Rust: cargo test, GSD: node verification)
- **Push to jw409/kinderpowers**: all changes go to the public repo

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Hints not mandates | Callers vary in capability (haiku→opus). Server observes, doesn't enforce. | ✓ Good |
| Vendor GSD, don't submodule | We want to evolve it (LSP mapper, parameterized agents), not just track upstream | — Pending |
| Agent collapse 16→8 | Same capabilities, fewer types, caller controls the mode | — Pending |
| Beads as GSD memory layer | STATE.md is session-scoped, beads survive compaction/sessions/machines | — Pending |

---
*Last updated: 2026-03-19 after initialization*
