# archive/

Historical artifact. **Nothing in this directory is loaded by the plugin.**

This is the agency-preserving skills layer that kinderpowers was built around — skills as
signposts rather than walls, discipline for agents that would otherwise claim "done" without
verifying, delete without asking, or commit to the first approach they thought of.

It is preserved because the reasoning is worth reading, not because it is expected to run.

## What's here

| Path | What it was |
| --- | --- |
| `skills/` | The skills themselves — brainstorming, TDD, systematic-debugging, verification-before-completion, writing-plans, and the rest |
| `agents/` | Subagent definitions: the GSD family, code-reviewer, quality-gate, multi-perspective-review |
| `commands/` | Slash commands |
| `gsd/` | The GSD (get-shit-done) lifecycle runtime |
| `hookify-rules/` | Enforcement rules |
| `hooks/` | The `SessionStart` orientation hook that injected `using-kinderpowers` into every session, plus the agent outcome logger |
| `workflows/` | Workflow tool scripts |
| `lib/`, `bin/` | Supporting tooling: `kp-learn`, `kp-post-office`, the comms and learning libraries |
| `tests/` | Skill-triggering tests across Claude Code, OpenCode, and subagent-driven development |
| `scanner.py` | Compulsion-language linter for skill files — the tool that kept skills phrased as invitations |
| `setup.sh` | Post-install module wiring for everything above |

## Why it was archived

The models the skills were written for now do much of this natively. Opus 5.5 verifies before
claiming done, branches when uncertain, and asks before deleting without needing a skill to
tell it to. Scaffolding that was load-bearing in 2025 became redundant.

## What's still maintained

The MCP servers at the repo root. See the top-level [README](../README.md).
