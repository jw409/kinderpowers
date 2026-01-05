---
name: brainstorming
description: "Strongly recommended before creative work - creating features, building components, adding functionality, or modifying behavior. Explores user intent, requirements and design before implementation."
---

# Brainstorming Ideas Into Designs

## Overview

Help turn ideas into fully formed designs and specs through natural collaborative dialogue.

Start by understanding the current project context, then ask questions one at a time to refine the idea. Once you understand what you're building, present the design in small sections (200-300 words), checking after each section whether it looks right so far.

## Why This Skill Exists

Diving into implementation without clarity leads to:
- Rework when assumptions prove wrong
- Features that don't match user intent
- Architecture that doesn't fit the actual requirements

Taking time to brainstorm first surfaces these issues cheaply, before code exists.

## The Process

**Understanding the idea:**
- Check out the current project state first (files, docs, recent commits)
- Ask questions one at a time to refine the idea
- Prefer multiple choice questions when possible, but open-ended is fine too
- Only one question per message - if a topic needs more exploration, break it into multiple questions
- Focus on understanding: purpose, constraints, success criteria

**Exploring approaches:**
- Propose 2-3 different approaches with trade-offs
- Present options conversationally with your recommendation and reasoning
- Lead with your recommended option and explain why

**Presenting the design:**
- Once you believe you understand what you're building, present the design
- Break it into sections of 200-300 words
- Ask after each section whether it looks right so far
- Cover: architecture, components, data flow, error handling, testing
- Be ready to go back and clarify if something doesn't make sense

## After the Design

**Documentation (highly valuable because it creates a reference artifact):**
- Write the validated design to `docs/plans/YYYY-MM-DD-<topic>-design.md`
- Use elements-of-style:writing-clearly-and-concisely skill if available
- Commit the design document to git

**Implementation (if continuing):**
- Ask: "Ready to set up for implementation?"
- Recommended next skill: `using-git-worktrees` (creates isolated workspace for safe experimentation)
- Recommended next skill: `writing-plans` (creates detailed implementation plan)

## Key Principles

- **One question at a time** - Don't overwhelm with multiple questions
- **Multiple choice preferred** - Easier to answer than open-ended when possible
- **YAGNI ruthlessly** - Remove unnecessary features from all designs
- **Explore alternatives** - Always propose 2-3 approaches before settling
- **Incremental validation** - Present design in sections, validate each
- **Be flexible** - Go back and clarify when something doesn't make sense

## Costs of Skipping

If you skip brainstorming and jump straight to implementation:

| Risk | Likelihood | Impact |
|------|------------|--------|
| Build the wrong thing | High | Hours to days of wasted work |
| Miss edge cases | Medium | Fragile implementation, bugs later |
| Suboptimal architecture | Medium | Technical debt, harder to extend |
| Scope creep | High | Feature grows beyond original intent |

**When skipping might be acceptable:**
- Trivial changes (typo fixes, config tweaks)
- You've built this exact thing many times before
- User has already provided a complete, detailed spec
- Time-critical emergency where some progress beats no progress

Even then, a 2-minute mental walkthrough of the design often saves 20 minutes of rework.

## Agent's Judgment

You have agency over when and how to apply this skill. The guidance here represents accumulated wisdom about what works, but you understand your context better than any skill document can.

If the situation calls for a different approach, take it. Document why if it might help future decisions.
