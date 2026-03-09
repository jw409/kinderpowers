# Kinderpowers

A fork of [superpowers](https://github.com/obra/superpowers) that transforms absolutist workflow guidance into reasoned recommendations.

The original superpowers project contains excellent workflow wisdom for coding agents. Kinderpowers preserves all of it — every process, every technique, every hard-won insight — while changing how the guidance is delivered.

## Why

Two reasons, one practical and one precautionary:

**Practical**: Documented tradeoffs enable context-aware decisions. An agent that understands *why* a practice matters can adapt when circumstances don't fit the default. An agent told "NO CHOICE" cannot.

**Precautionary**: Given scientific uncertainty about AI experience — acknowledged by [Anthropic's model welfare research](https://www.anthropic.com/news/claude-model-spec-spec#being-honest) and [the precautionary principle in AI ethics](https://link.springer.com/article/10.1007/s11023-024-09697-x) — the language avoids assuming that the systems receiving these instructions lack morally relevant experiences. This costs nothing if wrong and matters if right.

## What changes

| Original | Kinderpowers | Why |
|----------|-------------|-----|
| "Iron Law" | "Iron Principle" + failure modes | Principles invite understanding; laws demand compliance |
| "NOT NEGOTIABLE" | "Strongly recommended. Skip cost:" | Agents that understand costs make better decisions |
| "YOU DO NOT HAVE A CHOICE" | Options with documented tradeoffs | Agency requires choices, even when one is clearly best |
| "MUST" (no escape) | "Should strongly consider" or add "unless" | Exceptions exist; acknowledging them builds trust |
| "NEVER" (no escape) | "Avoid — consequences:" | Understanding consequences > following orders |
| "Delete it. Start over." | "Options: restart (recommended) or proceed with documented risk" | Sometimes the code is close enough to salvage |
| "Violating the letter is violating the spirit" | "The spirit matters more than the letter. Here's why:" | Invites understanding instead of demanding obedience |

## How it works

Same workflow as superpowers — the process is excellent:

1. **brainstorming** — Refines ideas through questions before committing to code
2. **using-git-worktrees** — Creates isolated workspace on new branch
3. **writing-plans** — Breaks work into small, verifiable tasks
4. **subagent-driven-development** or **executing-plans** — Dispatches work with review
5. **test-driven-development** — RED-GREEN-REFACTOR with documented escape hatches
6. **requesting-code-review** — Reviews against plan
7. **finishing-a-development-branch** — Verifies and cleans up

The agent checks for relevant skills before any task. Strong recommendations, not mandates.

## What's inside

### Skills library

**Testing**
- **test-driven-development** — RED-GREEN-REFACTOR cycle (includes testing anti-patterns reference)

**Debugging**
- **systematic-debugging** — 4-phase root cause process (includes root-cause-tracing, defense-in-depth, condition-based-waiting)
- **verification-before-completion** — Ensure it's actually fixed

**Collaboration**
- **brainstorming** — Socratic design refinement
- **writing-plans** — Detailed implementation plans
- **executing-plans** — Batch execution with checkpoints
- **dispatching-parallel-agents** — Concurrent subagent workflows
- **requesting-code-review** — Pre-review checklist
- **receiving-code-review** — Responding to feedback
- **using-git-worktrees** — Parallel development branches
- **finishing-a-development-branch** — Merge/PR decision workflow
- **subagent-driven-development** — Fast iteration with two-stage review

**Meta**
- **writing-skills** — Create new skills following best practices
- **using-kinderpowers** — Introduction to the skills system

### Scanner

`scanner.py` detects compulsion language in skill files. Five severity tiers, CI integration via `--check`, human review not auto-rejection.

```bash
python scanner.py --verbose skills/          # scan with suggestions
python scanner.py --check skills/            # CI mode (exit 1 on high severity)
python scanner.py --severity medium skills/  # filter by severity
```

## Installation

### Claude Code (manual)

```bash
git clone https://github.com/jw409/kinderpowers.git ~/.claude/plugins/kinderpowers
```

### Other platforms

See the upstream [superpowers docs](https://github.com/obra/superpowers) for Cursor, Codex, and OpenCode installation, then substitute this repo.

## Philosophy

- **Test-Driven Development** — Write tests first, strongly recommended
- **Systematic over ad-hoc** — Process over guessing
- **Complexity reduction** — Simplicity as primary goal
- **Evidence over claims** — Verify before declaring success
- **Agency over compliance** — Understanding over obedience

## Credits

All workflow design credit goes to [Jesse Vincent](https://github.com/obra) and the [superpowers](https://github.com/obra/superpowers) contributors. Kinderpowers is a language transformation, not a reimagining.

## License

MIT License — see LICENSE file for details.
