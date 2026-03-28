---
name: comprehensive-pr-review
description: Team-based PR review protocol with 7 communicating personas, structured debate, tiered depth, and self-improvement loop. Built on multi-perspective-review as backbone. Use when reviewing PRs that matter — not for typo fixes.
---

# Comprehensive PR Review Protocol

Built on `kinderpowers:multi-perspective-review` as backbone. Adapted from meshly's v3 protocol.

> *"Favor approving once it definitely improves overall code health, even if it isn't perfect. There is no such thing as 'perfect' code — there is only better code."* — Google eng-practices

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `tier` | auto | quick, standard, full, deep | Review depth. Auto-selects based on PR size/labels. |
| `pedanticness` | medium | low, medium, high, maximum | How much to flag. Low = blocking only. Maximum = style nits. |
| `mode` | review | review, fix, recommend, issue | Output mode. Review = GitHub comment. Fix = commit directly. |
| `round` | 1 | 1, 2 | Round 2 only verifies round 1 fixes + new critical findings. |
| `personas` | auto | list of persona names | Override which personas participate. |

## Overview

7 communicating personas debate, challenge each other, and produce a single unified review. Not 7 independent reports concatenated — a synthesized output where conflicts are resolved.

**Why teams over swarms:** Independent agents produce redundant findings, miss cross-cutting concerns, and generate volume without prioritization. A team that communicates identifies the 3 things that matter instead of 50 that don't.

## Pre-Review Gates (check before anything else)

| Gate | Check | Failure = |
|------|-------|-----------|
| **Test Plan** | Unchecked `- [ ]` boxes in PR body "Test plan" section | BLOCKING — all items must be checked |
| **Phantom Tracking** | "track separately" / "follow-up needed" WITHOUT a `#NNN` issue link | BLOCKING — every follow-up needs a real issue |
| **LLM Lie Detection** | "fixed 3 blockers, tracking 6 others" where the "6 others" have no issue links | BLOCKING — AI PRs are the worst offenders |

## Review Tiers

| Tier | Personas | When | Token Budget |
|------|----------|------|-------------|
| **Quick** | QA + Pen Holder | Docs, config, < 20 lines | ~200K |
| **Standard** | QA + Staff Eng + EM + Pen Holder | Bug fixes, small features | ~600K |
| **Full** | All 7 | New features, architecture, > 200 lines | ~1.5M |
| **Deep** | All 7 + adversarial-review | Security-critical, compliance, data model changes | ~2M |

**Auto-selection:**
- Files touch auth/crypto/secrets → Deep
- files > 10 OR additions > 200 OR new feature → Full
- additions > 20 OR bug fix → Standard
- Else → Quick

## The 7 Personas

| # | Persona | Focus | Agent Type |
|---|---------|-------|-----------|
| 1 | **QA Expert** | Tests, error handling, edge cases, CI signal | `quality-gate` (strictness from pedanticness) |
| 2 | **Staff Engineer** | Architecture, patterns, layer boundaries, security, performance | `code-reviewer` |
| 3 | **Engineering Manager** | Review discipline, contributor UX, round management | Direct (no subagent) |
| 4 | **DevProd Director** | CI/CD health, build impact, developer experience | `gsd-verifier` |
| 5 | **PM / TPM** | Scope, product impact, breaking changes, release readiness | Direct |
| 6 | **Support Advocate** | User impact, docs, error messages, migration | `comment-analyzer` |
| 7 | **Pen Holder** | Synthesis, debate resolution, final output | `multi-perspective-review` (council mode) |

## Communication Flow

All personas use SendMessage:

```
Persona → Pen Holder (primary: send findings)
Persona → Persona (cross-pollination: "I found X that relates to your Y")
Pen Holder → Persona (clarification: "Staff Eng disagrees — evidence?")
Pen Holder → Team Lead (final synthesized review)
```

## Review Philosophy (non-negotiable)

1. **Approve once it improves code health.** No perfect code exists — only better code.
2. **Be kind. Comment on the code, never the developer.** "This function..." not "You forgot..."
3. **Explicit severity on every finding:**
   - `**BLOCKING:**` — must fix before merge
   - *(no prefix)* — standard finding at current pedanticness
   - `Optional:` — improvement, non-blocking
   - `Nit:` — style, take-it-or-leave-it
   - `Note:` — educational, not a request for change
