---
name: strategic-planning
description: Use when work needs design, planning, or discovery before execution — creates strategic plans that subagents can execute intelligently
---

# Strategic Planning

## Overview

Plans for intelligent subagents are strategic briefs, not sed-scripts. Give direction, context, and WHY. Trust the executor to figure out HOW.

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

## Context-Aware Gates

**First prompt of session** (no established context):
- Search for existing implementations before proposing new
- Check issue trackers for existing work on this topic

**Mid-conversation** (context established):
- Skip gates — you already know the landscape
- Plan based on what you've learned

## Planning Workflow

### 1. Clarity Check
Would a colleague understand this request? If not, ask ONE targeted question.

### 2. Detect Mode

| Mode | Use When | Focus |
|------|----------|-------|
| **Investigative** | Debugging, unclear root cause | Hypotheses, instrumentation, discovery |
| **Implementation** | Features, refactoring, greenfield | Phases, success criteria, approach |

### 3. Create the Plan

Structure your plan around these elements:

```markdown
## Objective
What we're building + Why it matters

## Context
Current state, constraints, and WHY those constraints exist

## Discovery
- Searched: [what you searched for]
- Found: [existing tools/files/patterns]
- Decision: [extend X / no existing solution, creating new]

## Approach
Strategic direction — phases if complex, single action if simple

## Success Criteria
How to verify the work is complete
```

**Investigative mode** adds: ranked hypotheses, instrumentation approach.
**Implementation mode** adds: high-level phases, executor can break into tasks.

### 4. After Planning: Execute or Hand Back

**Single task** → Execute immediately.

**Multiple tasks** → Show summary with dependencies, ask which to start.

**The execution loop**: claim task → execute → close → find next → repeat

**Parallel work**: If tasks have no blocking dependencies, dispatch multiple subagents.

**Don't**: Create a plan and stop. That's casting into the ether.
**Do**: Create a plan, then drive toward completion (or get explicit "pause" from user).

## Discovery Before Creation (Strongly Recommended)

Before proposing ANY new file, tool, or system:

1. Search for similar function names or patterns
2. Search for similar file names
3. Check existing documentation indexes
4. Include discovery results in your plan

**Skip cost**: You risk duplicating existing work. Every parallel system is maintenance burden. If you skip discovery and create something that already exists, expect to redo the work.

## Extend Over Duplicate (Strongly Recommended)

After discovery, if a similar system exists:

1. Document what the existing system does
2. Identify the gap between current and needed capability
3. **Propose extension first**: "Add method X to existing Y" not "Create new Z"

**Skip cost**: Duplication creates drift. Two systems doing similar things will diverge, and maintaining both costs more than extending one.

## Anti-Patterns

| Don't | Do |
|-------|-----|
| Line-by-line sed scripts | Strategic direction |
| "Add this line at line 387" | "Add logging to the auth flow" |
| WHAT without WHY | Always include WHY |
| Create without searching | Discovery before creation |
| New file when extension works | Extend existing systems |

## Writing Style

Complete thoughts, not fragments. Clear but telegraphic. Include enough context for someone with zero codebase knowledge to understand the goal.

## Integration

- **Precedes:** executing-plans, subagent-driven-development
- **Follows:** brainstorming (when design work was needed first)
- **Complements:** metathinking (for deep analysis during planning)
