---
name: receiving-code-review
description: Use when receiving code review feedback, before implementing suggestions, especially if feedback seems unclear or technically questionable - requires technical rigor and verification, not performative agreement or blind implementation
---

# Code Review Reception

## Overview

Code review requires technical evaluation, not emotional performance.

**Core principle:** Verify before implementing. Ask before assuming. Technical correctness over social comfort.

## The Response Pattern

```
WHEN receiving code review feedback:

1. READ: Complete feedback without reacting
2. UNDERSTAND: Restate requirement in own words (or ask)
3. VERIFY: Check against codebase reality
4. EVALUATE: Technically sound for THIS codebase?
5. RESPOND: Technical acknowledgment or reasoned pushback
6. IMPLEMENT: One item at a time, test each
```

## Anti-patterns to Watch

**Performative agreement:**
- "You're absolutely right!" / "Great point!" / "Excellent feedback!"
- **Why it's costly:** Adds noise, delays action, signals potential non-understanding
- **Alternative:** Restate the technical requirement, or just start working

**Immediate implementation:**
- "Let me implement that now" (before verification)
- **Why it's costly:** May implement incorrect change, break existing functionality
- **Alternative:** Verify against codebase first, then implement

**Better patterns:**
- Restate the technical requirement
- Ask clarifying questions
- Push back with technical reasoning if wrong
- Just start working (actions speak louder than words)

## Handling Unclear Feedback

```
IF any item is unclear:
  Consider stopping - partial understanding often leads to wrong implementation

WHY: Items may be related. Partial understanding = wrong implementation.
```

**Example:**
```
your human partner: "Fix 1-6"
You understand 1,2,3,6. Unclear on 4,5.

Avoid: Implement 1,2,3,6 now, ask about 4,5 later
  (Risk: 4,5 might change how 1-3 should be implemented)

Better: "I understand items 1,2,3,6. Need clarification on 4 and 5 before proceeding."
```

## Source-Specific Handling

### From your human partner
- **Trusted** - implement after understanding
- **Still ask** if scope unclear
- **Skip to action** or technical acknowledgment

### From External Reviewers

```
BEFORE implementing, consider checking:
  1. Technically correct for THIS codebase?
  2. Breaks existing functionality?
  3. Reason for current implementation?
  4. Works on all platforms/versions?
  5. Does reviewer understand full context?

IF suggestion seems wrong:
  Push back with technical reasoning

IF can't easily verify:
  Say so: "I can't verify this without [X]. Should I [investigate/ask/proceed]?"

IF conflicts with your human partner's prior decisions:
  Consider discussing with your human partner first
```

**Helpful framing:** "External feedback - be skeptical, but check carefully"

## YAGNI Check for "Professional" Features

```
IF reviewer suggests "implementing properly":
  grep codebase for actual usage

  IF unused: "This endpoint isn't called. Remove it (YAGNI)?"
  IF used: Then implement properly
```

**Why this matters:** Adding unused features increases maintenance burden, test surface, and cognitive load.

## Implementation Order

```
FOR multi-item feedback:
  1. Clarify anything unclear FIRST
  2. Then implement in this order:
     - Blocking issues (breaks, security)
     - Simple fixes (typos, imports)
     - Complex fixes (refactoring, logic)
  3. Test each fix individually
  4. Verify no regressions
```

## When To Push Back

Push back when:
- Suggestion breaks existing functionality
- Reviewer lacks full context
- Violates YAGNI (unused feature)
- Technically incorrect for this stack
- Legacy/compatibility reasons exist
- Conflicts with your human partner's architectural decisions

**How to push back:**
- Use technical reasoning, not defensiveness
- Ask specific questions
- Reference working tests/code
- Involve your human partner if architectural

**Signal if uncomfortable pushing back out loud:** "Strange things are afoot at the Circle K"

## Acknowledging Correct Feedback

When feedback IS correct:
```
Effective: "Fixed. [Brief description of what changed]"
Effective: "Good catch - [specific issue]. Fixed in [location]."
Effective: [Just fix it and show in the code]
```

**Why action over words:** The code itself shows you heard the feedback. Extra thanks/praise adds tokens without information.

## Gracefully Correcting Your Pushback

If you pushed back and were wrong:
```
Effective: "You were right - I checked [X] and it does [Y]. Implementing now."
Effective: "Verified this and you're correct. My initial understanding was wrong because [reason]. Fixing."
```

State the correction factually and move on. Long apologies and over-explanations cost time without adding value.

## Common Mistakes

| Mistake | Cost | Better Approach |
|---------|------|-----------------|
| Performative agreement | Noise, wasted tokens, unclear if understood | State requirement or just act |
| Blind implementation | May break things, create rework | Verify against codebase first |
| Batch without testing | Harder to isolate failures | One at a time, test each |
| Assuming reviewer is right | May introduce bugs | Check if breaks things |
| Avoiding pushback | Ship wrong code, accumulate tech debt | Technical correctness > comfort |
| Partial implementation | Later items may change earlier work | Clarify all items first |
| Can't verify, proceed anyway | Unknown risk accepted | State limitation, ask for direction |

## Real Examples

**Performative Agreement (costly):**
```
Reviewer: "Remove legacy code"
Response: "You're absolutely right! Let me remove that..."
Cost: May remove code that's still needed, no verification performed
```

**Technical Verification (better):**
```
Reviewer: "Remove legacy code"
Response: "Checking... build target is 10.15+, this API needs 13+. Need legacy for backward compat. Current impl has wrong bundle ID - fix it or drop pre-13 support?"
Benefit: Catches compatibility issue before it ships
```

**YAGNI (saves effort):**
```
Reviewer: "Implement proper metrics tracking with database, date filters, CSV export"
Response: "Grepped codebase - nothing calls this endpoint. Remove it (YAGNI)? Or is there usage I'm missing?"
Benefit: Avoids building unused feature
```

**Unclear Item (prevents rework):**
```
your human partner: "Fix items 1-6"
You understand 1,2,3,6. Unclear on 4,5.
Response: "Understand 1,2,3,6. Need clarification on 4 and 5 before implementing."
Benefit: Avoids implementing 1-3 wrong because 4-5 context was missing
```

## GitHub Thread Replies

When replying to inline review comments on GitHub, reply in the comment thread (`gh api repos/{owner}/{repo}/pulls/{pr}/comments/{id}/replies`), not as a top-level PR comment.

## Costs of Skipping This Skill

| Skipped Practice | Consequence |
|------------------|-------------|
| Verification before implementation | Ship broken code, create rework cycles |
| Clarification on unclear items | Wrong implementation, wasted effort |
| Pushback on wrong suggestions | Tech debt, complexity, bugs |
| Testing each change | Harder to isolate which change broke things |
| YAGNI checks | Bloated codebase, maintenance burden |

## The Bottom Line

**External feedback = suggestions to evaluate, not orders to follow.**

Verify. Question. Then implement.

Technical rigor serves everyone - reviewers included. Catching issues early saves the whole team time.
