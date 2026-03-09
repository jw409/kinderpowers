---
name: retrospective
description: Use after completing an epic, finishing a sprint, resolving a major incident, or at natural project milestones — extracts lessons learned to improve future work
---

# Retrospective

## Overview

A retrospective extracts lessons from completed work so future iterations improve. The goal is not to assign blame or celebrate victory — it's to identify specific, actionable changes to process, tooling, and approach.

**Core principle:** Good outcomes have lessons too. Don't only retro when things go wrong.

**Announce at start:** "I'm using the retrospective skill to extract lessons from this work."

## When to Run

**Strongly recommended after:**
- Completing an epic or major feature
- Finishing a sprint or iteration
- Resolving a major incident or outage
- Any project milestone (v1.0 ship, migration complete, new system launched)
- A project that took significantly longer or shorter than estimated

**Also valuable after:**
- Work that went smoothly — understanding *why* it went well is as useful as understanding failures
- Cancelled or pivoted work — the decision to stop is worth examining
- Multi-agent collaboration — each agent's perspective surfaces blind spots

**Fine to skip for:**
- Individual bug fixes or small tasks (unless they revealed systemic issues)
- Routine, well-established workflows that haven't changed

**Cost of skipping:** Without retrospectives, teams repeat the same mistakes. The lessons exist in everyone's heads but never become shared knowledge or process improvements. A 30-minute retro can save days on the next iteration.

## Structure

Save retrospective output to: `docs/retrospectives/YYYY-MM-DD-<project-or-milestone>.md`

### 1. What Went Well

Practices worth repeating. Be specific — vague praise doesn't help future planning.

**Good:** "TDD caught the authentication bypass bug before deploy. The failing test on line 47 of `test_auth.py` caught a missing permission check that would have exposed admin endpoints."

**Not useful:** "Testing was good."

**Why document successes:** Knowing what works prevents accidentally discarding effective practices. "We stopped doing X and things got worse" is avoidable if X was documented as valuable.

Questions to explore:
- What practices saved time or prevented bugs?
- What tools or techniques worked better than expected?
- Where did estimates match reality? What made those estimates accurate?
- What went smoothly enough that we didn't notice? (Invisible successes are still successes.)

### 2. What Could Improve

Pain points, bottlenecks, and surprises. Focus on process, not people. "The deploy process took 4 hours" is actionable. "Bob was slow" is not.

**Good:** "Context switching between 3 concurrent tasks caused an average 20-minute ramp-up each switch. Batching similar work would reduce this."

**Not useful:** "Things could be faster."

Questions to explore:
- Where did we spend time that didn't produce value?
- What surprised us? What assumptions were wrong?
- Where did we work around tooling instead of with it?
- What information did we wish we had earlier?
- Where did communication break down?

### 3. Action Items

Concrete changes for the next iteration. Each item should be specific and assignable (to a person, a team, or a future task).

**Good action item:**
```
- [ ] Add pre-commit hook for schema validation (prevents the deploy failure from day 3)
      Owner: infrastructure task
      Deadline: before next sprint
```

**Bad action item:**
```
- [ ] Be more careful with deploys
```

**The action item test:** Could someone who wasn't in this retro pick up this action item and execute it? If not, it's too vague.

### 4. Metrics Review

Compare expectations to reality. Numbers surface patterns that narratives miss.

| Metric | Expected | Actual | Notes |
|--------|----------|--------|-------|
| Duration | 2 weeks | 3 weeks | Auth integration took 5 days vs estimated 2 |
| Task count | 12 tasks | 15 tasks | 3 unplanned tasks from edge cases |
| Rework rate | — | 20% | 3 of 15 tasks required significant rework |
| Test coverage (new code) | 80% | 73% | Integration tests deferred to follow-up |

**Estimation calibration:** If estimates were consistently off, examine why. Common causes:
- Optimism bias (everything takes longer than you think)
- Hidden dependencies (task B blocked on task A's output format)
- Scope discovery (requirements emerged during implementation)
- Context switching (calendar time vs focused time)

Track estimation accuracy over multiple retros to calibrate.

## Multi-Agent Retrospective

When multiple agents worked on the project, each contributes their perspective. Disagreement is valuable — it surfaces blind spots.

**Process:**
1. Each agent independently answers: what went well, what could improve, surprises
2. Collect perspectives and identify themes
3. Note where agents disagree — these are the most interesting findings
4. Synthesize into unified action items

**Why disagreement matters:** If Agent A thinks the deployment process was fine and Agent B found it painful, that's a signal — the process works for some workflows but not others. Both perspectives are true. The retro should capture both.

**For agent handoffs:** If one agent started work and another finished it, the handoff itself is worth examining. Was context preserved? What was lost in translation? How could the handoff be smoother?

## Connection to Learning

Retrospective findings should feed forward, not just sit in a document.

| Finding Type | Feeds Into |
|-------------|-----------|
| Process improvement | Updated workflows, new automation, changed practices |
| Estimation calibration | Future task sizing, sprint planning |
| Tool effectiveness | Tool adoption/retirement decisions |
| Communication gaps | Updated documentation, clearer handoff protocols |
| Agent memory | Session recovery, future task context |

**The feedback loop:** Retro findings become inputs for future planning sessions. When starting a new sprint or epic, review recent retros: "Last time we underestimated integration work by 2x — account for that."

## Anti-Patterns

| Pattern | Problem | Better Approach |
|---------|---------|-----------------|
| Blame-focused retros | People get defensive, hide mistakes, retro becomes punishment | Focus on process and systems, not individuals. "The deploy process allowed X" not "Person Y caused X" |
| Retros without action items | Feels productive but changes nothing — same problems recur | Every retro produces at least one concrete, assignable action item |
| Skipping retros when things went well | Miss the chance to understand and replicate success | Good outcomes are just as instructive. "Why did this go well?" prevents accidentally losing good practices |
| Retro fatigue | Doing retros so frequently they feel empty | Match retro frequency to work cadence. After milestones, not after every task |
| Vague findings | "Communication could be better" — true but useless | Force specificity: what communication, between whom, about what, when |
| Action items without owners | Everyone's responsibility is no one's responsibility | Every action item gets an owner and a deadline (even if the deadline is "before next sprint") |

## The Iron Principle

```
EXTRACT SPECIFIC LESSONS AND TURN THEM INTO CONCRETE CHANGES
```

**Why:** Experience without reflection is just repetition. Retrospectives convert experience into improvement by forcing explicit examination of what happened, why, and what to do differently. The difference between a team that improves and one that doesn't is whether lessons get captured and acted on.

**Cost of ignoring:** Same mistakes, different sprint. The team works harder but not smarter. Problems feel familiar because they are — they were never addressed, just survived.

## Output Format

```markdown
# Retrospective: [Project/Milestone Name]

**Date:** YYYY-MM-DD
**Scope:** [What work is being retrospected]
**Participants:** [Who contributed to this retro]
**Duration:** [How long the work took vs estimate]

## What Went Well
1. [Specific practice + evidence of value]
2. [Specific practice + evidence of value]

## What Could Improve
1. [Specific pain point + impact]
2. [Specific pain point + impact]

## Action Items
- [ ] [Specific action] — Owner: [who], Deadline: [when]
- [ ] [Specific action] — Owner: [who], Deadline: [when]

## Metrics
| Metric | Expected | Actual | Notes |
|--------|----------|--------|-------|
| ...    | ...      | ...    | ...   |

## Key Takeaways
[2-3 sentence summary of the most important lessons]
```
