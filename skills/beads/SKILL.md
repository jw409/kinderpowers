---
name: beads
description: Use when work spans multiple sessions, has dependencies or blockers, needs context recovery after compaction, or involves multi-step tasks that benefit from persistent tracking
---

# Beads — Persistent Task Tracking with `bd`

## Overview

Beads survive conversation compaction. When context is lost, `bd ready` recovers it. They replace TodoWrite, TaskCreate, and markdown-file tracking with a git-synced system that persists across sessions, agents, and machines.

**Core principle:** The cost of tracking is low; the cost of lost work is high. Persistence you don't need beats lost context.

## When to Use

**Use beads when:**
- Work spans multiple sessions or conversations
- Tasks have dependencies or blockers
- You need context recovery after compaction or session restart
- Multi-step work needs progress tracking
- Multiple agents coordinate on related tasks

**Skip beads when:**
- Single quick fix, completed in one exchange
- One-shot question or trivial change
- No dependencies, no follow-up needed

**After compaction or new session:** Run `bd ready` first. This is your memory.

## Quick Reference

| Action | Command |
|--------|---------|
| Create task | `bd create --title="..." --description="..." --type=task --priority=2` |
| List ready work | `bd ready` |
| List all open | `bd list --status=open` |
| Show details | `bd show <id>` |
| Claim work | `bd update <id> --status=in_progress` |
| Close one | `bd close <id>` |
| Close with reason | `bd close <id> --reason="explanation"` |
| Batch close | `bd close <id1> <id2> <id3>` |
| Add dependency | `bd dep add <issue> <depends-on>` |
| Show blocked | `bd blocked` |
| Project health | `bd stats` |
| Check integrity | `bd doctor` |
| Sync to remote | `bd sync` |
| Check sync status | `bd sync --status` |

## Creating Tasks

```bash
bd create --title="Wire dispatch into pipeline" \
  --description="Connect UnifiedDispatcher to stage 5 of the learning pipeline" \
  --type=task \
  --priority=2
```

**Priority is numeric, 0-4:**

| Priority | Meaning | Use for |
|----------|---------|---------|
| 0 | Critical | Production down, data loss |
| 1 | High | Blocking other work |
| 2 | Normal | Standard feature/bug work |
| 3 | Low | Nice-to-have improvements |
| 4 | Backlog | Someday/maybe |

Do NOT use "high", "medium", "low" — `bd` expects integers.

**Types:** `task`, `bug`, `feature`

## Session Recovery

When you start a new session or context has been compacted:

```bash
# Step 1: What's ready to work on?
bd ready

# Step 2: Pick a task, review details
bd show <id>

# Step 3: Claim it
bd update <id> --status=in_progress
```

`bd ready` shows tasks with no unresolved blockers — the work you can actually do right now. This is the single most important command for context recovery.

## Working with Dependencies

```bash
# "auth-refactor" depends on "schema-migration" finishing first
bd dep add auth-refactor schema-migration

# What's stuck?
bd blocked

# Full picture of a task's dependency chain
bd show <id>
```

`bd show` displays both "blocked by" (what this task waits on) and "blocks" (what waits on this task). Use this to understand sequencing before starting work.

## Session Close Protocol

Run this sequence at the end of every working session. Order matters — `bd sync` must bracket the commit to capture both the code changes and bead state.

```bash
# 1. See what changed
git status

# 2. Stage work
git add <files>

# 3. Sync bead state to git
bd sync

# 4. Commit everything
git commit -m "feat: wire dispatch into pipeline"

# 5. Final sync (captures commit reference)
bd sync

# 6. Push
git push
```

Skipping `bd sync` before commit means bead state drifts from code state. Skipping it after means the commit reference isn't captured. Both syncs take under a second.

## Common Mistakes

**Using `bd edit` in an agent context**
- `bd edit` opens `$EDITOR`, which blocks non-interactive agents indefinitely
- Use `bd update <id> --field=value` instead for any field changes

**Forgetting to claim work before starting**
- Other agents (or future you) won't know what's in progress
- Always `bd update <id> --status=in_progress` before starting

**Creating beads for trivial work**
- A bead for "fix typo in README" adds overhead without value
- If it fits in one exchange with no follow-up, skip the bead

**Not checking `bd blocked` before starting**
- You might start work that depends on unfinished prerequisites
- `bd ready` already filters for this — use it instead of `bd list`

**Priority as strings**
- `--priority=high` fails silently or errors
- Always use integers: `--priority=0` through `--priority=4`

## Project Health

```bash
# Counts: open, closed, blocked, in-progress
bd stats

# Structural integrity check
bd doctor
```

Run `bd doctor` if beads seem inconsistent (missing dependencies, orphaned references). It detects and reports structural issues.

## When in Doubt

Beads are useful for multi-session work. Skipping tracking can mean lost context later — worth considering upfront. An unnecessary bead costs nothing to close; reconstructing lost context is expensive.
