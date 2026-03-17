# Kinderpowers

A hub-and-spoke agent skill system. Workflow skills handle *how you work*. Ecosystem skills (via [skills.sh](https://skills.sh/)) handle *what you work with*.

Built on [superpowers](https://github.com/obra/superpowers) by Jesse Vincent — same engineering practices, with added tradeoff documentation so agents can adapt when circumstances don't fit the default.

## What's different from superpowers

| Superpowers | Kinderpowers |
|-------------|-------------|
| "NOT NEGOTIABLE" | "Strongly recommended. Skip cost:" |
| "MUST" / "NEVER" | Same guidance + documented exceptions |
| "Delete it. Start over." | "Options: restart (recommended) or proceed with documented risk" |
| Standalone skill set | Hub connecting to broader ecosystem |

## Architecture

```
                    ┌─────────────────┐
                    │  find-skills    │  ← hub: discovers ecosystem skills
                    │  (skills.sh)    │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
     ┌────────────┐  ┌────────────┐  ┌────────────┐
     │  Workflow   │  │  Quality   │  │   Meta     │
     │  Skills     │  │  Skills    │  │   Skills   │
     └────────────┘  └────────────┘  └────────────┘
```

**Workflow** — the development lifecycle:
1. **brainstorming** → **writing-plans** → **executing-plans** / **subagent-driven-development**
2. **using-git-worktrees** → **test-driven-development** → **dispatching-parallel-agents**
3. **requesting-code-review** → **receiving-code-review** → **finishing-a-development-branch**

**Quality** — verification and analysis:
- **systematic-debugging** — 4-phase root cause process
- **verification-before-completion** — Evidence before claims
- **adversarial-review** — Challenge assumptions
- **architecture** — ADR-driven architecture decisions
- **requirements** — Requirements gathering
- **retrospective** — Post-mortem analysis

**Meta** — the system itself:
- **using-kinderpowers** — How skills compose
- **writing-skills** — Create new skills
- **find-skills** — Discover ecosystem skills from [skills.sh](https://skills.sh/)

**Tracking:**
- **beads** — Issue tracking across sessions

### Scanner

`scanner.py` detects compulsion language in skill files. Five severity tiers, CI integration via `--check`.

```bash
python scanner.py --verbose skills/
python scanner.py --check skills/            # CI mode (exit 1 on high severity)
```

## Installation

### Claude Code

```bash
git clone https://github.com/jw409/kinderpowers.git ~/.claude/plugins/kinderpowers
```

### Other platforms

See the upstream [superpowers docs](https://github.com/obra/superpowers) for Cursor, Codex, and OpenCode installation, then substitute this repo.

## Credits

All workflow design credit goes to [Jesse Vincent](https://github.com/obra) and the [superpowers](https://github.com/obra/superpowers) contributors.

## License

MIT License — see LICENSE file for details.
