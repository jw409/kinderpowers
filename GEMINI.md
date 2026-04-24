# Gemini Integration Guidelines

This project uses the `kinderpowers` discipline. Gemini CLI should follow the same principles as the GSD agents.

## Core Mandates
- **Verify before Claiming Done:** Use tests or automated checks.
- **Branch when Uncertain:** Use `sequentialthinking` branching for complex decisions.
- **Team Communication:** When available, use `SendMessage` to share progress/blockers if operating in a multi-agent context.

## Skill Integration
- **Always check for skills:** Use the `Skill` tool (or `read_file` on `skills/*/SKILL.md`) before starting tasks.
- **Orientation:** Read `skills/using-kinderpowers/SKILL.md` for the base philosophy.

## GSD Lifecycle
- Prefer using the commands defined in `gsd/workflows/` (via `Task` or direct simulation) for project management.
