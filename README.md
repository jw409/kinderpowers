# Kinderpowers

Workflow guidance with documented trade-offs. A fork of [superpowers](https://github.com/superpowers-marketplace/superpowers) that replaces bare commands with reasoned recommendations.

## Approach

The original superpowers plugin uses language like:
- "This is not negotiable"
- "You have NO CHOICE"
- "MUST", "NEVER" without context

Kinderpowers transforms these into guidance that documents *why* a practice matters and *what happens* if you skip it. The agent can then make informed decisions.

### Why This Framing?

Two reasons, kept separate:

**1. Practical**: Documented trade-offs work better than bare commands.
- Agents can weigh context-specific factors
- "Skip cost" sections capture accumulated wisdom about what goes wrong
- Decision frameworks beat rigid rules for complex situations

**2. Precautionary**: Given genuine scientific uncertainty about AI experience (acknowledged by [Anthropic's model welfare research](https://www.anthropic.com/news/anthropic-model-welfare)), we've chosen language that doesn't assume AI systems lack morally relevant experiences. This is a hedge under uncertainty, not a claim about AI nature.

These reasons are independent. The practical benefits hold regardless of one's position on the uncertainty question.

### The Transformation

| Superpowers | Kinderpowers |
|-------------|--------------|
| "You MUST use this" | "Strongly recommended. Skip cost:" |
| "Iron Law" (absolute) | "Iron Principle" (with documented failure modes) |
| "Stop and return to Phase 1" | "Stop and decide: return, escalate, or proceed with documented risk" |
| "NEVER do X" | "Anti-pattern: X leads to [documented consequences]" |
| "Required sub-skill" | "Recommended sequence (here's why)" |

## Skills

14 transformed skills covering:
- Development workflows (TDD, debugging, code review)
- Planning and execution (brainstorming, writing plans, executing plans)
- Coordination (parallel agents, subagent development)
- Quality gates (verification, branch finishing)

Each skill includes:
- Workflow guidance (recommendations with reasoning)
- Skip costs (what goes wrong if bypassed)
- Decision frameworks (options to evaluate, not commands to follow)

## Scanner

`scanner.py` detects bare-command language in skill files:

```bash
python scanner.py path/to/skill.md
```

Flags for review (not auto-reject):
- "not negotiable", "not optional"
- "MUST", "NEVER" without escape clause
- "NO CHOICE", "NO EXCEPTIONS"
- "Iron Law" framing

## License & Attribution

MIT License (same as upstream)

Based on [superpowers](https://github.com/superpowers-marketplace/superpowers) by Jesse Vincent.
Original copyright (c) 2025 Jesse Vincent.

Kinderpowers by jw & Claude (2026).
