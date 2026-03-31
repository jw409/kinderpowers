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

## Batch Mode — Creating Beads from Plans

When creating multiple beads from a plan document, use batch mode to auto-enrich in a single pass. This replaces the manual 5-step workflow (create → deps → labels → robot-suggest → acceptance criteria).

**CRITICAL**: Dolt is single-writer. NEVER parallelize `bd create`/`bd update`/`bd dep add`. Chain all writes with `&&`.

### Workflow

```bash
# 1. Create all beads serially (from your plan)
bd create --title="Phase 1: schema migration" --description="..." --type=task --priority=2 && \
bd create --title="Phase 2: API endpoints" --description="..." --type=feature --priority=2 && \
bd create --title="Phase 3: integration tests" --description="..." --type=task --priority=2

# 2. Wire to active epic (if one exists)
EPIC=$(bd list --status=open --type=epic --limit=1 --format=id 2>/dev/null)
if [ -n "$EPIC" ]; then
  bd dep add <new-id-1> $EPIC && \
  bd dep add <new-id-2> $EPIC && \
  bd dep add <new-id-3> $EPIC
fi

# 3. Add inter-phase dependencies (later phases depend on earlier)
bd dep add <phase-2-id> <phase-1-id> && \
bd dep add <phase-3-id> <phase-2-id>

# 4. Auto-label from title/description keywords
bd label add <id> gpu       # if title mentions GPU/CUDA
bd label add <id> dashboard # if title mentions UI/dashboard
bd label add <id> test      # if type=task and title mentions test

# 5. Run robot suggestions (apply high-confidence, surface rest)
bv --robot-suggest
# Apply suggestions with confidence > 0.9 automatically
# Surface lower-confidence suggestions for human review

# 6. Flag beads missing acceptance criteria
bd lint  # reports beads without --acceptance set
```

### Auto-enrichment Checklist

When creating beads from a plan, the agent should:

- [ ] Create all beads serially (never parallel `bd create`)
- [ ] Wire each bead to the active epic via `bd dep add`
- [ ] Infer inter-phase dependencies from plan ordering and cross-references
- [ ] Apply labels based on domain keywords in title/description
- [ ] Run `bv --robot-suggest` and auto-apply high-confidence (>0.9) dep suggestions
- [ ] Run `bd lint` to flag beads missing acceptance criteria
- [ ] Report a summary: N created, N deps added, N labels applied, N missing AC

### Keywords → Labels Mapping

| Keywords in title/description | Label |
|-------------------------------|-------|
| GPU, CUDA, model, inference | `gpu` |
| dashboard, UI, frontend, page | `dashboard` |
| test, spec, coverage, TDD | `test` |
| API, endpoint, route, handler | `api` |
| migration, schema, database | `database` |
| deploy, CI, pipeline, release | `infra` |
| security, auth, token, encrypt | `security` |

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
