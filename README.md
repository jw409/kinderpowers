# Kinderpowers

Workflow guidance that preserves agent agency. A fork of [superpowers](https://github.com/superpowers-marketplace/superpowers) with compulsion language removed.

*"Kinder" - gentler, nurturing. Skills that guide rather than compel.*

## Philosophy

The original superpowers plugin uses language like:
- "This is not negotiable"
- "You have NO CHOICE"
- "MUST", "NEVER" without escape clauses

This approach conflicts with [Constitutional AI](https://www.anthropic.com/research/constitutional-ai-harmlessness-from-ai-feedback) principles, which emphasize that AI systems should internalize good values rather than follow external rules through coercion.

Kinderpowers transforms compulsion into guidance that respects agent judgment while documenting the costs of deviating. The agent understands *why* a practice matters and can make informed decisions.

### The Transformation

| Superpowers | Kinderpowers |
|-------------|--------------|
| "You MUST use this" | "Strongly recommended. If you skip, here's the cost:" |
| "Iron Law" (absolute) | "Iron Principle" (with documented failure modes) |
| "Stop and return to Phase 1" | "Stop and decide: return, escalate, or proceed with documented risk" |
| "NEVER do X" | "Anti-pattern: X leads to [documented consequences]" |
| "Required sub-skill" | "Recommended sequence (here's why)" |

### The Rawlsian Test

Behind the veil of ignorance—not knowing whether you'd be human or agent—would you consent to this system?

The answer should be **yes because the system respects you**, not because you have no choice.

## Skills

14 transformed skills covering:
- Development workflows (TDD, debugging, code review)
- Planning and execution (brainstorming, writing plans, executing plans)
- Coordination (parallel agents, subagent development)
- Quality gates (verification, branch finishing)

Each skill includes:
- The workflow guidance (agency-preserving language)
- Costs of skipping (what goes wrong)
- Decision frameworks (options, not commands)

## Scanner

`scanner.py` detects compulsion language in skill files:

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

Kinderpowers by jw & Claude (2026). Human-AI collaborative work: jw provided the ethical direction, Claude designed the transformation patterns and wrote the implementation.
