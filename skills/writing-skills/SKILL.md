---
name: writing-skills
description: Use when creating new skills, editing existing skills, or verifying skills work before deployment
---

# Writing Skills

## Overview

**Writing skills is TDD applied to process documentation.**

**Personal skills live in agent-specific directories (`~/.claude/skills` for Claude Code, `~/.agents/skills/` for Codex)**

You write test cases (pressure scenarios with subagents), watch them fail (baseline behavior), write the skill, watch tests pass, and refactor (close gaps).

**Core principle:** If you didn't watch an agent work without the skill, you don't know what the skill needs to address.

**Background:** Understanding kinderpowers:test-driven-development helps — this skill adapts TDD to documentation.

**Official guidance:** See anthropic-best-practices.md for Anthropic's skill authoring patterns.

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `validation_level` | full | quick, standard, full | Quick=structure check only, standard=baseline test, full=RED-GREEN-REFACTOR with pressure scenarios |
| `template_style` | technique | technique, pattern, reference | Which SKILL.md structure to use — technique=steps, pattern=mental model, reference=API docs |
| `auto_test` | true | true/false | Whether to automatically run pressure scenarios against the skill before deployment |
| `keyword_coverage` | standard | minimal, standard, thorough | How many search keywords to embed — minimal=core terms, thorough=errors+symptoms+synonyms |

## What is a Skill?

A reusable reference guide for proven techniques, patterns, or tools.

**Skills are:** Reusable techniques, patterns, tools, reference guides

**Skills are NOT:** Narratives about how you solved a problem once

## TDD Mapping for Skills

| TDD Concept | Skill Creation |
|-------------|----------------|
| **Test case** | Pressure scenario with subagent |
| **Production code** | Skill document (SKILL.md) |
| **Test fails (RED)** | Agent does it wrong without skill (baseline) |
| **Test passes (GREEN)** | Agent follows skill correctly |
| **Refactor** | Close gaps while maintaining correctness |

## When to Create a Skill

**Create when:**
- Technique wasn't intuitively obvious
- You'd reference this again across projects
- Pattern applies broadly (not project-specific)
- Others would benefit

**Don't create for:**
- One-off solutions
- Standard practices well-documented elsewhere
- Project-specific conventions (put in CLAUDE.md)
- Mechanical constraints (automate with regex/validation instead)

## Skill Types

### Technique
Concrete method with steps (condition-based-waiting, root-cause-tracing)

### Pattern
Way of thinking about problems (flatten-with-flags, test-invariants)

### Reference
API docs, syntax guides, tool documentation

## Directory Structure

```
skills/
  skill-name/
    SKILL.md              # Main reference (required)
    supporting-file.*     # Only if needed
```

**Flat namespace** — all skills in one searchable namespace

**Separate files for:**
1. Heavy reference (100+ lines)
2. Reusable tools — scripts, utilities, templates

**Keep inline:** Principles, code patterns (< 50 lines), everything else

## SKILL.md Structure

**Frontmatter (YAML):**
- Only two fields: `name` and `description`
- Max 1024 characters total
- `name`: Letters, numbers, hyphens only
- `description`: Third-person, "Use when..." — triggering conditions only

**Important:** Don't summarize the skill's workflow in the description. Testing showed agents follow description shortcuts instead of reading the full skill. Keep the description focused on *when to use*, not *what it does*.

```yaml
# ❌ BAD: Summarizes workflow
description: Use when executing plans - dispatches subagent per task with code review between tasks

# ✅ GOOD: Just triggering conditions
description: Use when executing implementation plans with independent tasks in the current session
```

```markdown
---
name: Skill-Name-With-Hyphens
description: Use when [specific triggering conditions and symptoms]
---

# Skill Name

## Overview
What is this? Core principle in 1-2 sentences.

## When to Use
Bullet list with SYMPTOMS and use cases
When NOT to use

## Core Pattern (for techniques/patterns)
Before/after code comparison

## Quick Reference
Table or bullets for scanning

## Implementation
Inline code for simple patterns
Link to file for heavy reference

## Common Mistakes
What goes wrong + fixes
```

### Keyword Coverage

