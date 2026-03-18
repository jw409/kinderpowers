---
name: executing-plans
description: Use when you have a written implementation plan to execute in a separate session with review checkpoints
---

# Executing Plans

## Overview

Load plan, review critically, execute tasks in batches, report for review between batches.

**Core principle:** Batch execution with checkpoints for architect review.

**Announce at start:** "I'm using the executing-plans skill to implement this plan."

## The Process

### Step 1: Load and Review Plan
1. Read plan file
2. Review critically - identify any questions or concerns about the plan
3. If concerns: Raise them with your human partner before starting
4. If no concerns: Create TodoWrite and proceed

### Step 2: Execute Batch
**Default: First 3 tasks**

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

### Step 4: Continue
Based on feedback:
- Apply changes if needed
- Execute next batch
- Repeat until complete

### Step 5: Complete Development

After all tasks complete and verified:
- Announce: "I'm using the finishing-a-development-branch skill to complete this work."
- **REQUIRED SUB-SKILL:** Use kinderpowers:finishing-a-development-branch
- Follow that skill to verify tests, present options, execute choice

## When to Stop and Ask for Help

**Pause execution when:**
- Hit a blocker mid-batch (missing dependency, test fails, instruction unclear)
- Plan has critical gaps preventing starting
- You don't understand an instruction
- Verification fails repeatedly

**Ask for clarification rather than guessing.**

## When to Revisit Earlier Steps

**Return to Review (Step 1) when:**
- Partner updates the plan based on your feedback
- Fundamental approach needs rethinking

**Don't force through blockers** - stop and ask.

## Just Do It When Clear

Not every task needs ceremony. If the plan step is explicit and unambiguous:
- Don't re-plan what's already planned
- Don't ask "should I brainstorm approaches?" when the approach is specified
- Don't create sub-plans for individual steps

**Execute when**: Plan provides file paths + exact changes, or describes a precise command sequence.
**Pause when**: Plan step is vague, open-ended, or you genuinely don't understand it.

**Default bias**: If the plan step is clear, just do it.

## Work Item Claim Protocol

Before executing, check if a tracking system is in use:

### Before Execution
1. Check for relevant work items (issues, beads, tickets)
2. If one exists: verify status, claim it (set in-progress + assignee)
3. If someone else claimed it: ask before proceeding (soft enforcement)
4. If no relevant work item exists: ask whether to create one or treat as ad-hoc

### During Execution
- If scope grows beyond the plan step: note the expansion
- If blocked: record the blocker, don't silently skip

### After Execution
- Close the work item with a summary
- If partial: note what's done and what remains

## Verify Before Assuming

Never assume services, ports, or capabilities work without explicit verification.

**Apply when**:
- Referencing any port/service (APIs, databases, LLM endpoints)
- Claiming infrastructure capability
- Describing what a subprocess or worker does

**Protocol**: Check that it's actually running/working before building on the assumption. A 5-second health check prevents 30 minutes of debugging phantom failures.

## Parallel Execution Patterns

| Pattern | Flow |
|---------|------|
| Multi-file edit | Read all files (parallel) -> Edit -> Test -> Verify |
| Fix failures | Run tests -> Fix each -> Re-test -> Repeat until green |
| Execute plan | Read plan -> Load context (parallel) -> Execute phases -> Verify |
| Independent tasks | Dispatch to separate agents -> Collect results -> Integrate |

When tasks within a batch are independent, run them in parallel. Sequential execution of independent tasks wastes time.

## Remember
- Review plan critically first
- Follow plan steps exactly
- Don't skip verifications
- Reference skills when plan says to
- Between batches: just report and wait
- Stop when blocked, don't guess
- Use a branch, not main -- accidental commits to main are hard to untangle
- Just do it when the step is clear (no re-planning)
- Claim work items before starting
- Verify assumptions before building on them
- Parallelize independent operations

## Integration

**Required workflow skills:**
- **kinderpowers:using-git-worktrees** - REQUIRED: Set up isolated workspace before starting
- **kinderpowers:writing-plans** - Creates the plan this skill executes
- **kinderpowers:finishing-a-development-branch** - Complete development after all tasks
