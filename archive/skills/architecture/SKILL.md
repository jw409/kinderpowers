---
name: architecture
description: Use when facing technical choices that affect multiple files, constrain future decisions, or would be non-obvious to a new contributor — documents decisions as Architecture Decision Records before implementation begins
---

# Architecture

## Overview

Architecture work captures the *why* behind technical choices before implementation begins. The output is Architecture Decision Records (ADRs) and an architecture document that future contributors (including future you) can reference.

**Core principle:** Decisions made implicitly still shape the system — documenting them explicitly makes them reviewable, reversible, and learnable.

**Announce at start:** "I'm using the architecture skill to document technical decisions before implementation."

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `detail_level` | medium | light, medium, full | Light=PR description, medium=short ADR, full=ADR with alternatives analysis |
| `include_alternatives` | true | true/false | Whether to document alternatives considered (skip for obvious choices) |
| `output_format` | adr | adr, inline, architecture_doc | ADR file, inline in PR/commit, or update architecture overview |
| `diagram_style` | text | text, mermaid, none | How to render component diagrams |

## When to Write ADRs

**Strongly recommended when:**
- A decision affects more than one file or module
- The choice constrains future options (database, API style, auth strategy, state management)
- A new contributor would ask "why did you do it this way?"
- You're choosing between two or more reasonable alternatives
- The decision involves a tradeoff (performance vs simplicity, flexibility vs consistency)

**Fine to skip when:**
- The choice is obvious and uncontroversial (naming a variable, choosing a standard library function)
- The decision is trivially reversible (internal function signature, local refactor)
- You're following an established pattern already documented elsewhere

**Cost of skipping:** Undocumented decisions get relitigated. New contributors (or future sessions) re-explore the same options, sometimes choosing differently. A 10-minute ADR saves time when context is lost.

## ADR Format

Save ADRs to: `docs/architecture/decisions/NNNN-<decision-name>.md`

```markdown
# NNNN: [Decision Title]

**Status:** Proposed | Accepted | Deprecated | Superseded by NNNN

**Date:** YYYY-MM-DD

## Context

What situation prompted this decision? What constraints exist?
What forces are at play (technical, business, team)?

## Decision

What was decided. State it directly: "We will use X because Y."

## Consequences

### Benefits
- What improves as a result

### Costs
- What gets harder, slower, or more complex

### Risks
- What could go wrong with this choice

## Alternatives Considered

### Alternative A: [Name]
- **Pros:** ...
- **Cons:** ...
- **Why not:** ...

### Alternative B: [Name]
- **Pros:** ...
- **Cons:** ...
- **Why not:** ...
```

## Architecture Document Structure

For larger systems, maintain an architecture overview at `docs/architecture/ARCHITECTURE.md`:

```markdown
# [Project Name] Architecture

## System Overview
One paragraph: what this system does, who it serves, key constraints.

## Component Diagram
Text-based (Mermaid, ASCII, or structured list). Show major components
and how they communicate.

## Data Flow
How data moves through the system. Input → processing → output.
Note where data is persisted, cached, or transformed.

## Key Decisions
Links to relevant ADRs. Brief summary of each.

## Deployment Model
How this runs in production. Infrastructure, scaling approach, dependencies.
```

**Keep it current:** An outdated architecture doc is worse than none — it actively misleads. Update when ADRs change the picture. If maintaining it becomes burdensome, that's a signal the doc is too detailed.

## Integration with Planning

Architecture precedes implementation plans. The workflow:

1. **Requirements** (what are we building and why)
2. **Architecture** (how will we build it, what tradeoffs are we making)
3. **Planning** (step-by-step implementation referencing architecture decisions)

Plans created with the writing-plans skill should reference ADRs: "Per ADR-0003, we're using REST over GraphQL, so endpoints follow..."

## Anti-Patterns

| Pattern | Problem | Better Approach |
|---------|---------|-----------------|
| Implicit architecture | Decisions live only in code, get relitigated | Write ADRs for non-obvious choices |
| Architecture astronaut | Over-engineering for hypothetical futures | Design for current requirements + one level of foreseen change |
| Premature optimization | Choosing complex solutions before measuring | Start simple, ADR the upgrade path, optimize when evidence demands it |
| Decision by default | Using a technology because it was already there, not because it fits | Document why you're keeping the status quo — that's an ADR too |
| Perfect document syndrome | Delaying implementation until docs are flawless | ADRs can be brief. A paragraph of context + decision + consequences beats a blank page |

## The Iron Principle

```
DOCUMENT THE DECISION, NOT JUST THE OUTCOME
```

**Why:** Code shows *what* was chosen. Documentation shows *why* it was chosen, *what else* was considered, and *what tradeoffs* were accepted. Without the why, future contributors can't evaluate whether the decision still holds.

## Proportional Depth

Match documentation depth to decision impact:

- **Small scope** (affects 1-2 files, easily reversible): A comment in code or PR description suffices
- **Medium scope** (affects a module, somewhat reversible): Short ADR — context, decision, consequences
- **Large scope** (affects system shape, hard to reverse): Full ADR with alternatives analysis + architecture doc update

The goal is informed decisions, not paperwork.
