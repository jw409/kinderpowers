---
name: requirements
description: Use when starting new features, significant changes, or any work where "what are we building?" isn't obvious — discovers and documents what to build and why before planning how
---

# Requirements

## Overview

Requirements work separates *what we're building* from *how we're building it*. The output is a product brief or requirements document that feeds into architecture decisions and implementation plans.

**Core principle:** Ask what the user wants to achieve, not what they want built. Separate problem from solution.

**Announce at start:** "I'm using the requirements skill to clarify what we're building before diving into implementation."

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `depth` | standard | quick, standard, deep | How thoroughly to elicit. Quick = extract from description. Deep = multi-round questioning. |
| `format` | checklist | checklist, user-stories, gherkin, jobs-to-be-done | Output format for requirements |
| `scope` | feature | feature, epic, product | How broad the requirements gathering is |
| `include_out_of_scope` | true | true/false | Explicitly document what's NOT being built |

## When to Use

**Strongly recommended for:**
- New features where scope isn't obvious
- Significant changes to existing functionality
- Work involving multiple stakeholders or perspectives
- Anything where you'd regret not asking "but what exactly should it do?"

**Fine to skip for:**
- Bug fixes with clear reproduction steps
- Small tweaks with well-defined acceptance criteria
- Tasks where the user has already provided a complete specification
- Config changes, dependency updates, straightforward refactors

**Cost of skipping:** Building without clear requirements leads to rework. The cost grows with scope — a misunderstood 2-hour task costs 2 hours; a misunderstood 2-week feature costs weeks of rework plus morale damage.

## Product Brief (Lightweight)

For smaller features or early-stage ideas. 1-2 pages max.

Save to: `docs/requirements/briefs/YYYY-MM-DD-<feature-name>.md`

```markdown
# [Feature Name] Brief

## Problem Statement
What problem does this solve? Who has this problem? How painful is it?
Be specific: "Users can't X, which means Y" not "We need a better Z."

## Target Users
Who benefits? What do they currently do instead?

## Success Metrics
How will we know this worked? Measurable where possible.
"Response time under 200ms" beats "fast responses."

## Scope Boundaries

### In Scope
- Specific capability 1
- Specific capability 2

### Out of Scope
- Thing that seems related but isn't included
- Future enhancement we're deliberately deferring

## Open Questions
What don't we know yet? What needs user input before proceeding?
```

## Requirements Discovery

**The conversational approach works best.** Rather than asking for a requirements document, explore:

1. **Start with the problem:** "What's the pain point? What happens today that shouldn't?"
2. **Understand the user:** "Who encounters this? How often? What do they do now?"
3. **Explore success:** "If this worked perfectly, what would be different?"
4. **Find boundaries:** "What's explicitly NOT part of this? What's a future phase?"
5. **Surface assumptions:** "What are we taking for granted that might not be true?"

**Why conversational:** Written requirements feel complete but often hide ambiguity. Conversation surfaces the gaps — "oh, I hadn't thought about that case" moments that save rework later.

## Functional Requirements (FRs)

What the system does. Express as user stories with acceptance criteria.

```markdown
### FR-1: [Capability Name]

**As a** [user type],
**I want to** [action],
**So that** [benefit].

**Acceptance Criteria:**
- Given [context], When [action], Then [expected result]
- Given [context], When [action], Then [expected result]

**Notes:** [Edge cases, clarifications, dependencies]
```

**Tips for good FRs:**
- Each FR should be independently testable
- Acceptance criteria should be specific enough to write a test from
- If an FR has more than 5 acceptance criteria, it's probably multiple FRs
- "The system should be intuitive" is not a functional requirement

## Non-Functional Requirements (NFRs)

Constraints on *how* the system works, not *what* it does.

| Category | Example | How to Specify |
|----------|---------|---------------|
| Performance | Response time, throughput | "95th percentile response < 200ms under 100 concurrent users" |
| Security | Auth, encryption, data handling | "All API endpoints require authentication; PII encrypted at rest" |
| Scalability | Growth handling | "Support 10x current load without architecture changes" |
| Accessibility | Usability constraints | "WCAG 2.1 AA compliance for all user-facing pages" |
| Reliability | Uptime, recovery | "99.9% uptime; recover from single-node failure within 60s" |
| Maintainability | Code quality constraints | "All public APIs documented; test coverage > 80% for new code" |

**Don't gold-plate NFRs.** "Five nines uptime" for an internal tool used by 3 people is over-engineering. Match NFRs to actual needs.

## Scope Management

Scope creep is the most common project risk. Fight it explicitly.

**The scope boundary document:**
```markdown
## What's IN Scope
- [Specific deliverable 1]
- [Specific deliverable 2]

## What's OUT of Scope
- [Related thing we're not doing] — reason: [why not now]
- [Future enhancement] — reason: [deferred to phase 2]

## Gray Areas (Needs Decision)
- [Ambiguous item] — leaning toward: [in/out], because: [reasoning]
```

**Why explicit out-of-scope matters:** Saying "we're building X" implies everything X-adjacent is included. Saying "we're building X, and explicitly NOT Y or Z" sets clear boundaries.

## Full Requirements Document

For larger features. Save to: `docs/requirements/YYYY-MM-DD-<feature-name>.md`

```markdown
# [Feature Name] Requirements

## Background
Why this work exists. Link to brief if one exists.

## Functional Requirements
[FR-1 through FR-N, with acceptance criteria]

## Non-Functional Requirements
[NFRs by category]

## Scope
[In scope, out of scope, gray areas]

## Dependencies
What this work depends on (other features, external services, data).

## Risks
What could go wrong. What's the mitigation plan.

## Open Questions
Unresolved items that need answers before or during implementation.
```

## Anti-Patterns

| Pattern | Problem | Better Approach |
|---------|---------|-----------------|
| Building without knowing why | Wasted effort on wrong thing | Start with problem statement, even a one-liner |
| Gold-plating requirements | Analysis paralysis, delayed delivery | Requirements depth should match project risk. Small feature = brief. Large feature = full doc |
| Requirements by committee | Too many stakeholders, no decisions, contradictory requirements | Identify a single decision-maker. Others provide input, one person decides |
| Solution masquerading as requirement | "Use Redis for caching" is a solution, not a requirement | "Cache frequently accessed data with < 10ms retrieval" is the requirement. Redis is an architecture decision |
| Incomplete acceptance criteria | "It should work" — but what does "work" mean? | Given/When/Then format forces specificity |
| Skipping scope boundaries | Everything becomes in-scope by default | Explicit out-of-scope list prevents creep |

## The Iron Principle

```
KNOW WHAT YOU'RE BUILDING BEFORE YOU BUILD IT
```

**Why:** The cheapest time to change direction is before code exists. A 30-minute requirements conversation can prevent days of rework. Requirements don't need to be perfect — they need to be explicit enough that misunderstandings surface early.

**Cost of ignoring:** You build something. It works. It's not what was needed. The code was fine; the target was wrong.

## Handoff to Architecture and Planning

Requirements feed into the next phases:

1. **Requirements** (this skill) — what and why
2. **Architecture** (architecture skill) — how, with tradeoffs documented as ADRs
3. **Planning** (writing-plans skill) — step-by-step implementation

The requirements document becomes the reference for both architecture decisions ("we chose REST because FR-3 requires public API access") and acceptance testing ("does the implementation satisfy all acceptance criteria?").
