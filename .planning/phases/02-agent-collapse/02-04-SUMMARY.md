---
phase: 02-agent-collapse
plan: 04
subsystem: workflows
tags: [agent-collapse, workflow-update, parameterized-agents, consumer-migration]

# Dependency graph
requires:
  - phase: 02-agent-collapse/01
    provides: "Parameterized gsd-researcher and gsd-verifier agents"
  - phase: 02-agent-collapse/02
    provides: "Parameterized gsd-ui and gsd-planner (scope) agents"
provides:
  - "All workflow files reference new parameterized agent names"
  - "model-profiles.cjs maps both new and legacy agent entries"
  - "Mode/scope context injected into all spawn prompts"
affects: [05-cleanup-validation]

# Tech tracking
tech-stack:
  added: []
  patterns: [mode-context-in-spawn-prompts, legacy-alias-preservation]

key-files:
  created: []
  modified:
    - gsd/bin/lib/model-profiles.cjs
    - gsd/workflows/plan-phase.md
    - gsd/workflows/research-phase.md
    - gsd/workflows/new-project.md
    - gsd/workflows/new-milestone.md
    - gsd/workflows/execute-phase.md
    - gsd/workflows/verify-work.md
    - gsd/workflows/validate-phase.md
    - gsd/workflows/ui-phase.md
    - gsd/workflows/ui-review.md
    - gsd/workflows/audit-milestone.md
    - gsd/workflows/quick.md
    - gsd/workflows/discuss-phase.md
    - commands/gsd/plan-phase.md
    - commands/gsd/research-phase.md
    - commands/gsd/ui-phase.md

key-decisions:
  - "Added 'Your mode is: X' or 'Your scope is: X' as first line of spawn prompts to communicate parameterized mode"
  - "Preserved all old agent entries in model-profiles.cjs as legacy aliases with comment block"
  - "gsd/get-shit-done/ directory left untouched per STATE.md concern"

requirements-completed: [COLLAPSE-06]

# Metrics
duration: 14min
completed: 2026-03-20
---

# Phase 02 Plan 04: Workflow Consumer Migration Summary

**Updated 16 files across workflows/, commands/, and bin/lib/ to reference new parameterized agent names with mode/scope context in spawn prompts**

## Performance

- **Duration:** 14 min
- **Started:** 2026-03-20T00:43:14Z
- **Completed:** 2026-03-20T00:57:14Z
- **Tasks:** 2
- **Files modified:** 16

## Accomplishments
- Added gsd-researcher and gsd-ui entries to model-profiles.cjs alongside legacy alias entries
- Replaced 10 old agent name patterns across 15 workflow/command files with 4 parameterized agents
- Added mode/scope context to every spawn prompt so parameterized agents know which behavior to activate
- Zero old subagent_type references remain in gsd/workflows/ and commands/gsd/

## Task Commits

Each task was committed atomically:

1. **Task 1: Update model-profiles.cjs with new agent entries** - `da9c691` (feat)
2. **Task 2: Update workflow files to use new agent names** - `e75e3ce` (feat)

## Files Created/Modified
- `gsd/bin/lib/model-profiles.cjs` - Added gsd-researcher and gsd-ui profiles, reorganized with legacy alias section
- 12 workflow files in `gsd/workflows/` - All subagent_type and resolve-model calls updated
- 3 command files in `commands/gsd/` - Agent name references in descriptions and spawn calls updated

## Agent Name Substitutions Applied

| Old Name | New Name | Mode/Scope | Files Updated |
|----------|----------|------------|---------------|
| gsd-phase-researcher | gsd-researcher | mode=phase | 6 |
| gsd-project-researcher | gsd-researcher | mode=project | 2 |
| gsd-research-synthesizer | gsd-researcher | mode=synthesize | 2 |
| gsd-plan-checker | gsd-verifier | mode=plan-quality | 4 |
| gsd-integration-checker | gsd-verifier | mode=integration | 1 |
| gsd-nyquist-auditor | gsd-verifier | mode=coverage | 1 |
| gsd-ui-researcher | gsd-ui | mode=spec | 1 |
| gsd-ui-checker | gsd-ui | mode=validate | 1 |
| gsd-ui-auditor | gsd-ui | mode=audit | 1 |
| gsd-roadmapper | gsd-planner | scope=project | 2 |

## Decisions Made
- Mode context added as first line of spawn prompts: "Your mode is: X" or "Your scope is: X"
- Legacy aliases preserved in model-profiles.cjs with comment block for backward compatibility
- gsd/get-shit-done/ directory intentionally untouched (upstream copy concern from STATE.md)

## Deviations from Plan
None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All consumers now reference parameterized agents
- Old alias .md files (from Plan 03) provide backward compatibility for any external references
- Original source agent .md files can be removed in cleanup phase (Phase 05)

---
*Phase: 02-agent-collapse*
*Completed: 2026-03-20*
