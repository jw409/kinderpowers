---
name: verification-before-completion
description: Use when about to claim work is complete, fixed, or passing, before committing or creating PRs - requires running verification commands and confirming output before making any success claims; evidence before assertions always
---

# Verification Before Completion

## Overview

Claiming work is complete without verification undermines trust and wastes time on rework.

**Core principle:** Evidence before claims, always.

**The spirit matters more than the letter. Here's why:** Verification exists to maintain trust between you and your human partner. Every unverified claim that turns out wrong erodes that trust. The specific commands matter less than the commitment to proving your claims before making them.

## The Iron Principle

```
NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE
```

**Why this is critical:** Unverified claims have historically led to shipped bugs, broken trust, and costly rework. If you haven't run the verification command in this message, claiming it passes is speculation — not fact.

**Cost of skipping:** False completion claims force your human partner to re-verify, debug, and rework. A 30-second verification run prevents hours of debugging.

## The Gate Function

```
BEFORE claiming any status or expressing satisfaction:

1. IDENTIFY: What command proves this claim?
2. RUN: Execute the FULL command (fresh, complete)
3. READ: Full output, check exit code, count failures
4. VERIFY: Does output confirm the claim?
   - If NO: State actual status with evidence
   - If YES: State claim WITH evidence
5. ONLY THEN: Make the claim

Skipping any step means the claim is unverified — state it as such
```

## Common Failures

| Claim | Requires | Not Sufficient |
|-------|----------|----------------|
| Tests pass | Test command output: 0 failures | Previous run, "should pass" |
| Linter clean | Linter output: 0 errors | Partial check, extrapolation |
| Build succeeds | Build command: exit 0 | Linter passing, logs look good |
| Bug fixed | Test original symptom: passes | Code changed, assumed fixed |
| Regression test works | Red-green cycle verified | Test passes once |
| Agent completed | VCS diff shows changes | Agent reports "success" |
| Requirements met | Line-by-line checklist | Tests passing |

## Red Flags — Pause and Verify

- Using "should", "probably", "seems to"
- Expressing satisfaction before verification ("Great!", "Perfect!", "Done!", etc.)
- About to commit/push/PR without verification
- Trusting agent success reports
- Relying on partial verification
- Thinking "just this once"
- Tired and wanting work over
- **Any wording implying success without having run verification**

## Common Rationalization Patterns

Watch for these — they feel reasonable but lead to false claims:

| Shortcut | Why it fails |
|----------|-------------|
| "Should work now" | "Should" is a prediction, not evidence. Run the verification. |
| "I'm confident" | Confidence is not evidence. Verification takes seconds. |
| "Just this once" | Each exception normalizes the next. The cost compounds. |
| "Linter passed" | Linter checks different things than compiler/tests. Each tool verifies different properties. |
| "Agent said success" | Verify independently — agents can report success on partial completion. |
| "I'm tired" | Exhaustion makes verification more important, not less — tired brains make more mistakes. |
| "Partial check is enough" | Partial verification can miss the specific thing that's broken. |
| "Different words so rule doesn't apply" | The principle is about evidence before claims, regardless of phrasing. |

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

## Why This Matters

From 24 failure memories:
- your human partner said "I don't believe you" - trust broken
- Undefined functions shipped - would crash
- Missing requirements shipped - incomplete features
- Time wasted on false completion → redirect → rework
- Maintaining trust requires evidence — unverified claims erode it

## When To Apply

**Strongly recommended before:**
- Any variation of success/completion claims
- Any expression of satisfaction
- Any positive statement about work state
- Committing, PR creation, task completion
- Moving to next task
- Delegating to agents

**Rule applies to:**
- Exact phrases
- Paraphrases and synonyms
- Implications of success
- Any communication suggesting completion/correctness

## The Bottom Line

**Verification is how you maintain trust.**

Run the command. Read the output. THEN claim the result.

This principle is strongly recommended — the cost of skipping is broken trust and wasted rework time.
