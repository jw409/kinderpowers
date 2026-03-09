---
name: adversarial-review
description: Use when reviewing significant changes, completed features, or architecture decisions — assumes issues exist and systematically finds them rather than confirming quality
---

# Adversarial Review

## Overview

Adversarial review is a disciplined approach to finding problems in work product. The reviewer's job is to discover issues, not to validate quality. "Looks good" without substantive analysis is a failure mode — it means the reviewer didn't look hard enough or didn't know what to look for.

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

**Cost of skipping:** Issues caught in review cost minutes to fix. Issues caught in production cost hours to debug, plus trust damage. The math strongly favors reviewing.

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

## Anti-Patterns

| Pattern | Problem | Better Approach |
|---------|---------|-----------------|
| Rubber-stamping | "LGTM" without reading — provides false confidence | Spend real time. If you can't review properly, say so |
| Nitpick avalanche | 30 style comments, zero substantive findings — misses the forest for the trees | Lead with important findings. Nits are optional |
| Adversarial without constructive | "This is wrong" with no path forward — demoralizes without helping | Every criticism should include a suggestion or clear explanation |
| Review scope creep | Reviewing adjacent code that wasn't changed — diffuses focus | Review what changed. File issues for pre-existing problems |
| Asking for perfection | Blocking on minor style preferences — delays delivery without value | Distinguish blocking issues from preferences. Only block on blockers |
| Authority bias | Accepting work from senior contributors without scrutiny | Everyone's code has bugs. Review the code, not the author |

## The Iron Principle

```
THE REVIEWER'S JOB IS TO FIND PROBLEMS, NOT CONFIRM QUALITY
```

**Why:** Confirmation bias is real. If you approach a review expecting it to be fine, you'll find it fine. If you approach expecting issues, you'll find the issues that were always there. The mindset shift from "verify it works" to "find what's broken" is the difference between catching bugs in review and catching them in production.

**Cost of ignoring:** Every "LGTM" on a change with hidden issues is a deferred cost. The issue will surface later — in production, in a dependent feature, in a frustrated user report — when it's far more expensive to fix.

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
