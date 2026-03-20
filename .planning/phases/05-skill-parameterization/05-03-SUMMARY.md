---
phase: 05-skill-parameterization
plan: 03
subsystem: skills
tags: [parameterization, adversarial-review, subagent-driven-development, caller-controls]
dependency_graph:
  requires: []
  provides: [parameterized-adversarial-review, parameterized-subagent-driven-development]
  affects: [agents/multi-perspective-review.md]
tech_stack:
  added: []
  patterns: [parameter-table, natural-language-parsing, conditional-behavior]
key_files:
  created: []
  modified:
    - skills/adversarial-review/SKILL.md
    - skills/subagent-driven-development/SKILL.md
decisions:
  - "intensity parameter maps to existing Pedanticness Slider levels: gentle=Low, standard=Medium, hostile=High"
  - "focus=all preserves full review protocol; specific focus deep-dives that category with 80/20 split"
  - "parallelism=aggressive dispatches all non-overlapping tasks simultaneously with file domain analysis gate"
  - "review_between=false defers all quality gates to final review — noted as faster but riskier"
metrics:
  duration: 3min
  completed: 2026-03-20T01:17:21Z
  tasks_completed: 2
  files_modified: 2
---

# Phase 5 Plan 03: Adversarial Review and Subagent-Driven Development Parameterization Summary

Parameterized the final two skills: adversarial-review gains intensity/min_findings/focus controls, and subagent-driven-development gains worker_model/review_between/parallelism controls.

## What Was Built

### Task 1: Parameterize adversarial-review skill (commit: 6a6cd60)

Added `## Parameters (caller controls)` section immediately after Overview with three parameters:

- **intensity** (gentle/standard/hostile): Controls review aggression. Maps to the existing Pedanticness Slider (Low/Medium/High) and proportional review levels. hostile always triggers Full adversarial regardless of change size.
- **min_findings** (0-10): Minimum findings floor. When set and findings fall short, triggers re-analysis with stronger prompting — not manufacturing issues, ensuring thoroughness.
- **focus** (all/security/correctness/completeness/performance): When != all, concentrates 80% of review time on that category, still noting blocking issues in others.

Updated body sections: Step 3 references focus, Step 4 references intensity with per-level behavior, Step 5 references min_findings floor logic, Proportional Review maps intensity to review levels, Pedanticness Slider cross-references intensity.

### Task 2: Parameterize subagent-driven-development skill (commit: 218fd16)

Added `## Parameters (caller controls)` section immediately after the overview paragraph with three parameters:

- **worker_model** (haiku/sonnet/opus): Implementation subagent model. Can be overridden per-task for unusual complexity.
- **review_between** (true/false): When false, skips per-task spec+quality reviews and runs only final review. Faster but risks compounding issues.
- **parallelism** (sequential/conservative/aggressive): conservative dispatches up to 2 non-overlapping tasks simultaneously; aggressive dispatches all non-overlapping tasks simultaneously with conflict detection.

Updated dot graph to reference `{worker_model}` in dispatch nodes. Added `Parameter-Conditional Behavior` section explaining each parameter's effect on process flow. Updated Watch For to address file domain analysis requirement for non-sequential parallelism. Added parallelism cost note to Advantages/Cost section.

## Deviations from Plan

None — plan executed exactly as written.

## Self-Check

- [x] skills/adversarial-review/SKILL.md exists and contains "Parameters (caller controls)"
- [x] skills/subagent-driven-development/SKILL.md exists and contains "Parameters (caller controls)"
- [x] Commit 6a6cd60 exists (adversarial-review)
- [x] Commit 218fd16 exists (subagent-driven-development)
- [x] YAML frontmatter unchanged in both files
- [x] Defaults (standard/0/all and sonnet/true/sequential) preserve current behavior

## Self-Check: PASSED
