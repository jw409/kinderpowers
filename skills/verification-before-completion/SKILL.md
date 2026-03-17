---
name: verification-before-completion
description: Use when about to claim work is complete, fixed, or passing, before committing or creating PRs - requires running verification commands and confirming output before making any success claims; evidence before assertions always
---

# Verification Before Completion

## Overview

Run the verification command. Read the output. Then make the claim.

**Core principle:** Evidence before claims.

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

## The Bottom Line

Run the command. Read the output. Then claim the result.

A 30-second verification run is cheaper than debugging a false claim downstream.
