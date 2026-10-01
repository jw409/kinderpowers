---
phase: 05-skill-parameterization
plan: 01
subsystem: skills
tags: [parameterization, systematic-debugging, brainstorming, skill-tuning]

# Dependency graph
requires: []
provides:
  - "systematic-debugging skill with depth/hypothesis_count/reproduce_first parameters"
  - "brainstorming skill with breadth/mode/time_box parameters"
affects: [06-skill-parameterization-2, any caller that invokes these skills with parameters]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Parameter table format: Parameter | Default | Range | Description (consistent with metathinking)"
    - "Parsing hints section converts natural language to parameter values"
    - "Conditional phase behavior documented inline within existing sections"

key-files:
  created: []
  modified:
    - skills/systematic-debugging/SKILL.md
    - skills/brainstorming/SKILL.md

key-decisions:
  - "Parameters section placed immediately after Overview, before existing content — same location as metathinking"
  - "Defaults chosen to preserve exact current behavior (depth=standard, hypothesis_count=3, reproduce_first=true, breadth=3, mode=divergent, time_box=standard)"
  - "Parameter behavior documented inline within existing phase/process sections rather than in a separate section"
  - "Quick Reference table in systematic-debugging gained a Parameter Effects column"

patterns-established:
  - "Skill parameter tables: Parameter | Default | Range | Description"
  - "Parsing hints: natural language phrase -> parameter=value mappings"
  - "Conditional behavior: inline annotations (depth=quick: skip this phase)"

requirements-completed: [PARAM-05, PARAM-06]

# Metrics
duration: 3min
completed: 2026-03-19
---

# Phase 5 Plan 01: Skill Parameterization (systematic-debugging + brainstorming) Summary

**Caller-controlled parameters added to systematic-debugging (depth/hypothesis_count/reproduce_first) and brainstorming (breadth/mode/time_box), making both skills tunable without breaking backward compatibility**

## Performance

- **Duration:** ~3 min
- **Started:** 2026-03-19T22:14:17Z
- **Completed:** 2026-03-19T22:17:03Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- systematic-debugging skill now accepts depth (quick/standard/exhaustive), hypothesis_count (2-8), and reproduce_first (true/false) — all phases updated with conditional behavior
- brainstorming skill now accepts breadth (2-8), mode (divergent/convergent/devil-advocate), and time_box (none/quick/standard) — checklist, process flow, and key principles updated
- Both skills remain fully backward compatible: default parameter values reproduce exact prior behavior

## Task Commits

Each task was committed atomically:

1. **Task 1: Parameterize systematic-debugging skill** - `a219d57` (feat)
2. **Task 2: Parameterize brainstorming skill** - `937015d` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `skills/systematic-debugging/SKILL.md` - Added Parameters section, updated Phase 1-4 with conditional behavior, updated Quick Reference table with Parameter Effects column
- `skills/brainstorming/SKILL.md` - Added Parameters section, updated Checklist/Process Flow/The Process/Key Principles to reference breadth, mode, time_box

## Decisions Made

- Parameters placed immediately after Overview (before When to Use / DESIGN-FIRST block) — consistent with metathinking placement
- Defaults preserve exact current behavior: depth=standard (all 4 phases), hypothesis_count=3 (matches hardcoded "< 3" threshold), reproduce_first=true, breadth=3 (matches hardcoded "2-3 approaches"), mode=divergent, time_box=standard
- Conditional behavior documented inline within existing sections rather than extracting separate parameter-behavior section — keeps context near the relevant phase

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Both parameterized skills ready for callers to use immediately
- Pattern established: parameter table + parsing hints + inline conditional behavior
- Phase 05 plan 02 can follow same pattern for remaining skills

---
*Phase: 05-skill-parameterization*
*Completed: 2026-03-19*

## Self-Check: PASSED

- skills/systematic-debugging/SKILL.md: FOUND
- skills/brainstorming/SKILL.md: FOUND
- .planning/phases/05-skill-parameterization/05-01-SUMMARY.md: FOUND
- Commit a219d57 (systematic-debugging): FOUND
- Commit 937015d (brainstorming): FOUND
