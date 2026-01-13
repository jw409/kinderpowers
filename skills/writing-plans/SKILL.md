---
name: writing-plans
description: Use when you have a spec or requirements for a multi-step task, before touching code
---

# Writing Plans

## Overview

Write comprehensive implementation plans assuming the engineer has zero context for the codebase and may need guidance on best practices. Document everything they need: which files to touch, code patterns, testing approach, relevant docs. Give them the whole plan as bite-sized tasks.

Assume they are a skilled developer but unfamiliar with this specific toolset or problem domain.

**Announce at start:** "I'm using the writing-plans skill to create the implementation plan."

**Context:** This is highly valuable when run in a dedicated worktree (created by using-git-worktrees skill). Running in a worktree provides isolation and prevents polluting the main workspace with planning artifacts.

**Save plans to:** `docs/plans/YYYY-MM-DD-<feature-name>.md`

## Bite-Sized Task Granularity

**Each step is one action (2-5 minutes):**
- "Write the failing test" - step
- "Run it to make sure it fails" - step
- "Implement the minimal code to make the test pass" - step
- "Run the tests and make sure they pass" - step
- "Commit" - step

**Why this granularity matters:** Smaller steps are easier to verify, easier to recover from if something goes wrong, and provide natural checkpoints for review. Larger steps increase the risk of compound errors.

## Plan Document Header

**Every plan benefits from this header structure:**

```markdown
# [Feature Name] Implementation Plan

> **For Claude:** Recommended next skill: Use superpowers:executing-plans to implement this plan task-by-task. This provides structure for systematic execution with review checkpoints.

**Goal:** [One sentence describing what this builds]

**Architecture:** [2-3 sentences about approach]

**Tech Stack:** [Key technologies/libraries]

---
```

## Task Structure

```markdown
### Task N: [Component Name]

**Files:**
- Create: `exact/path/to/file.py`
- Modify: `exact/path/to/existing.py:123-145`
- Test: `tests/exact/path/to/test.py`

**Step 1: Write the failing test**

```python
def test_specific_behavior():
    result = function(input)
    assert result == expected
```

**Step 2: Run test to verify it fails**

Run: `pytest tests/path/test.py::test_name -v`
Expected: FAIL with "function not defined"

**Step 3: Write minimal implementation**

```python
def function(input):
    return expected
```

**Step 4: Run test to verify it passes**

Run: `pytest tests/path/test.py::test_name -v`
Expected: PASS

**Step 5: Commit**

```bash
git add tests/path/test.py src/path/file.py
git commit -m "feat: add specific feature"
```
```

## Why These Practices Matter

### Exact file paths always
- **Value:** Eliminates ambiguity, reduces errors
- **Cost of skipping:** Time wasted searching for files, wrong file modified

### Complete code in plan (not "add validation")
- **Value:** Removes interpretation, copy-paste ready
- **Cost of skipping:** Implementation drift from intent, debugging time

### Exact commands with expected output
- **Value:** Verifiable steps, clear success criteria
- **Cost of skipping:** Uncertainty about whether step succeeded

### TDD approach (test-first)
- **Value:** Tests define behavior, catch regressions, force design thinking
- **Cost of skipping:** Tests written after often test implementation rather than behavior

### Frequent commits
- **Value:** Recovery points, clear history, easier code review
- **Cost of skipping:** Large diffs, harder to isolate issues

## Execution Handoff

After saving the plan:

**"Plan complete and saved to `docs/plans/<filename>.md`. Ready to execute with subagent-driven development."**

- **Recommended next skill:** superpowers:subagent-driven-development (provides structured subagent dispatch with review cycles)
- Stay in this session
- Fresh subagent per task + code review

## Exceptions and When to Deviate

### When exact paths aren't known
If you're exploring a new codebase, it's acceptable to use patterns like `src/**/*handler*.py` with a note to verify the actual path before implementation.

### When TDD isn't appropriate
Some changes (configuration, documentation, migrations) may not benefit from test-first approach. Document why tests aren't included for these steps.

### When commits should be batched
If changes are tightly coupled and only make sense together, group them into a single commit. Document why the batching was chosen.

### When code can't be complete in plan
For complex implementations depending on runtime discovery, provide the structure and key decision points rather than complete code.

## Common Mistakes

### Vague file references
- **Problem:** "Update the handler file"
- **Cost:** Time searching, potential wrong file
- **Fix:** Exact paths always

### Implementation-detail steps
- **Problem:** Steps that describe HOW rather than WHAT
- **Cost:** Brittle plans that break with minor changes
- **Fix:** Focus on outcomes, not mechanics

### Missing expected outputs
- **Problem:** "Run the tests"
- **Cost:** Unclear success criteria
- **Fix:** Include expected output or success indicators

### Monolithic tasks
- **Problem:** Single task spanning 30+ minutes of work
- **Cost:** No checkpoints, hard to recover from errors
- **Fix:** Break into 2-5 minute steps
