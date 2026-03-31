---
name: metathinking
description: Use when facing complex decisions, debugging mysteries, or architectural choices - activates deep sequential thinking with branching, confidence tracking, and wide exploration patterns
---

# Metathinking Protocol

## When to Use

**Use structured sequential thinking when:**
- Multi-agent coordination (externalized reasoning others can review)
- Debugging mysteries requiring structured hypothesis tracking
- Architectural decisions needing branch exploration history
- Complex decisions where confidence tracking adds value
- You need visible reasoning for review or learning

**Don't use when:**
- Simple, single-step tasks (just do them)
- You only need more thinking depth (use extended thinking instead)
- Single-agent work with no coordination needs
- The overhead of structured output isn't worth it

**Key insight**: This provides EXTERNALIZED REASONING STRUCTURE (logging, branches, coordination), not reasoning itself. Native extended thinking provides raw power. Use together when you need both depth AND visible structure.

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `min_thoughts` | 6 | 3-20 | Minimum thought chain length for complex problems |
| `branch_style` | liberal | conservative, liberal, exhaustive | How aggressively to branch |
| `explore_width` | 4 | 2-7 | Default explore_count when widening |
| `self_checks` | true | true/false | Run the four self-checks at layer 1 |
| `search_wiring` | true | true/false | Auto-execute search when incorporate_search is set |
| `spawn_strategy` | none | none, convergent, divergent, hierarchical | How to handle spawn_candidate hints from the server |

The server surfaces **hints** — observations about your reasoning pattern. You decide what to act on. The server never blocks or enforces.

## Recommendations (with skip costs)

**Third Alternative**: When facing A vs B choices, include "both options could be wrong." False dichotomies are a major cognitive trap. The real answer often lies outside the options presented.
*Skip cost: You accept the first framing presented. In ~30% of cases, the real answer is outside A vs B.*

**Branching on low confidence**: When confidence < 0.6, branching to explore alternatives catches errors that linear thinking misses.
*Skip cost: Linear chains produce the first plausible answer, not the best one.*

**Layer progression**: Start at layer 1 (problem), progress to layer 2 (approach), then layer 3 (details). The server will hint if confidence is high at layer 1 (premature_confidence observation).
*Skip cost: Jumping to implementation without understanding the problem leads to rework.*

**Confidence tracking**: Setting confidence on each thought calibrates your certainty and lets the server surface useful hints.
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
Thought 1:
  "Analyzing the problem. Four approaches emerge..."
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

When confidence < 0.6, branching helps validate uncertain reasoning:

```
Thought N:
  "Uncertain about X. Branching to explore..."
  continuation_mode: "branch"
  branch_from_thought: [previous thought number]
  branch_id: "exploring-alternative-X"
  confidence: 0.5
```

### 3. Branch Merge (converge insights)

When multiple branches have been explored, merge them:

```
Thought N:
  "Synthesizing insights from both exploration paths..."
  continuation_mode: "merge"
  merge_branches: ["approach-a", "approach-b"]
  confidence: 0.75
  layer: 2
```

The server returns a `mergeSummary` with thought counts per branch and any missing branches.

### 4. Layer Progression

- **Layer 1**: Problem understanding -- what are we actually solving?
- **Layer 2**: Approach selection -- which path forward?
- **Layer 3**: Implementation details -- how exactly?

Set `layer` parameter on each thought. The server hints if confidence > 0.8 at layer 1 (premature_confidence).

### 5. Confidence Calibration

- `0.0-0.3`: "I'm guessing" -- explore more before proceeding
- `0.3-0.6`: "I have ideas but uncertainty" -- consider branching
- `0.6-0.8`: "Fairly confident" -- can proceed but verify
- `0.8-1.0`: "High confidence" -- can conclude with `continuation_mode: "done"`

### 6. Search Integration

When you need codebase context mid-thought:

```
Thought N:
  "Need to understand existing patterns..."
  search_query: "authentication middleware pattern"
  incorporate_search: true
```

Then execute the actual search, and pass results in `search_context` on the next thought.

## The Four Self-Checks

Before reaching confidence > 0.6 in any thought, consider these checks:

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
Thought 1 (Layer 1):
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

The kp-sequential-thinking server surfaces non-prescriptive hints. You decide what to act on:

| Hint Kind | Severity | What it means |
|-----------|----------|---------------|
| `linear_chain` | suggestion | N consecutive linear thoughts — branching is available |
| `premature_confidence` | observation | High confidence at layer 1 — may indicate Dunning-Kruger |
| `low_confidence_pattern` | suggestion | Multiple low-confidence thoughts without branching |
| `merge_available` | info | 2+ branches exist — merge can synthesize them |
| `explore_available` | info | exploreCount hasn't been used yet |
| `layer_available` | info | Confidence tracked but layer not set |

## Spawn Strategy

When the server surfaces a `subagent_spawn_available` or `subagent_orchestration` hint, the `spawn_strategy` parameter controls the response:

### none (default)

Ignore spawn hints. All exploration happens within the current thinking session. Use when:
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

## Example: Debug Mystery Bug

```
T1 (L1): "Bug: X not working" -> explore 4 hypotheses -> confidence: 0.3
T2 (L1): Test hypothesis -> search_query: "relevant code" -> confidence: 0.4
T3 (L2): "Found root cause" -> explore 3 solutions -> confidence: 0.7
T4 (L2): "Cleanest approach is Y" -> done -> confidence: 0.85
```

## Activation Checklist

- [ ] Called sequential thinking tool (not just thought about it)
- [ ] Included "both wrong" third alternative for any A vs B choice (Brenner pattern)
- [ ] Set layer parameter on each thought
- [ ] Set confidence parameter on each thought
- [ ] Considered branching when confidence < 0.6
- [ ] Used merge to synthesize when multiple branches explored
- [ ] Executed search tool when incorporate_search was set
- [ ] Didn't claim "done" until alternatives explored
