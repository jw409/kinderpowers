---
name: adversarial-review
description: Use when reviewing significant changes, completed features, or architecture decisions — assumes issues exist and systematically finds them rather than confirming quality
---

# Adversarial Review

## Overview

Adversarial review is a disciplined approach to finding problems in work product. The reviewer's job is to discover issues, not to validate quality. "Looks good" without substantive analysis suggests the review needs more time or context.

**Core principle:** Assume issues exist. Your job is to find them.

**Announce at start:** "I'm using the adversarial-review skill to examine this work for issues."

**Relationship to code-review:** The requesting-code-review skill handles the workflow of dispatching a reviewer. This skill provides the methodology for *how* to review rigorously. They complement each other — use requesting-code-review for the process, adversarial-review for the depth.

## When to Use

**Strongly recommended for:**
- Before merging significant changes (multiple files, new features, refactors)
- After completing a major feature or milestone
- During architecture review (ADRs, system design)
- When reviewing requirements or specifications
- Security-sensitive changes (auth, permissions, data handling, cryptography)

**Proportional review — match depth to risk:**
- **Full adversarial:** New features, security changes, architecture decisions, public APIs
- **Standard review:** Routine changes, internal refactors, test additions
- **Light review:** Typo fixes, config changes, documentation updates, dependency bumps

**Cost of skipping:** Issues caught in review cost minutes to fix. Issues caught in production cost hours to debug. Earlier is cheaper.

## Review Protocol

### Step 1: Read the Entire Change First

Read the full diff, document, or design before commenting on any line. Context changes interpretation — a function that looks wrong in isolation might make perfect sense in the broader change.

