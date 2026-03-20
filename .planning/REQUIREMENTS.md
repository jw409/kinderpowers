# Requirements: kinderpowers v6.2

**Defined:** 2026-03-19
**Core Value:** Every agent and skill is a parameterized canvas — server hints, caller decides

## v1 Requirements

### Sequential Thinking Orchestration (#2)

- [x] **SPAWN-01**: kp-sequential-thinking server surfaces `spawn_candidate` hint with spawn_meta (branch points, recommended depth, recommended model)
- [x] **SPAWN-02**: Enhanced mergeSummary includes branchOutcomes (finalConfidence, doneReason per branch) and convergenceSignal
- [x] **SPAWN-03**: Metathinking skill parses spawn_strategy parameter (none/convergent/divergent/hierarchical)
- [x] **SPAWN-04**: Rust tests for spawn hint generation (>= 5 new tests)
- [x] **SPAWN-05**: Rust tests for enhanced merge with branch outcomes (>= 5 new tests)

### Agent Collapse (#4)

- [x] **COLLAPSE-01**: gsd-researcher agent with mode parameter (phase/project/synthesize) replaces 3 agents
- [x] **COLLAPSE-02**: gsd-verifier agent with mode parameter (goal-backward/integration/plan-quality/coverage) replaces 4 agents
- [x] **COLLAPSE-03**: gsd-ui agent with mode parameter (spec/audit/validate) replaces 3 agents
- [x] **COLLAPSE-04**: gsd-planner absorbs roadmapper via scope parameter (phase/milestone/project)
- [x] **COLLAPSE-05**: Old agent .md files become thin aliases (set defaults, delegate to new agent)
- [x] **COLLAPSE-06**: GSD workflow files reference new parameterized agents

### Beads Integration (#5)

- [x] **BEADS-01**: /gsd:new-project creates epic bead when beads available
- [x] **BEADS-02**: /gsd:plan-phase creates task bead as child of epic
- [x] **BEADS-03**: /gsd:execute-phase marks phase bead in_progress
- [x] **BEADS-04**: /gsd:verify-work attaches verification evidence to bead notes
- [x] **BEADS-05**: /gsd:ship closes phase bead with PR link
- [x] **BEADS-06**: beadsAvailable() check in gsd-tools.cjs — graceful degradation

### Parameterization (#6)

- [x] **PARAM-01**: code-reviewer agent: focus, pedanticness, scope parameters
- [x] **PARAM-02**: research-extractor agent: mode, depth, output parameters
- [ ] **PARAM-03**: team-coordinator agent: worker_count, worker_model, isolation, coordination parameters
- [ ] **PARAM-04**: gsd-debugger agent: method, max_hypotheses, checkpoint_frequency, escalation parameters
- [ ] **PARAM-05**: systematic-debugging skill: depth, hypothesis_count, reproduce_first parameters
- [ ] **PARAM-06**: brainstorming skill: breadth, mode, time_box parameters
- [ ] **PARAM-07**: test-driven-development skill: strictness, coverage_target, test_style parameters
- [ ] **PARAM-08**: verification-before-completion skill: evidence_bar, auto_run, check_types parameters
- [ ] **PARAM-09**: adversarial-review skill: intensity, min_findings, focus parameters
- [ ] **PARAM-10**: subagent-driven-development skill: worker_model, review_between, parallelism parameters

## v2 Requirements

### Inter-Agent Communication (party mode)

- **PARTY-01**: Agents can send findings to other running agents via shared context
- **PARTY-02**: Mapper findings automatically available to planner without orchestrator relay
- **PARTY-03**: Verification agent can query executor about deviations in real-time

### Self-Tuning Pipeline

- **TUNE-01**: Sequential thinking JSONL logs analyzed for pattern→outcome correlations
- **TUNE-02**: Profile parameters auto-adjusted based on historical branch success rates

## Out of Scope

| Feature | Reason |
|---------|--------|
| Prebuilt binary releases | Infrastructure work, not agent/skill evolution |
| GSD upstream wholesale sync | We're evolving, not tracking (#3) |
| Learning pipeline (scavenger/teacher) | TalentOS, not kinderpowers |
| New MCP servers | Existing 2 are sufficient for v6.2 |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SPAWN-01 | Phase 1 | Complete |
| SPAWN-02 | Phase 1 | Complete |
| SPAWN-03 | Phase 1 | Complete |
| SPAWN-04 | Phase 1 | Complete |
| SPAWN-05 | Phase 1 | Complete |
| COLLAPSE-01 | Phase 2 | Complete |
| COLLAPSE-02 | Phase 2 | Complete |
| COLLAPSE-03 | Phase 2 | Complete |
| COLLAPSE-04 | Phase 2 | Complete |
| COLLAPSE-05 | Phase 2 | Complete |
| COLLAPSE-06 | Phase 2 | Complete |
| BEADS-01 | Phase 3 | Complete |
| BEADS-02 | Phase 3 | Complete |
| BEADS-03 | Phase 3 | Complete |
| BEADS-04 | Phase 3 | Complete |
| BEADS-05 | Phase 3 | Complete |
| BEADS-06 | Phase 3 | Complete |
| PARAM-01 | Phase 4 | Complete |
| PARAM-02 | Phase 4 | Complete |
| PARAM-03 | Phase 4 | Pending |
| PARAM-04 | Phase 4 | Pending |
| PARAM-05 | Phase 5 | Pending |
| PARAM-06 | Phase 5 | Pending |
| PARAM-07 | Phase 5 | Pending |
| PARAM-08 | Phase 5 | Pending |
| PARAM-09 | Phase 5 | Pending |
| PARAM-10 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 27 total
- Mapped to phases: 27
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-19*
