---
name: team-orchestration
description: Use when coordinating multiple Claude Code agents or parallel workers — covers team creation, task assignment, worker sizing, file domain separation, and INJECT patterns
---

# Team Orchestration

## Overview

Coordinate multiple agents working in parallel. The key insight: agents are cheap, context switches are expensive. Dispatch right the first time by giving each worker a clear domain, complete context upfront, and exactly one task.

**Announce at start:** "I'm using the team-orchestration skill to plan the parallel work."

## Core Principles

### 1. One Task Per Worker
Workers that juggle multiple tasks produce partial completions. Split ambitious tasks into focused units. Each worker should finish and report within the tool-count budget.

**Skip cost**: Multi-task workers hit the tool-count ceiling, produce partial work, and require expensive rework.

### 2. Non-Overlapping File Domains
Each worker edits different files. Zero merge conflicts. Zero coordination mid-task.

```
Worker A: src/auth/* (auth module)
Worker B: src/billing/* (billing module)
Worker C: tests/* (test files)
# Never: Worker A and B both editing src/shared/utils.ts
```

**Skip cost**: Merge conflicts require manual resolution and negate the time savings of parallelism.

### 3. INJECT Upfront
Workers don't coordinate mid-task. Everything they need goes into the initial prompt:
- Complete task description with WHY
- File paths and relevant code snippets
- API contracts and interfaces they depend on
- Expected output format
- Verification command

**Skip cost**: Workers that need to ask questions block until answered. The whole point of parallelism is eliminated.

### 4. Right Model for the Job

| Task Type | Model | Why |
|-----------|-------|-----|
| Deterministic (template fill, schema write, known fix) | haiku | Fast (~90s), cheap, sufficient with rich INJECT |
| Judgment (refactoring, architecture, preservation decisions) | opus | Reasoning quality matters |
| Discovery (grep, file listing, API exploration) | haiku | Just reading, not deciding |

**Skip cost**: Opus on a template fill wastes money. Haiku on architecture produces shallow work.

## Workflow

### Step 1: Decompose
Break the work into independent units. Ask:
- Can each unit be completed without knowledge of the others?
- Do any units share files? (If yes, merge them or sequence them)
- What context does each unit need?

### Step 2: Create Tasks
Create one task per worker with:
- Clear objective
- File domain (which files to touch)
- Context (code snippets, interfaces, constraints)
- Verification command
- Expected output

### Step 3: Spawn Workers
Launch agents with descriptive names, appropriate model, background execution.

### Step 4: Monitor
Workers report completion via task updates. Check output against verification criteria. If a worker fails, diagnose prompt issue vs genuine blocker.

### Step 5: Integrate
After all workers complete: verify no file conflicts, run full test suite, merge or commit.

## Team Sizing

| Work Scope | Workers | Notes |
|------------|---------|-------|
| 2-3 independent files | 2-3 haiku | Minimal coordination |
| Module-level refactor | 3-5 mixed | Need domain mapping |
| Cross-cutting feature | 5-8 opus | Need interface contracts |
| Full project delivery | GSD agents | Use GSD lifecycle |

**Above 8 workers**: Consider the GSD lifecycle engine (`/gsd:*` commands) which handles phased delivery with built-in coordination.

## Idle State Handling

Workers go idle after each turn — this is normal. An idle worker is waiting for input, not stuck.

- **Idle after message**: Normal — sent their message, waiting for response
- **Idle without output**: Check if blocked on missing file or unclear instruction
- **Multiple idle**: Time to check task list and assign next work

## Anti-Patterns

| Don't | Do |
|-------|-----|
| Give one worker 5 tasks | Split into 5 workers with 1 task each |
| Let workers edit shared files | Map non-overlapping file domains |
| Assume workers will coordinate | INJECT all context upfront |
| Use opus for grep/listing tasks | Use haiku for discovery, opus for judgment |
| Spawn without verification criteria | Include verification command in every task |
| Monitor workers continuously | Let them work, check on notification |

## Integration

- **Precedes**: verification-before-completion (after workers finish)
- **Complements**: dispatching-parallel-agents (for simpler parallel dispatch)
- **Complements**: strategic-planning (for decomposing work)
- **Scales to**: GSD lifecycle engine (for project-scale orchestration)