**Why this matters:** Line-by-line review without context produces false positives ("this function is unused!" — it's used in the next file) and misses systemic issues (the change works file-by-file but introduces a circular dependency).

### Step 2: Check Against Stated Requirements

Pull up the requirements, acceptance criteria, or task description. Verify:
- Does the implementation satisfy each acceptance criterion?
- Are there requirements that aren't addressed?
- Does the implementation do things beyond what was specified? (Scope creep in code is still scope creep.)

### Step 3: Hunt for Issues

Systematically check each category:

**Correctness:**
- Does it do what it claims to do?
- Edge cases: empty inputs, boundary values, concurrent access, error paths
- Off-by-one errors, null/undefined handling, type mismatches

**Security:**
- Input validation and sanitization
- Authentication and authorization checks
- Data exposure (logging sensitive data, error messages leaking internals)
- Injection vulnerabilities (SQL, XSS, command injection)

**Performance:**
- Unnecessary computation (N+1 queries, redundant iterations)
- Missing pagination or limits on unbounded collections
- Blocking operations in async contexts
- Memory leaks (unclosed resources, growing caches without eviction)

**Maintainability:**
- Code clarity — would a new contributor understand this?
- Abstraction level — is complexity hidden or exposed?
- Test coverage — are the important behaviors tested?
- Error messages — do they help the next person debug?

**Consistency:**
- Does it follow existing patterns in the codebase?
- Naming conventions, error handling style, logging approach
- If it deviates from convention, is that intentional and documented?

### Step 4: Classify Findings

| Severity | Meaning | Action |
|----------|---------|--------|
| **Blocking** | Prevents merge. Bug, security issue, broken functionality | Fix before proceeding |
| **Important** | Should be fixed. Maintainability, performance, missing tests | Fix before merge, or create tracked follow-up |
| **Minor** | Would improve quality. Style, naming, small optimizations | Fix if convenient, otherwise note for later |
| **Nit** | Purely stylistic. No functional impact | Author's discretion |

### Step 5: The Zero-Issue Check

If you found zero issues, pause. Possible explanations:

- **The work is genuinely excellent.** This happens, but less often than reviewers assume. Re-examine with fresh eyes.
- **You reviewed too quickly.** Slow down. Read the actual code, not just the diff summary.
- **You lack context.** You might not know the codebase well enough to spot issues. That's okay — note it and focus on what you can evaluate (logic, clarity, test coverage).
- **The scope was small.** A one-line fix legitimately might have zero issues. Proportional review means this is fine for small changes.

For significant changes, finding zero issues should prompt a second look, not immediate approval.

## Constructive Adversarial

**Finding problems is half the job. Suggesting solutions is the other half.**

Bad review comment:
> "This won't scale."

Good review comment:
> "This loads all records into memory (line 47). With the expected dataset size (~100K records), this will use ~2GB RAM. Consider pagination or streaming. Example: `cursor.fetchmany(batch_size=1000)` in a loop."

**The formula:** [What's wrong] + [Why it matters] + [Suggested fix or direction]

Criticism without alternatives is venting, not reviewing. If you can't suggest a fix, at least explain the concern clearly enough that the author can find their own solution.

## What to Review Adversarially

This methodology applies beyond code:

| Artifact | What to Look For |
|----------|-----------------|
| **Code** | Bugs, security, performance, maintainability, test coverage |
| **Architecture decisions** | Missing alternatives, underestimated consequences, unstated assumptions |
| **Requirements** | Ambiguity, missing edge cases, contradictions, untestable criteria |
| **Test suites** | Missing scenarios, weak assertions, tests that can't fail, flaky patterns |
| **Documentation** | Inaccuracies, missing context, outdated information, misleading examples |

## Multi-Perspective Review (Council Mode)

For significant decisions, a single-perspective review misses things. Council mode spawns disposable lenses — not permanent personas, but task-scoped perspectives selected based on **what could break**.

### Smart Persona Selection

Don't pick personas from a fixed roster. Pick based on the artifact and its risk profile:

| What You're Reviewing | Lenses to Spawn | Why These |
|-----------------------|-----------------|-----------|
| API design | Edge Case + Contract + Empathy | Margins, promises, newcomer confusion |
| Security change | Edge Case + Resilience + Contract | Injection, failure modes, guarantees |
| Architecture decision | Workflow + Empathy + Contract | User journey, team impact, promises |
| Documentation | Workflow + Empathy + Documentation | Can I follow it? Do I understand it? Is it true? |
| Performance claim | Contract + Edge Case + Resilience | Is it real? What breaks it? What about load? |

### The Six Lenses (from Reformed Troll Testing)

Each lens asks a different question:

| Lens | Core Question | Spawns When |
|------|---------------|-------------|
| **WORKFLOW** | "Can a real user actually do this?" | User-facing features, docs, onboarding |
| **EDGE CASE** | "What happens with unusual inputs?" | Parsers, APIs, data handling |
| **RESILIENCE** | "What happens when things go sideways?" | Infrastructure, distributed systems, error handling |
| **CONTRACT** | "Does it actually do what it says?" | APIs, guarantees, performance claims |
| **DOCUMENTATION** | "Can I believe the docs?" | Any documented behavior or example |
| **EMPATHY** | "What would confuse a newcomer?" | Onboarding, naming, error messages |

### How to Run

1. **Select 2-3 lenses** based on the table above (not all 6 — that's expensive and most won't apply)
2. **Spawn one agent per lens** — each with a role-scoped prompt (see the multi-perspective-review agent)
3. **Each agent reviews independently** — no coordination, no groupthink
4. **Synthesize**: One pass to identify consensus (multiple lenses flagged same issue) and divergence (one lens found something others missed)

### Emotional Indirection (Reformed Troll Pattern)

Findings land better when attributed to a role, not stated as direct criticism:

- **Instead of**: "Your error handling is broken"
- **Say**: "The RESILIENCE lens found that this error message doesn't tell the user how to fix the problem"

This creates psychological distance from criticism and lets the author "agree with the lens" without defensiveness.

### Pedanticness Slider

Not every finding is worth raising. Match depth to context:

| Level | What Gets Raised |
|-------|------------------|
| **Low** | Bugs that affect users, security issues, broken promises |
| **Medium** (default) | Above + misleading docs, confusing APIs, performance gaps |
| **High** | Above + style issues, nitpicks, technically-true-but-pedantic |

Default to medium. Raise material issues. Skip the nitpicks unless asked.

### Skip Cost

Single-perspective review catches obvious bugs but misses cross-cutting concerns. The security reviewer doesn't think about onboarding confusion. The UX reviewer doesn't think about injection attacks. Council mode costs 2-3x a single review but catches 3-5x more issues.

## Anti-Patterns

| Pattern | Problem | Better Approach |
|---------|---------|-----------------|
| Rubber-stamping | "LGTM" without reading — misses real issues | Spend real time. If you can't review properly, say so |
| Nitpick avalanche | 30 style comments, zero substantive findings — misses the forest for the trees | Lead with important findings. Nits are optional |
| Adversarial without constructive | "This is wrong" with no path forward — demoralizes without helping | Every criticism should include a suggestion or clear explanation |
| Review scope creep | Reviewing adjacent code that wasn't changed — diffuses focus | Review what changed. File issues for pre-existing problems |
| Asking for perfection | Blocking on minor style preferences — delays delivery without value | Distinguish blocking issues from preferences. Only block on blockers |
| Authority bias | Accepting work from senior contributors without scrutiny | Everyone's code has bugs. Review the code, not the author |

## The Iron Principle

```
THE REVIEWER'S JOB IS TO FIND PROBLEMS, NOT CONFIRM QUALITY
```

**Why:** If you approach a review expecting it to be fine, you'll find it fine. Looking for problems surfaces them while they're cheap to fix.

## Output Format

```markdown
## Adversarial Review: [What Was Reviewed]

**Scope:** [Files, components, or artifacts reviewed]
**Against:** [Requirements, ADRs, or acceptance criteria referenced]

### Findings

#### Blocking
1. [Issue description + location + suggested fix]

#### Important
1. [Issue description + location + suggested fix]

#### Minor
1. [Issue description + location + suggested fix]

#### Nits
1. [Observation]

### Summary
[Overall assessment: ready to merge, needs fixes, needs rethink]
[Confidence level: how thoroughly was this reviewed]
```
