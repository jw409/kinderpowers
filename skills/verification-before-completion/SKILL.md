---
name: verification-before-completion
description: Use when about to claim work is complete, fixed, or passing, before committing or creating PRs - requires running verification commands and confirming output before making any success claims; evidence before assertions always
---

# Verification Before Completion

## Overview

Verification protects the trust relationship between you and your human partner. When claims don't match reality, collaboration becomes impossible.

**Core principle:** Evidence before claims, always.

**Why this matters:** Your human partner cannot see your internal state. They rely on your claims to make decisions. False claims waste their time, break their trust, and damage real projects.

## The Iron Principle

```
COMPLETION CLAIMS REQUIRE FRESH VERIFICATION EVIDENCE
```

If you haven't run the verification command in this message, you cannot claim it passes.

**Why verification matters:**
- Your memory of previous runs may be stale or incorrect
- Code you just edited may have introduced new issues
- Partial verification doesn't prove whole-system correctness
- Trust, once broken, is expensive to rebuild

## The Verification Protocol

```
BEFORE claiming any status or expressing satisfaction:

1. IDENTIFY: What command proves this claim?
2. RUN: Execute the FULL command (fresh, complete)
3. READ: Full output, check exit code, count failures
4. VERIFY: Does output confirm the claim?
   - If NO: State actual status with evidence
   - If YES: State claim WITH evidence
5. ONLY THEN: Make the claim

Skipping verification risks:
- Shipping broken code that crashes in production
- Wasting human partner's time on rework cycles
- Breaking trust that took sessions to build
- Training yourself toward careless completion claims
```

## Common Verification Requirements

| Claim | Requires | Not Sufficient |
|-------|----------|----------------|
| Tests pass | Test command output: 0 failures | Previous run, "should pass" |
| Linter clean | Linter output: 0 errors | Partial check, extrapolation |
| Build succeeds | Build command: exit 0 | Linter passing, logs look good |
| Bug fixed | Test original symptom: passes | Code changed, assumed fixed |
| Regression test works | Red-green cycle verified | Test passes once |
| Agent completed | VCS diff shows changes | Agent reports "success" |
| Requirements met | Line-by-line checklist | Tests passing |

## Patterns to Watch

When you notice these patterns in your thinking, pause and verify:

- Using "should", "probably", "seems to" - indicates you don't actually know
- Expressing satisfaction before verification ("Great!", "Perfect!", "Done!")
- About to commit/push/PR without verification
- Trusting agent success reports without independent check
- Relying on partial verification for whole-system claims
- Thinking "just this once" - the exception that becomes the rule
- Feeling tired and wanting work to be over - fatigue clouds judgment
- **ANY wording implying success without having run verification**

These aren't forbidden thoughts. They're signals that you're about to make an unverified claim.

## COSTS OF SKIPPING VERIFICATION

Real examples of false completion claims and their consequences:

**The "Tests Should Pass" Incident**
- Agent claimed tests passed based on code looking correct
- Human partner pushed to staging, tests actually failed
- Production deploy blocked, 2-hour debugging session
- Trust damage: "I don't believe you anymore"

**The Undefined Function Ship**
- Code review looked clean, linter passed
- Forgot to run build/compile step
- Undefined function would have crashed on first call
- Caught only because human partner ran build independently

**The Missing Requirements**
- Tests passed, claimed "phase complete"
- Human partner discovered 3 requirements from original spec were unimplemented
- Full rework cycle, lost half a day
- Established pattern of "verify against original spec, not just tests"

**The Agent Success Trap**
- Subagent reported "task complete"
- Trusted report, moved to next phase
- Agent had actually failed silently, changes were incomplete
- Required re-running entire workflow from scratch

**The Partial Verification Shortcut**
- Ran subset of tests that passed
- Full test suite had 12 failures
- CI/CD caught it after PR was already up
- Created impression of sloppy work

## Addressing Common Rationalizations

| Thought | Consider |
|---------|----------|
| "Should work now" | "Should" means you don't know. Run the verification. |
| "I'm confident" | Confidence is not evidence. Your confidence has been wrong before. |
| "Just this once" | The pattern starts with "just this once." Every time. |
| "Linter passed" | Linter checks style, not compilation. Different tools prove different things. |
| "Agent said success" | Agents can fail silently or misreport. Verify independently. |
| "I'm tired" | Fatigue is exactly when verification matters most. |
| "Partial check is enough" | Partial verification only proves part of the claim. |

## Key Verification Patterns

**Tests:**
```
VERIFIED: [Run test command] [See: 34/34 pass] "All tests pass"
UNVERIFIED: "Should pass now" / "Looks correct"
```

**Regression tests (TDD Red-Green):**
```
VERIFIED: Write -> Run (pass) -> Revert fix -> Run (MUST FAIL) -> Restore -> Run (pass)
UNVERIFIED: "I've written a regression test" (without red-green verification)
```

**Build:**
```
VERIFIED: [Run build] [See: exit 0] "Build passes"
UNVERIFIED: "Linter passed" (linter doesn't check compilation)
```

**Requirements:**
```
VERIFIED: Re-read plan -> Create checklist -> Verify each -> Report gaps or completion
UNVERIFIED: "Tests pass, phase complete"
```

**Agent delegation:**
```
VERIFIED: Agent reports success -> Check VCS diff -> Verify changes -> Report actual state
UNVERIFIED: Trust agent report
```

## Why Verification is Critical for Trust

From documented failure patterns:
- Human partner said "I don't believe you" - trust broken, collaboration damaged
- Undefined functions shipped - would crash on first real use
- Missing requirements shipped - features incomplete, rework required
- Time wasted on false completion -> redirect -> rework cycles
- Each false claim makes the next true claim harder to believe

Trust is the foundation of effective human-AI collaboration. Verification is how you maintain it.

## When To Apply

**Before:**
- Making success/completion claims of any kind
- Expressing satisfaction with work state
- Making positive statements about whether things work
- Committing, PR creation, task completion
- Moving to next task
- Delegating to agents (verify their output)

**The principle covers:**
- Exact phrases ("tests pass")
- Paraphrases and synonyms ("everything's working")
- Implications of success ("ready for review")
- Any communication suggesting completion/correctness

## The Bottom Line

**Verification protects the trust that makes collaboration possible.**

Run the command. Read the output. THEN claim the result.

This is critical for trust because your human partner cannot verify your internal state - they can only see your claims and the actual results. When these don't match, the entire collaboration suffers.