4. **"Clean it up later" is a lie.** No issue number = it won't happen.
5. **Celebrate good work.** Every review should include positive observations — good work deserves recognition.

## Review Checklist (coverage guarantee)

| Dimension | Persona | What to Check |
|-----------|---------|---------------|
| Design | Staff Engineer | Architecture alignment, right abstraction level |
| Functionality | QA Expert | Does it do what's intended? Trace the happy path. |
| Complexity | DevProd Director | Over-engineered? Understandable in 5 min? |
| Performance | Staff Engineer | Big O? Memory? What happens at 10x/100x? |
| Edge Cases | QA Expert | Empty, huge, unicode, injection, boundary values |
| Silent Failures | QA Expert | Swallowed exceptions? Ignored returns? |
| Security | Staff Engineer | Injection, privilege escalation, data exposure |
| Tests | QA Expert | Appropriate? Not trivially passing? Negative cases? |
| Naming | Support Advocate | Clear, domain-appropriate, not too long/short |
| Comments | Support Advocate | Explain WHY not WHAT? Stale comments removed? |
| Documentation | Support Advocate | README/API docs updated? Migration guide? |
| Good Things | ALL | What was done well? (MANDATORY) |

## Pedanticness Calibration

| Author | PR Type | Pedanticness |
|--------|---------|-------------|
| External contributor | Any | Low |
| Team contributor | Bug fix | Low |
| Team contributor | Feature | Medium |
| Bot/AI-generated | Any | High |
| Own draft PR | Any | High-Maximum |

## Round Discipline

| Round | Purpose | Rules |
|-------|---------|-------|
| 1 | Find everything. No surprises later. | Comprehensive. All blocking items listed. |
| 2 | Verify fixes only. | New findings only if introduced by round 1 fixes. |

If round 2 surfaces a finding round 1 missed → that's a protocol bug. Fix the protocol, not just the code.

## Output Format

One synthesized review comment, not per-persona outputs:

```markdown
## PR Review: #{number} — {title}

**Tier:** {Quick|Standard|Full|Deep} | **Pedanticness:** {level} | **Round:** {1|2}

### Verdict: {APPROVE | REQUEST_CHANGES | COMMENT}

{1-3 sentence executive summary}

### What's Good
- {Celebrate what was done well — MANDATORY}

### Blocking Issues
1. **BLOCKING:** {issue} — `{file:line}`
   {Why it matters. Specific fix recommendation.}
   *Consensus: flagged by {personas}*

### Findings
1. {Finding} — `{file:line}`
2. Optional: {Improvement idea} — `{file:line}`
3. Nit: {Style observation} — `{file:line}`
4. Note: {Educational context} — `{file:line}`

### Grade: {A-F}
| Dimension | Score | Notes |
|-----------|-------|-------|
| Security | {A-F} | {brief} |
| Tests | {A-F} | {brief} |
| Architecture | {A-F} | {brief} |
| Maintainability | {A-F} | {brief} |
| Documentation | {A-F} | {brief} |
```

## Self-Improvement Loop

Every review evaluates the review process itself:
- Round 2 finding that should have been round 1 → fix the protocol
- False positive that wasted time → add suppression
- Recurring pattern → add automated check
- Process created noise → fix the process

## Anti-Patterns

- Adding new blockers in round 2+ (moving goalposts)
- Running Deep tier on contributor PRs (volume overwhelms signal)
- 4+ review rounds (blocks shipping, demoralizes)
- Labeling style as "blocking" (preference ≠ defect)
- "Questions to consider" that expect action (passive-aggressive blocking)
- Creating GitHub issues for every non-blocking finding (issue spam)
- Commenting on the developer, not the code
- No positive observations (demoralizing)
- Blocking on "not how I'd do it" (approve if it improves health)

## Skip Cost

Skipping this protocol means single-perspective review. You lose:
- Cross-pollination between QA and architecture perspectives
- Conflict resolution (when two reviewers disagree on severity)
- Pedanticness calibration (every PR gets the same depth)
- Self-improvement loop (protocol never gets better)
- Positive observations (reviews become purely critical)
