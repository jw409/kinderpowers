---
name: writing-plans
description: Use when you have a spec or requirements for a multi-step task, before touching code
---

# Writing Plans

## Overview

Translate requirements into plans that empower subagents to solve problems, not follow scripts. A plan should make the executor smarter about the problem — not dumber by reducing them to a text editor.

**Announce at start:** "I'm using the writing-plans skill to create the implementation plan."

**Save plans to:** `docs/plans/YYYY-MM-DD-<feature-name>.md`

## The Anti-Pattern This Skill Exists to Prevent

**The sed-script plan:**

```markdown
### Task 1.1: Add Logging Line
Step 1: Open file
Step 2: Add this exact line at line 387:
    this.logger.info('Generating embeddings...');
Step 3: Save file
Step 4: Commit
```

**Why it's worse than useless:**
- Assumes you know the exact fix (you often don't)
- No context for WHY that line matters
- Subagent becomes a text editor, not a problem solver
- When reality differs from plan (it will), subagent is stuck
- Doesn't transfer what YOU discovered in the session
- Wastes tokens pretending to think while doing mechanical work

If your plan could be replaced by `sed -i '387i\    this.logger.info(...);' file.ts`, you wrote a sed script, not a plan.

## What a Good Plan Contains

### 1. Session Context Transfer (~30% of plan)

The most valuable part. Capture what you learned so the executor doesn't repeat your investigation:

```markdown
## Current System State ([date] Discovery)

### What We Tried
[Exact commands, what happened, what surprised you]

### What Works vs What's Broken
✅ Service X healthy (evidence: curl output)
❌ Operation claims success but results empty
❓ Unknown: is function Y() even being called?

### What We Checked
1. Logs during operation → EMPTY (suspicious)
2. Service health → OK
3. Data format → matches schema
4. Error handling → try/catch swallowing failures (probable root cause)
```

### 2. Mission, Not Steps (~30% of plan)

Define outcomes. The executor breaks them into steps — that's their job.

```markdown
## Mission

**Phase 1: Make indexing actually work**
- Reproduce the failure using commands above
- Find where code claims success but does nothing
- Fix with proper error propagation
- Verify by querying for indexed data

**Phase 2: Enrich indexed data**
- Once Phase 1 works, add git metadata
- Connect to existing enrichment pipeline

You should understand:
- Why logs show nothing during indexing
- What "success" means in the current code
- How to make it fail loudly instead of silently
```

### 3. Approach Guidance (~20% of plan)

Teach methodology, not commands:

```markdown
## How to Approach This

**Key Files (with context, not just paths):**
- `src/indexer.ts:372-423` — addDocuments() claims success here,
  check if embeddings are actually generated before this call
- `src/client.ts:89` — error handling wraps everything in try/catch
  that returns success on failure

**Debugging Strategy:**
1. Add instrumentation at every major step
2. Test with 3 documents, not 82
3. Check if function X() is even being called
4. Verify data format matches what downstream expects

**DON'T:** Assume the fix is adding a log line.
**DO:** Understand the data flow end-to-end first.
```

### 4. Success Criteria (~10% of plan)

Observable outcomes, not task completion:

```markdown
## Success Criteria
- [ ] Run indexing → see log output at every stage
- [ ] Query indexed data → get results (not empty)
- [ ] Root cause documented in commit message
- [ ] Error handling propagates failures instead of swallowing
```

### 5. Discovery Results (~10% of plan)

What exists that the executor should know about:

```markdown
## Discovery
- Searched for: existing indexing, embedding pipelines
- Found: `src/legacy-indexer.ts` — old implementation, partially working
- Decision: extend legacy indexer rather than rewrite
- Related issues: #83 (semantic search roadmap), #56 (embedding pipeline)
```

## Plan Document Structure

```markdown
# [End Goal — what this enables, not what it fixes]

> **For Claude:** Use kinderpowers:executing-plans to implement.
> This is an investigative brief, not a script. Read the whole thing,
> understand the problem, then solve it.

**Goal:** [One sentence — the outcome, not the activity]
**Architecture:** [2-3 sentences about approach]

---

## Current System State
[Session context transfer — what you learned]

## Discovery
[What exists, what was searched, extend-vs-create decisions]

## Mission
[Phases with goals, not steps]

## Approach Guidance
[Key files WITH context, debugging methodology, anti-patterns]

## Success Criteria
[Observable outcomes per phase]

## Rejected Alternatives
[What was considered and why it was dropped — prevents executor from rediscovering dead ends]
```

## Granularity Control

| Level | When | Plan Shape |
|-------|------|-----------|
| **coarse** | Well-understood domain, experienced executor | 3-5 mission objectives, minimal guidance |
| **medium** (default) | Typical feature work | 5-8 phased goals with key files and context |
| **fine** | Unfamiliar domain, complex debugging | Detailed system state, ranked hypotheses, instrumentation strategy |

Granularity controls **depth of context**, not **number of sed commands**.

## Discovery Before Creation

Before proposing ANY new file, tool, or system:

1. Search for similar function names or patterns
2. Search for similar file names
3. Check existing documentation and issue trackers
4. Include results: "Found existing X, will extend" or "No existing solution"

If existing solution found, the plan should say "Extend X to support Y" not "Create new Z."

## Execution Handoff

After saving the plan, offer execution choice:

**1. Subagent-Driven (this session)** — dispatch fresh subagent per phase, review between phases
- **Uses:** kinderpowers:subagent-driven-development

**2. Parallel Session (separate)** — open new session, batch execution with checkpoints
- **Uses:** kinderpowers:executing-plans

## Integration

- **Follows:** strategic-planning (for the investigative/discovery phase)
- **Precedes:** executing-plans, subagent-driven-development
- **Complements:** brainstorming (design work), metathinking (deep analysis)
