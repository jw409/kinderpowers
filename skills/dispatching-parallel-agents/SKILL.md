---
name: dispatching-parallel-agents
description: Use when facing 2+ independent tasks that can be worked on without shared state or sequential dependencies
---

# Dispatching Parallel Agents

## Overview

When you have multiple unrelated failures (different test files, different subsystems, different bugs), investigating them sequentially wastes time. Each investigation is independent and can happen in parallel.

**Core principle:** Dispatch one agent per independent problem domain. Let them work concurrently.

## Why This Skill Exists

Sequential investigation of independent problems means:
- Time wasted waiting for each to complete
- Context switching overhead between different domains
- Longer feedback loops

Parallel dispatch turns N sequential investigations into 1 parallel block.

## When to Use

```
Decision Framework:

Multiple failures?
├── No → Single investigation
└── Yes → Are they independent?
    ├── No (related) → Single agent investigates all
    └── Yes → Can they work in parallel?
        ├── No (shared state) → Sequential agents
        └── Yes → Parallel dispatch
```

**Good fit:**
- 3+ test files failing with different root causes
- Multiple subsystems broken independently
- Each problem can be understood without context from others
- No shared state between investigations

**Poor fit:**
- Failures are related (fix one might fix others)
- Need to understand full system state
- Agents would interfere with each other

## The Pattern

### 1. Identify Independent Domains

Group failures by what's broken:
- File A tests: Tool approval flow
- File B tests: Batch completion behavior
- File C tests: Abort functionality

Each domain is independent - fixing tool approval doesn't affect abort tests.

### 2. Create Focused Agent Tasks

Each agent gets:
- **Specific scope:** One test file or subsystem
- **Clear goal:** Make these tests pass
- **Constraints:** Don't change other code
- **Expected output:** Summary of what you found and fixed

### 3. Dispatch in Parallel

```typescript
// In Claude Code / AI environment
Task("Fix agent-tool-abort.test.ts failures")
Task("Fix batch-completion-behavior.test.ts failures")
Task("Fix tool-approval-race-conditions.test.ts failures")
// All three run concurrently
```

### 4. Review and Integrate

When agents return:
- Read each summary
- Verify fixes don't conflict
- Run full test suite
- Integrate all changes

## Agent Prompt Structure

Good agent prompts are:
1. **Focused** - One clear problem domain
2. **Self-contained** - All context needed to understand the problem
3. **Specific about output** - What should the agent return?

```markdown
Fix the 3 failing tests in src/agents/agent-tool-abort.test.ts:

1. "should abort tool with partial output capture" - expects 'interrupted at' in message
2. "should handle mixed completed and aborted tools" - fast tool aborted instead of completed
3. "should properly track pendingToolCount" - expects 3 results but gets 0

These are timing/race condition issues. Your task:

1. Read the test file and understand what each test verifies
2. Identify root cause - timing issues or actual bugs?
3. Fix by:
   - Replacing arbitrary timeouts with event-based waiting
   - Fixing bugs in abort implementation if found
   - Adjusting test expectations if testing changed behavior

Avoid just increasing timeouts - find the real issue.

Return: Summary of what you found and what you fixed.
```

## Anti-Patterns and Consequences

**Too broad scope:** "Fix all the tests"
- Consequence: Agent gets lost, makes scattered changes, may introduce new bugs

**No context:** "Fix the race condition"
- Consequence: Agent spends time rediscovering what you already know

**No constraints:** No guidance on what to change
- Consequence: Agent might refactor everything, touching code it shouldn't

**Vague output expectation:** "Fix it"
- Consequence: You don't know what changed, harder to review

## When Sequential Investigation is Better

**Related failures:** Fixing one might fix others - investigate together first to avoid duplicate or conflicting fixes.

**Need full context:** Understanding requires seeing entire system - parallel agents each have partial view.

**Exploratory debugging:** You don't know what's broken yet - need one coherent investigation.

**Shared state:** Agents would interfere (editing same files, using same resources).

## Real Example

**Scenario:** 6 test failures across 3 files after major refactoring

**Failures:**
- agent-tool-abort.test.ts: 3 failures (timing issues)
- batch-completion-behavior.test.ts: 2 failures (tools not executing)
- tool-approval-race-conditions.test.ts: 1 failure (execution count = 0)

**Decision:** Independent domains - abort logic separate from batch completion separate from race conditions

**Dispatch:**
```
Agent 1 → Fix agent-tool-abort.test.ts
Agent 2 → Fix batch-completion-behavior.test.ts
Agent 3 → Fix tool-approval-race-conditions.test.ts
```

**Results:**
- Agent 1: Replaced timeouts with event-based waiting
- Agent 2: Fixed event structure bug (threadId in wrong place)
- Agent 3: Added wait for async tool execution to complete

**Integration:** All fixes independent, no conflicts, full suite green

**Time saved:** 3 problems solved in parallel vs sequentially

## Verification After Dispatch

After agents return:
1. **Review each summary** - Understand what changed
2. **Check for conflicts** - Did agents edit same code?
3. **Run full suite** - Verify all fixes work together
4. **Spot check** - Agents can make systematic errors

## Costs of Skipping

If you investigate sequentially when parallel would work:

| Cost | Impact |
|------|--------|
| Wall-clock time | 3x longer (or more) for N independent problems |
| Context switching | Mental overhead switching between domains |
| Delayed feedback | Longer until you know if approach is working |

**When sequential might still be right:**
- Only 2 problems (parallelization overhead may not be worth it)
- You suspect they're related despite appearing independent
- You want to learn from each investigation before starting the next
- Resource constraints make parallel execution impractical

## Agent's Judgment

You have agency over when parallel dispatch makes sense. This skill documents a pattern that's proven effective, but you understand your specific situation.

If the problems seem related, investigate together. If you're uncertain about independence, start with one and see what you learn.
