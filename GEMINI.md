# Gemini Integration Guidelines

This repo ships two MCP servers. The skills layer it used to ship is archived under
[`archive/`](archive/) and is no longer loaded — see the [README](README.md).

## Core Mandates
- **Verify before Claiming Done:** Use tests or automated checks.
- **Branch when Uncertain:** Use `stepwise_plan` branching for complex decisions.
- **Team Communication:** When available, use `SendMessage` to share progress/blockers if operating in a multi-agent context.

## MCP servers
- **kp-stepwise** — `stepwise_plan` for structured planning. Every call needs a `channelId`;
  parallel agents must use different ones. State is isolated per `(roomId, channelId)`.
- **kp-github** — compressed GitHub reads. Every read tool accepts `fields` and `format`.

## Archived
The skills, agents, commands, and the GSD lifecycle runtime live in `archive/` as a
historical artifact. Don't read them for operating instructions — they target an older
generation of models.
