---
name: stepwise
description: Use when facing complex decisions, debugging mysteries, or architectural choices - activates structured stepwise planning with branching, confidence tracking, and wide exploration patterns
---

# Stepwise Planning Protocol

## When to Use

**Use structured stepwise planning when:**
- Multi-agent coordination (a shared work log others can review)
- Debugging mysteries requiring structured hypothesis tracking
- Architectural decisions needing branch exploration history
- Complex decisions where confidence tracking adds value
- You need a visible decision trail for review or learning

**Don't use when:**
- Simple, single-step tasks (just do them)
- You only need more analysis depth (work the problem directly instead)
- Single-agent work with no coordination needs
- The overhead of structured output isn't worth it

**Key insight**: This provides EXTERNALIZED PLAN STRUCTURE (logging, branches, coordination), not analysis itself. Use it when you need a visible, reviewable decision trail.

For reasoning models such as OpenAI Sol, do not turn that decision trail into a narrated scratchpad. Let the model work normally between calls and add a checkpoint only when the workflow gains a conclusion, evidence, revision, decision, result, or handoff. A checkpoint should contain only externally reviewable work state.

## Multi-turn continuity

- Set `channelId` on every call. It is the independent planner lane; give parallel agents distinct channel IDs through their orchestrator.
- Set `roomId` when several agents belong to one task or phase. If omitted, the MCP host session is the room.
- Keep `stepNumber` channel-local and monotonic. Every new channel starts at 1; use that channel's returned `expectedNextStep` for its next call.
- Set `turnId` to a stable caller-defined identifier when a user/assistant turn contains one or more checkpoints.
- Use `checkpointKind`, `evidence`, `openQuestions`, and `nextAction` to carry reviewable state into later turns.
- Use `(roomId, channelId)` as the plan-state join key. `sessionId` identifies the MCP host process for auditing; in Codex, `CODEX_THREAD_ID` supplies it automatically and `STEPWISE_SESSION_ID` can override it.
- Never pipeline dependent checkpoints within one channel. Different channels may proceed in parallel.

Local JSONL logging is controlled by `KP_STEPWISE_LOG_MODE=full|metadata|off` (default `full`). Logs are written to `var/stepwise_logs/{roomId}/channels/{channelId}.jsonl`. `metadata` retains timing, routing, structure, and presence flags but omits checkpoint text, evidence, proposals, questions, next action, and search query.

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `min_steps` | adaptive | 1-20 | Minimum checkpoint count; reasoning models should prefer sparse, meaningful checkpoints |
| `branch_style` | liberal | conservative, liberal, exhaustive | How aggressively to branch |
| `explore_width` | 4 | 2-7 | Default explore_count when widening |
| `self_checks` | true | true/false | Run the four self-checks at layer 1 |
| `search_wiring` | true | true/false | Auto-execute search when incorporate_search is set |
| `spawn_strategy` | none | none, convergent, divergent, hierarchical | How to handle spawn_candidate hints from the server |

The server surfaces **hints** — observations about your work pattern. You decide what to act on. The server never blocks or enforces.

## Recommendations (with skip costs)

**Third Alternative**: When facing A vs B choices, include "both options could be wrong." False dichotomies are a major cognitive trap. The real answer often lies outside the options presented.
*Skip cost: You accept the first framing presented. In ~30% of cases, the real answer is outside A vs B.*

**Branching on low confidence**: When confidence < 0.6, branching to explore alternatives catches errors that a linear plan misses.
*Skip cost: Linear chains produce the first plausible answer, not the best one.*

**Layer progression**: Start at layer 1 (problem), progress to layer 2 (approach), then layer 3 (details). The server will hint if confidence is high at layer 1 (premature_confidence observation).
*Skip cost: Jumping to implementation without understanding the problem leads to rework.*

**Confidence tracking**: Setting confidence on each step calibrates your certainty and lets the server surface useful hints.
*Skip cost: No hints, no calibration signals. You're flying blind.*

