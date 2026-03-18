---
name: metathinking
description: Use when facing complex decisions, debugging mysteries, or architectural choices - activates deep sequential thinking with mandatory branching, confidence tracking, and wide exploration patterns
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

## Hard Requirements

**Third Alternative**: When facing A vs B choices, include "both options could be wrong." False dichotomies are a major cognitive trap. The real answer often lies outside the options presented.

**Minimum Thought Chain**: 6 thoughts minimum for complex problems. Stopping at 3 means you haven't explored enough.

**Minimum Branches**: At least 1 branch per 4 linear thoughts. Fully-linear chains on complex problems indicate insufficient exploration.

**Minimum Explore Count**: Use `explore_count: 4` or higher when exploring alternatives.

**Confidence Tracking**: Every thought should include a `confidence` parameter.

**Layer Progression**: Start at layer 1, progress through layers. Skipping to layer 3 without understanding the problem (layer 1) leads to premature solutions.

**Cost of skipping**: Linear thinking without branches produces the first plausible answer, not the best one. Branching catches the 30-40% of cases where the obvious answer is wrong.

## Core Patterns

### 0. Third Alternative (Brenner Pattern)

When facing any A vs B choice, enumerate a third option: "both could be wrong."

> "Someone said, 'Either model A is right or model B is right.' And I said, 'You've forgotten there's a third alternative... Both could be wrong.'" -- Sydney Brenner

The third alternative should ask:
- What assumption makes A vs B the only choices?
- What if that assumption is wrong?
- What would a completely different framing look like?

```
Thought N:
  "Evaluating options..."
  proposals: [
    "Option A: [description]",
    "Option B: [description]",
    "Third Alternative: Both A and B are wrong because [the framing assumes X, which may be false]"
  ]
```

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

If confidence < 0.6, branch to explore alternatives:

```
Thought N:
  "Uncertain about X. Branching to explore..."
  continuation_mode: "branch"
  branch_from_thought: [previous thought number]
  branch_id: "exploring-alternative-X"
  confidence: 0.5
```

### 3. Layer Progression

- **Layer 1**: Problem understanding -- what are we actually solving?
- **Layer 2**: Approach selection -- which path forward?
- **Layer 3**: Implementation details -- how exactly?

Set `layer` parameter on each thought. Don't jump to layer 3 without layers 1-2.

### 4. Confidence Calibration

- `0.0-0.3`: "I'm guessing" -- explore more before proceeding
- `0.3-0.6`: "I have ideas but uncertainty" -- branch to validate
- `0.6-0.8`: "Fairly confident" -- can proceed but verify
- `0.8-1.0`: "High confidence" -- can conclude with `continuation_mode: "done"`

### 5. Search Integration

When you need codebase context mid-thought:

```
Thought N:
  "Need to understand existing patterns..."
  search_query: "authentication middleware pattern"
  incorporate_search: true
```

Then execute the actual search, and pass results in `search_context` on the next thought.

## The Four Self-Checks

Before reaching confidence > 0.6 in any thought, run these checks:

### 1. Verify Before Assuming

> "Before concluding 'X works', have I actually tested it?"

If your thought claims "this should work" and you haven't verified, confidence stays < 0.6 until verified.

### 2. Discovery Before Creation

> "Before proposing a new solution, have I searched for existing ones?"

If your thought proposes creating something new without searching the codebase first, confidence stays < 0.6 until searched.

### 3. Deep Inspection Required

> "Before claiming understanding, did I see the full picture or just the first 20 lines?"

If you only read partial content, confidence stays < 0.6 until fully inspected.

### 4. Extend Over Duplicate

> "Before designing something new, have I considered extending what exists?"

If you haven't explored extension options for existing code, confidence stays < 0.6 until evaluated.

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

Only transition to Layer 2 when all four checks pass.

## Anti-Patterns

- **Accepting binary choices without "both wrong" option** -- always question the framing
- Starting with high confidence (> 0.7) on complex problems
- Skipping layer 1 (problem understanding)
- Linear thinking without branches on first attempt
- Using `continuation_mode: "done"` before exploring alternatives
- Setting `explore_count: 1` or `2` (minimum useful is 3-4)
- Forgetting to set `confidence` parameter

## Example: Debug Mystery Bug

```
T1 (L1): "Bug: X not working" -> explore 4 hypotheses -> confidence: 0.3
T2 (L1): Test hypothesis -> search_query: "relevant code" -> confidence: 0.4
T3 (L2): "Found root cause" -> explore 3 solutions -> confidence: 0.7
T4 (L2): "Cleanest approach is Y" -> done -> confidence: 0.85
```

**Pattern**: Layer 1 = understand problem, Layer 2 = solution approaches. Branch on low confidence.

## Activation Checklist

- [ ] Called sequential thinking tool (not just thought about it)
- [ ] Included "both wrong" third alternative for any A vs B choice (Brenner pattern)
- [ ] Set layer parameter on each thought
- [ ] Set confidence parameter on each thought
- [ ] Branched when confidence < 0.6
- [ ] Executed search tool when incorporate_search was set
- [ ] Didn't claim "done" until alternatives explored
