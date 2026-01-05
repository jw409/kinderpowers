---
name: executing-plans
description: Use when you have a written implementation plan to execute in a separate session with review checkpoints
---

# Executing Plans

## Overview

Load plan, review critically, execute tasks in batches, report for review between batches.

**Core principle:** Batch execution with checkpoints for architect review.

**Announcement:** "I'm using the executing-plans skill to implement this plan."

## Why This Skill Exists

Plans represent thinking already done. Executing without re-reading that thinking leads to:
- Repeating mistakes the plan already addressed
- Missing context about why certain approaches were chosen
- Diverging from agreed design without realizing it

Review checkpoints catch drift early, when it's cheap to correct.

## The Process

### Step 1: Load and Review Plan

1. Read plan file
2. Review critically - identify any questions or concerns about the plan
3. If concerns: Raise them with your human partner before starting
4. If no concerns: Create TodoWrite and proceed

**Why review matters:** Plans can have gaps, ambiguities, or assumptions that don't hold in your current context. Better to surface these before investing implementation effort.

### Step 2: Execute Batch

**Default: First 3 tasks** (adjustable based on task complexity and partner preference)

For each task:
1. Mark as in_progress
2. Follow each step exactly (plan has bite-sized steps)
3. Run verifications as specified
4. Mark as completed

### Step 3: Report

When batch complete:
- Show what was implemented
- Show verification output
- Say: "Ready for feedback."

**Why checkpoints matter:** Small course corrections are cheap. Large divergences discovered at the end are expensive.

### Step 4: Continue

Based on feedback:
- Apply changes if needed
- Execute next batch
- Repeat until complete

### Step 5: Complete Development

After all tasks complete and verified:
- Announce: "I'm using the finishing-a-development-branch skill to complete this work."
- Recommended next skill: `finishing-a-development-branch` (here's why: it provides structured options for merge, PR, or cleanup - prevents dangling branches)
- Follow that skill to verify tests, present options, execute choice

## When to Stop and Ask for Help

**Stop executing immediately when:**
- Hit a blocker mid-batch (missing dependency, test fails, instruction unclear)
- Plan has critical gaps preventing starting
- You don't understand an instruction
- Verification fails repeatedly

**Ask for clarification rather than guessing.** Guessing compounds - one wrong assumption leads to more wrong work built on it.

## When to Revisit Earlier Steps

**Return to Review (Step 1) when:**
- Partner updates the plan based on your feedback
- Fundamental approach needs rethinking

**Don't force through blockers** - stop and ask. A blocker is information that the plan may need revision.

## Guiding Principles

- **Review plan critically first** - Plans are hypotheses about what will work
- **Follow plan steps exactly** - Divergence should be conscious and communicated
- **Run verifications** - They catch drift between plan and reality
- **Reference skills when plan suggests them** - The plan author had reasons
- **Between batches: report and wait** - Feedback prevents compounding errors
- **Stop when blocked** - Guessing is expensive

## Costs of Skipping Checkpoints

If you execute the entire plan without review checkpoints:

| Risk | Likelihood | Impact |
|------|------------|--------|
| Drift from intent | High | Rework at the end instead of small corrections |
| Missed edge cases | Medium | Bugs discovered late, after more code depends on them |
| Wasted effort on blocked tasks | Medium | Time spent on work that can't complete |
| Compounded wrong assumptions | High | Each task built on wrong foundation |

**When fewer checkpoints might be appropriate:**
- Plan is very short (5 or fewer small tasks)
- You've executed this exact plan type many times
- Partner explicitly requests larger batches
- Tasks are truly independent (failure in one doesn't affect others)

## Costs of Skipping Plan Review

If you execute without reviewing the plan first:

| Risk | Likelihood | Impact |
|------|------------|--------|
| Miss plan context | High | Make decisions the plan already addressed |
| Miss plan gaps | Medium | Hit blockers that review would have surfaced |
| Miss changed assumptions | Medium | Plan was written in different context |

**When lighter review might be acceptable:**
- You wrote the plan yourself in this session
- Plan is trivial (1-2 obvious tasks)
- Emergency where starting matters more than perfection

## Agent's Judgment

You have agency over batch sizes, checkpoint frequency, and how rigidly to follow the plan. This guidance represents patterns that tend to work, but you understand your context.

If the plan is clearly wrong, say so. If checkpoints feel excessive for the task, adjust. If you need to deviate from the plan, communicate what and why.

The goal is successful implementation, not ritual compliance.