## Core Patterns

### 0. Third Alternative (Brenner Pattern)

When facing any A vs B choice, enumerate a third option: "both could be wrong."

> "Someone said, 'Either model A is right or model B is right.' And I said, 'You've forgotten there's a third alternative... Both could be wrong.'" -- Sydney Brenner

The third alternative should ask:
- What assumption makes A vs B the only choices?
- What if that assumption is wrong?
- What would a completely different framing look like?

### 1. Wide Exploration (Default Start)

Start with `continuation_mode: "explore"` and `explore_count: 4`:

```
Step 1:
  roomId: "eval-repair"
  channelId: "core"
  step: "Three viable approaches remain after repository inspection."
  turnId: "turn-1"
  checkpointKind: "observation"
  evidence: ["src/server.rs", "baseline MCP transcript"]
  nextAction: "Compare the three approaches against the acceptance criteria."
  continuation_mode: "explore"
  explore_count: 4
  proposals: [
    "Approach A: [description] - tradeoff X",
    "Approach B: [description] - tradeoff Y",
    "Approach C: [description] - tradeoff Z",
    "Approach D: [description] - tradeoff W"
  ]
  confidence: 0.4
  layer: 1
```

### 2. Branch on Uncertainty

When confidence < 0.6, branching helps validate uncertain conclusions:

```
Step N:
  "Uncertain about X. Branching to explore..."
  continuation_mode: "branch"
  branch_from_step: [previous step number]
  branch_id: "exploring-alternative-X"
  confidence: 0.5
```

### 3. Branch Merge (converge insights)

When multiple branches have been explored, merge them:

```
Step N:
  "Synthesizing insights from both exploration paths..."
  continuation_mode: "merge"
  merge_branches: ["approach-a", "approach-b"]
  confidence: 0.75
  layer: 2
```

The server returns a `mergeSummary` with step counts per branch and any missing branches.

### 4. Layer Progression

- **Layer 1**: Problem understanding -- what are we actually solving?
- **Layer 2**: Approach selection -- which path forward?
- **Layer 3**: Implementation details -- how exactly?

Set `layer` parameter on each step. The server hints if confidence > 0.8 at layer 1 (premature_confidence).

### 5. Confidence Calibration

- `0.0-0.3`: "I'm guessing" -- explore more before proceeding
- `0.3-0.6`: "I have ideas but uncertainty" -- consider branching
- `0.6-0.8`: "Fairly confident" -- can proceed but verify
- `0.8-1.0`: "High confidence" -- can conclude with `continuation_mode: "done"`

### 6. Search Integration

When you need codebase context mid-plan:

```
Step N:
  "Need to understand existing patterns..."
  search_query: "authentication middleware pattern"
  incorporate_search: true
```

Then execute the actual search, and pass results in `search_context` on the next step.

## The Four Self-Checks

Before reaching confidence > 0.6 in any step, consider these checks:

### 1. Verify Before Assuming

> "Before concluding 'X works', have I actually tested it?"

### 2. Discovery Before Creation

> "Before proposing a new solution, have I searched for existing ones?"

### 3. Deep Inspection Required

> "Before claiming understanding, did I see the full picture or just the first 20 lines?"

### 4. Extend Over Duplicate

> "Before designing something new, have I considered extending what exists?"

### Integration Pattern

In Layer 1 (problem understanding), run all four checks:

```
Step 1 (Layer 1):
  "Understanding the problem..."
  layer: 1
  confidence: 0.3

  SELF-CHECK:
  [ ] verify-before-assuming: Am I claiming something works without testing?
  [ ] discovery-before-creation: Am I proposing new without searching existing?
  [ ] deep-inspection-required: Did I see full context or partial?
  [ ] extend-over-duplicate: Am I designing new without considering extensions?
```

Transition to Layer 2 when checks pass.

## Server Hints

The kp-stepwise server surfaces non-prescriptive hints. You decide what to act on:

