# Roadmap: kinderpowers v6.2

## Overview

kinderpowers v6.2 propagates the "server hints, caller controls" philosophy across the entire system. Starting with Rust-level spawn hints in sequential thinking, then collapsing 16 GSD agents into 8 parameterized ones, wiring beads into the GSD lifecycle, and finishing by parameterizing all remaining agents and skills. Each phase delivers independently verifiable capability.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Sequential Thinking Spawn Hints** - Rust MCP server surfaces spawn candidates and enhanced merge summaries
- [ ] **Phase 2: Agent Collapse** - Consolidate 16 GSD agents into 8 parameterized agents with aliases
- [ ] **Phase 3: Beads Integration** - Wire bead lifecycle into GSD commands for cross-session memory
- [ ] **Phase 4: Agent Parameterization** - Add caller-controlled parameters to remaining 4 agents
- [ ] **Phase 5: Skill Parameterization** - Add caller-controlled parameters to 6 skills

## Phase Details

### Phase 1: Sequential Thinking Spawn Hints
**Goal**: The sequential thinking MCP server detects when thinking branches warrant subagent spawning and communicates this to callers via structured hints
**Depends on**: Nothing (first phase)
**Requirements**: SPAWN-01, SPAWN-02, SPAWN-03, SPAWN-04, SPAWN-05
**Success Criteria** (what must be TRUE):
  1. When sequential_thinking detects an explore or branch point, the response includes a spawn_candidate hint with branch points, recommended depth, and recommended model
  2. When branches merge, the mergeSummary includes per-branch finalConfidence, doneReason, and a convergenceSignal indicating whether branches agreed
  3. The metathinking skill accepts spawn_strategy parameter (none/convergent/divergent/hierarchical) and routes accordingly
  4. At least 10 new Rust tests pass covering spawn hint generation and enhanced merge behavior
**Plans:** 3 plans
Plans:
- [ ] 01-01-PLAN.md — Add spawn_candidate hint with SpawnMeta to thinking engine
- [ ] 01-02-PLAN.md — Enhance MergeSummary with branch outcomes and convergence signal
- [ ] 01-03-PLAN.md — Add spawn_strategy parameter to metathinking skill

### Phase 2: Agent Collapse
**Goal**: Callers interact with 8 parameterized GSD agents instead of 16 fixed-purpose ones, with old names still working as aliases
**Depends on**: Nothing (independent of Phase 1 -- different file domains)
**Requirements**: COLLAPSE-01, COLLAPSE-02, COLLAPSE-03, COLLAPSE-04, COLLAPSE-05, COLLAPSE-06
**Success Criteria** (what must be TRUE):
  1. `Task(subagent_type="gsd-researcher", mode="phase")` works identically to the old `Task(subagent_type="gsd-phase-researcher")`
  2. `Task(subagent_type="gsd-verifier", mode="integration")` works identically to the old `Task(subagent_type="gsd-integration-checker")`
  3. `Task(subagent_type="gsd-ui", mode="spec")` works identically to the old `Task(subagent_type="gsd-ui-researcher")`
  4. Old agent names (e.g., gsd-phase-researcher) still resolve via thin alias files that set defaults and delegate
  5. All GSD workflow files reference the new parameterized agent names
**Plans:** 3/4 plans executed
Plans:
- [ ] 02-01-PLAN.md — Create parameterized gsd-researcher and gsd-verifier agents
- [ ] 02-02-PLAN.md — Create parameterized gsd-ui agent and add scope to gsd-planner
- [ ] 02-03-PLAN.md — Create thin alias files for 10 old agent names
- [ ] 02-04-PLAN.md — Update workflow files and model-profiles to use new agent names

### Phase 3: Beads Integration
**Goal**: GSD lifecycle commands create, update, and close beads automatically -- providing cross-session project memory that survives compaction
**Depends on**: Phase 2 (workflow files updated in Phase 2 are the same files that gain bead calls in Phase 3)
**Requirements**: BEADS-01, BEADS-02, BEADS-03, BEADS-04, BEADS-05, BEADS-06
**Success Criteria** (what must be TRUE):
  1. Running /gsd:new-project creates an epic bead when `bd` is available
  2. Running /gsd:plan-phase creates a task bead as a child of the project epic
  3. Phase execution and verification update bead status (in_progress, evidence attached)
  4. Running /gsd:ship closes the phase bead with a PR link
  5. All bead operations degrade gracefully (no errors, just skipped) when beads CLI is not installed
**Plans**: TBD

### Phase 4: Agent Parameterization
**Goal**: The 4 remaining non-parameterized agents (code-reviewer, research-extractor, team-coordinator, gsd-debugger) accept caller-controlled parameters
**Depends on**: Phase 2 (collapse creates the parameterization patterns that Phase 4 follows)
**Requirements**: PARAM-01, PARAM-02, PARAM-03, PARAM-04
**Success Criteria** (what must be TRUE):
  1. code-reviewer accepts focus, pedanticness (1-5), and scope parameters with sensible defaults
  2. research-extractor accepts mode, depth, and output parameters
  3. team-coordinator accepts worker_count, worker_model, isolation, and coordination parameters
  4. gsd-debugger accepts method, max_hypotheses, checkpoint_frequency, and escalation parameters
**Plans**: TBD

### Phase 5: Skill Parameterization
**Goal**: 6 skills accept caller-controlled parameters in their YAML frontmatter, completing the "server hints, caller decides" vision across all layers
**Depends on**: Phase 4 (establishes parameter conventions that skills follow)
**Requirements**: PARAM-05, PARAM-06, PARAM-07, PARAM-08, PARAM-09, PARAM-10
**Success Criteria** (what must be TRUE):
  1. systematic-debugging skill accepts depth, hypothesis_count, reproduce_first parameters
  2. brainstorming skill accepts breadth, mode, time_box parameters
  3. test-driven-development skill accepts strictness, coverage_target, test_style parameters
  4. verification-before-completion skill accepts evidence_bar, auto_run, check_types parameters
  5. adversarial-review and subagent-driven-development skills accept their respective parameters
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5
Note: Phases 1 and 2 are independent (different file domains) and could execute in parallel.

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Sequential Thinking Spawn Hints | 0/3 | Planning complete | - |
| 2. Agent Collapse | 3/4 | In Progress|  |
| 3. Beads Integration | 0/TBD | Not started | - |
| 4. Agent Parameterization | 0/TBD | Not started | - |
| 5. Skill Parameterization | 0/TBD | Not started | - |
