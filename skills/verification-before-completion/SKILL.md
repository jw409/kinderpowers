---
name: verification-before-completion
description: Use when about to claim work is complete, fixed, or passing, before committing or creating PRs - requires running verification commands and confirming output before making any success claims; evidence before assertions always
---

# Verification Before Completion

## Overview

Run the verification command. Read the output. Then make the claim.

**Core principle:** Evidence before claims.

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `evidence_types` | all | all, tests, build, requirements, agent | Which verification categories to check — all, just tests, just build, requirements checklist, or agent delegation verification |
| `min_checks` | 3 | 1-10 | Minimum number of verification checks before claiming completion |
| `auto_run` | true | true/false | Whether to automatically run verification commands or just list what should be checked |
| `deep_inspection` | auto | auto, always, never | Whether to apply deep inspection protocol (count/sample/check) on output — auto=for non-trivial output |

## The Gate

```
BEFORE claiming any status:

1. IDENTIFY: What command proves this claim?
2. RUN: Execute the command (fresh, complete)
3. READ: Full output, check exit code, count failures
4. VERIFY: Does output confirm the claim?
   - If NO: State actual status with evidence
   - If YES: State claim WITH evidence
5. ONLY THEN: Make the claim

Skipping any step means the claim is unverified — state it as such
```

## What Counts as Evidence

| Claim | Requires | Not Sufficient |
|-------|----------|----------------|
| Tests pass | Test command output: 0 failures | Previous run, "should pass" |
| Linter clean | Linter output: 0 errors | Partial check, extrapolation |
| Build succeeds | Build command: exit 0 | Linter passing, logs look good |
| Bug fixed | Test original symptom: passes | Code changed, assumed fixed |
| Regression test works | Red-green cycle verified | Test passes once |
| Agent completed | VCS diff shows changes | Agent reports "success" |
| Requirements met | Line-by-line checklist | Tests passing |

## Patterns to Watch For

If you notice yourself about to:
- Use "should", "probably", "seems to"
- Express satisfaction before running verification ("Great!", "Done!")
- Commit/push/PR without fresh verification
- Trust an agent's success report without checking the diff
- Rely on partial verification

...pause and run the command first.

## Key Patterns

**Tests:**
```
GOOD: [Run test command] [See: 34/34 pass] "All tests pass"
AVOID: "Should pass now" / "Looks correct"
```

**Regression tests (TDD Red-Green):**
```
GOOD: Write → Run (pass) → Revert fix → Run (fails) → Restore → Run (pass)
AVOID: "I've written a regression test" (without red-green verification)
```

**Build:**
```
GOOD: [Run build] [See: exit 0] "Build passes"
AVOID: "Linter passed" (linter doesn't check compilation)
```

**Requirements:**
```
GOOD: Re-read plan → Create checklist → Verify each → Report gaps or completion
AVOID: "Tests pass, phase complete"
```

**Agent delegation:**
```
GOOD: Agent reports success → Check VCS diff → Verify changes → Report actual state
AVOID: Trust agent report without independent verification
```

## When To Apply

**Before:**
- Any variation of success/completion claims
- Committing, PR creation, task completion
- Moving to next task
- Delegating to agents

## Verification Checklist

Before claiming any work is complete, run through this checklist:

### Code Changes
- [ ] Tests pass (ran the actual test command, read the output)
- [ ] Build succeeds (ran the build, checked exit code)
- [ ] Linter clean (if applicable -- linter passing != build passing)
- [ ] No regressions (full test suite, not just new tests)

### Bug Fixes
- [ ] Original symptom verified fixed (test the actual bug scenario)
- [ ] Regression test added (red-green cycle: write test, see it fail on old code, see it pass on new code)
- [ ] Related edge cases checked

### Feature Work
- [ ] Requirements met (re-read plan, line-by-line checklist)
- [ ] Tests cover the new behavior
- [ ] No unfinished TODO/FIXME left in new code

### Agent Delegation
- [ ] Agent reports success -- but verify independently
- [ ] Check the actual diff (VCS diff shows expected changes)
- [ ] Run tests yourself (don't trust the agent's test output claim)

### Deep Inspection Protocol

When verifying non-trivial output (logs, data files, test results):

1. **Count total first**: How many lines/items/tests?
2. **Sample beginning/middle/end**: Don't just read the first 20 lines
3. **Check for contradictions**: If there are both successes and failures, investigate the failures
4. **If you truncated, say so**: "Sampled first 20 of 847 lines. Full analysis requires..."

The anti-pattern: `| head -20` then "I've analyzed the output." You haven't. You've sampled.

## The Bottom Line

Run the command. Read the output. Then claim the result.

A 30-second verification run is cheaper than debugging a false claim downstream.