| Hint Kind | Severity | What it means |
|-----------|----------|---------------|
| `linear_chain` | suggestion | N consecutive linear steps — branching is available |
| `premature_confidence` | observation | High confidence at layer 1 — may indicate Dunning-Kruger |
| `low_confidence_pattern` | suggestion | Multiple low-confidence steps without branching |
| `merge_available` | info | 2+ branches exist — merge can synthesize them |
| `explore_available` | info | exploreCount hasn't been used yet |
| `layer_available` | info | Confidence tracked but layer not set |

## Spawn Strategy

When the server surfaces a `subagent_spawn_available` or `subagent_orchestration` hint, the `spawn_strategy` parameter controls the response:

Before spawning, assign each worker a unique `channelId` and include the shared `roomId` in its prompt. For example: core=`core`, implementation worker=`opus-gold-type`, replay worker=`sonnet-corpus-replay`. `branchId` is only a branch label inside one channel; it is never a substitute for channel isolation.

### none (default)

Ignore spawn hints. All exploration happens within the current planning session. Use when:
- Single-agent work with no orchestrator
- Simple problems that don't warrant parallelism
- Context budget is tight

### convergent

Spawn subagents that explore branch points independently, then merge results. Use when:
- You need agreement/consensus across approaches
- The problem has a single correct answer explored from multiple angles

### divergent

Spawn subagents that explore independently WITHOUT merging. The orchestrator selects the best. Use when:
- You want the widest possible solution space
- Multiple valid answers exist (creative tasks, brainstorming)

### hierarchical

Spawn subagents in layers. Layer 1 explores, reports to layer 2 synthesizer. Use when:
- Deep, multi-level problems (architecture, system design)
- Delegation to specialized subagents at different layers

### When to spawn vs. explore linearly

| Signal | Action |
|--------|--------|
| 2 proposals, one clearly better | Explore linearly |
| 3+ proposals, unclear winner | Spawn subagents |
| `subagent_orchestration` hint (3+ branches) | Definitely spawn |
| Time pressure / cost constraints | Explore linearly |
| Complex trade-offs needing deep analysis | Spawn subagents |

## Anti-Patterns

- **Accepting binary choices without "both wrong" option** -- question the framing
- Starting with high confidence (> 0.7) on complex problems
- Skipping layer 1 (problem understanding)
- Using `continuation_mode: "done"` before exploring alternatives
- Setting `explore_count: 1` or `2` (minimum useful is 3-4)
- **Spawning without strategy** -- if you spawn subagents, set spawn_strategy explicitly. Default "none" means hints are informational only.
- **Sharing a channel across parallel agents** -- this merges their ordering, branches, and confidence heuristics. Assign one channel per independently running agent.
- **Treating `branchId` as agent isolation** -- branches are scoped inside a channel and do not create independent planner state.
- Logging a stream of speculative internal narration instead of meaningful external checkpoints
- Reusing or skipping `stepNumber` within a channel; use that channel's `expectedNextStep` and retry rejected out-of-order calls

## Example: Debug Mystery Bug

```
S1 (L1): "Bug: X not working" -> explore 4 hypotheses -> confidence: 0.3
S2 (L1): Test hypothesis -> search_query: "relevant code" -> confidence: 0.4
S3 (L2): "Found root cause" -> explore 3 solutions -> confidence: 0.7
S4 (L2): "Cleanest approach is Y" -> done -> confidence: 0.85
```

## Activation Checklist

- [ ] Called the stepwise_plan tool (not just planned in your head)
- [ ] Included "both wrong" third alternative for any A vs B choice (Brenner pattern)
- [ ] Set layer parameter on each step
- [ ] Set confidence parameter on each step
- [ ] Considered branching when confidence < 0.6
- [ ] Used merge to synthesize when multiple branches explored
- [ ] Executed search tool when incorporate_search was set
- [ ] Didn't claim "done" until alternatives explored
