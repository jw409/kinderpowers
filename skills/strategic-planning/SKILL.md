---
name: strategic-planning
description: Use when work needs design, planning, or discovery before execution — creates investigative briefs that subagents can execute intelligently
---

# Strategic Planning

## Overview

Plans for subagents are **investigative briefs**, not sed-scripts. You're briefing an investigator, not programming a robot. Give them the problem, what you've learned, what's unknown, and the mission. Trust the executor to figure out HOW.

**Announce at start:** "I'm using the strategic-planning skill to design the approach."

## When to Use

- Request needs design work or clarification
- Complex feature requiring phases
- Infrastructure debugging with unclear root cause
- Multiple approaches need consideration
- User says "write a plan" or "show me options"

## When NOT to Use

- Explicit file/function instructions with clear intent — just execute
- Simple one-line fixes
- User has already provided a detailed plan

## The Core Principle

The single most important thing in a plan is capturing **what you discovered in this session** so the executor doesn't repeat your investigation. Every plan should transfer context, not prescribe commands.

## Planning Workflow

### 1. Clarity Check

Would a colleague understand this request? If not, ask ONE targeted question.

### 2. Detect Mode

| Mode | Use When | Focus |
|------|----------|-------|
| **Investigative** | Debugging, unclear root cause, infrastructure broken | Hypotheses, what was tried, what works vs broken |
| **Implementation** | Features, refactoring, greenfield | Phases, approach, success criteria |

### 3. Investigate Before Planning

**Before writing any plan**, gather evidence:

1. Search for existing implementations (prevent duplication)
2. Check issue trackers for prior work
3. Run commands to understand current system state
4. Document what works and what's broken — with evidence

Include discovery results in the plan. "Found existing X, will extend" or "No existing solution found, creating new."

### 4. Write the Plan

#### Investigative Mode Template

```markdown
# [End Goal, Not Just Fix Name]

> For Claude: This is a debug-first situation. Orient yourself to the
> problem, understand system state, fix what's broken, then build.

## The Goal
[What we're actually building, with vision — not "fix the logs"]

## Current System State ([date] Session Discovery)

### What We Tried
[Exact commands from THIS session, with outputs]

### The Problem
[What failed, with evidence — logs, outputs, error messages]

### What We Checked
[Everything investigated, numbered, with results]

### What Works vs What's Broken
✅ WORKS: [List with evidence]
❌ BROKEN: [List with evidence]
❓ UNKNOWN: [Hypotheses to test]

## Related Context
[Issues, prior work, architectural decisions that inform this]

## Your Mission

**Phase 1: [Fix Foundation]**
[High-level goals — what they should understand, not what lines to type]

**Phase 2: [Build on Working Foundation]**
[Next layer, only reachable after Phase 1]

## How to Approach This

**DON'T:**
- Just run the commands listed above and add lines suggested
- Assume the fix is known (it isn't — that's why this is investigative)
- Fix symptoms without understanding root cause

**DO:**
- Read the code to understand what SHOULD happen
- Add instrumentation to see what ACTUALLY happens
- Test with minimal examples first
- Verify each fix before moving to next phase

**Key Files:**
- `path/to/main.ts:123-456` — [WHY this file matters, not just its name]
- `path/to/service.ts:789` — [What to look for here]

## Success Criteria
- [ ] [Outcome 1 — observable, testable]
- [ ] [Outcome 2]
- [ ] Root cause documented
```

#### Implementation Mode Template

```markdown
# [Feature Name]

## Objective
[What we're building + Why it matters]

## Context
[Current state, constraints, and WHY those constraints exist]

## Discovery
- Searched: [what you searched for]
- Found: [existing tools/files/patterns]
- Decision: [extend X / no existing solution, creating new]

## Approach

**Phase 1: [Component/Concern]**
Goal: [outcome, not steps]
Key files: [with context on why]
Depends on: nothing / Phase N

**Phase 2: [Component/Concern]**
Goal: [outcome]
Key files: [with context]
Depends on: Phase 1

## Rejected Alternatives
- [Approach A] — rejected because [concrete reason]

## Success Criteria
- [ ] [Outcome — observable, testable]
```

### 5. Anti-Patterns — Bad vs Good

**Bad: "Fix the logs" as the goal**
- Bad: "Add logging to LanceDBService"
- Good: "Make GitHub issue indexing actually work (logs are just instrumentation to debug it)"

**Bad: Assuming you know the fix**
- Bad: "The problem is line 387 doesn't log"
- Good: "Embeddings aren't generated OR aren't written OR wrong format (unknown — investigate)"

**Bad: Listing files without context**
- Bad: "Files: LanceDBService.ts, EmbeddingFunction.ts"
- Good: "LanceDBService.ts:372-423 — addDocuments() claims success, check if embeddings are actually generated"

**Bad: Prescriptive steps when root cause unknown**
- Bad: "Step 1: Add this line. Step 2: Add that line."
- Good: "Add instrumentation to determine if embeddings are being generated at all"

**Bad: Missing the forest for the trees**
- Bad: "Fix silent failure in batch indexing"
- Good: "Enable knowledge graph search over GitHub issues for 3D code evolution visualization"

### 6. After Planning: Execute or Hand Back

**Single task** → Execute immediately.

**Multiple tasks** → Show summary with dependencies, ask which to start.

**Parallel work**: If tasks have no blocking dependencies, dispatch multiple subagents.

**Don't**: Create a plan and stop. That's casting into the ether.
**Do**: Create a plan, then drive toward completion (or get explicit "pause" from user).

## Extend Over Duplicate

After discovery, if a similar system exists:

1. Document what the existing system does
2. Identify the gap between current and needed capability
3. **Propose extension first**: "Add method X to existing Y" not "Create new Z"

**Why**: Duplication creates drift. Two systems doing similar things will diverge, and maintaining both costs more than extending one.

## Integration

- **Precedes:** executing-plans, subagent-driven-development, writing-plans
- **Follows:** brainstorming (when design work was needed first)
- **Complements:** metathinking (for deep analysis during planning)