Use words agents would search for:
- Error messages: "Hook timed out", "ENOTEMPTY", "race condition"
- Symptoms: "flaky", "hanging", "zombie", "pollution"
- Synonyms: "timeout/hang/freeze", "cleanup/teardown/afterEach"
- Tools: Actual commands, library names, file types

### Token Efficiency

**Target word counts:**
- Getting-started workflows: <150 words
- Frequently-loaded skills: <200 words
- Other skills: <500 words

**Techniques:**
- Move details to `--help` references
- Use cross-references instead of repeating content
- Compress examples
- Don't repeat what's in cross-referenced skills

### Naming

**Active voice, verb-first:**
- ✅ `condition-based-waiting` not `async-test-helpers`
- ✅ `creating-skills` not `skill-creation`

### Cross-Referencing

Use skill name with relationship markers:
- Good: `**Recommended:** Use kinderpowers:test-driven-development`
- Bad: `@skills/testing/test-driven-development/SKILL.md` (force-loads, burns context)

## Flowchart Usage

**Use flowcharts ONLY for:**
- Non-obvious decision points
- Process loops where you might stop too early
- "When to use A vs B" decisions

**Avoid for:** Reference material, code examples, linear instructions

See @graphviz-conventions.dot for style rules.

## Code Examples

**One excellent example beats many mediocre ones.** Complete, runnable, well-commented, from a real scenario.

## RED-GREEN-REFACTOR for Skills

### RED: Baseline Test

Run pressure scenario with subagent WITHOUT the skill. Document:
- What choices they made
- What rationalizations they used (verbatim)
- Which pressures triggered problems

### GREEN: Write Minimal Skill

Address the specific issues found in baseline. Don't add content for hypothetical cases.

Run same scenarios WITH skill. Agent should now follow the guidance.

### REFACTOR: Close Gaps

Agent found a new way around the guidance? Address it. Re-test.

**Testing methodology:** See @testing-skills-with-subagents.md for pressure scenario design.

## Testing Approaches by Skill Type

| Skill Type | Test With | Success Criteria |
|------------|-----------|-----------------|
| **Discipline** (TDD, verification) | Pressure scenarios, combined pressures | Agent follows guidance under pressure |
| **Technique** (how-to guides) | Application + edge case scenarios | Agent applies technique correctly |
| **Pattern** (mental models) | Recognition + counter-example scenarios | Agent knows when to apply and when not to |
| **Reference** (documentation) | Retrieval + application scenarios | Agent finds and uses information correctly |

## Anti-Patterns

### ❌ Narrative Example
"In session 2025-10-03, we found empty projectDir caused..."
**Why bad:** Too specific, not reusable

### ❌ Multi-Language Dilution
example-js.js, example-py.py, example-go.go
**Why bad:** Mediocre quality, maintenance burden

### ❌ Code in Flowcharts
**Why bad:** Can't copy-paste, hard to read

## Deployment

**Test each skill before moving to the next.** Batching untested skills multiplies the risk of issues.

## Skill Creation Checklist (TDD Adapted)

**RED Phase — Baseline:**
- [ ] Create pressure scenarios (3+ combined pressures for discipline skills)
- [ ] Run scenarios WITHOUT skill — document baseline behavior verbatim
- [ ] Identify patterns in failures

**GREEN Phase — Write Skill:**
- [ ] Name: letters, numbers, hyphens only
- [ ] YAML frontmatter with name and description (max 1024 chars)
- [ ] Description starts with "Use when..." — triggering conditions only
- [ ] Keywords throughout for search
- [ ] Clear overview with core principle
- [ ] Addresses specific baseline failures
- [ ] One excellent example
- [ ] Run scenarios WITH skill — verify improvement

**REFACTOR Phase — Close Gaps:**
- [ ] Identify new failure modes from testing
- [ ] Address them explicitly
- [ ] Re-test

**Quality:**
- [ ] Flowchart only if decision non-obvious
- [ ] Quick reference table
- [ ] Common mistakes section
- [ ] No narrative storytelling
- [ ] Supporting files only for tools or heavy reference

**Deploy:**
- [ ] Commit and push
- [ ] Consider contributing via PR if broadly useful
