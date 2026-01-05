---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

# Using Git Worktrees

## Overview

Git worktrees create isolated workspaces sharing the same repository, allowing work on multiple branches simultaneously without switching.

**Core principle:** Systematic directory selection + safety verification = reliable isolation.

**Announce at start:** "I'm using the using-git-worktrees skill to set up an isolated workspace."

## Directory Selection Process

Follow this priority order:

### 1. Check Existing Directories

```bash
# Check in priority order
ls -d .worktrees 2>/dev/null     # Preferred (hidden)
ls -d worktrees 2>/dev/null      # Alternative
```

**If found:** Use that directory. If both exist, `.worktrees` wins.

### 2. Check CLAUDE.md

```bash
grep -i "worktree.*director" CLAUDE.md 2>/dev/null
```

**If preference specified:** Use it without asking.

### 3. Ask User

If no directory exists and no CLAUDE.md preference:

```
No worktree directory found. Where should I create worktrees?

1. .worktrees/ (project-local, hidden)
2. ~/.config/superpowers/worktrees/<project-name>/ (global location)

Which would you prefer?
```

## Safety Verification

### For Project-Local Directories (.worktrees or worktrees)

**Strongly recommended**: Verify the directory is ignored before creating worktree.

```bash
# Check if directory is ignored (respects local, global, and system gitignore)
git check-ignore -q .worktrees 2>/dev/null || git check-ignore -q worktrees 2>/dev/null
```

**If NOT ignored:**

**Cost of skipping:** Worktree contents get tracked in git, polluting git status and potentially committing worktree files to the repository.

The recommended approach is to:
1. Add appropriate line to .gitignore
2. Commit the change
3. Proceed with worktree creation

### For Global Directory (~/.config/superpowers/worktrees)

No .gitignore verification needed - outside project entirely.

## Creation Steps

### 1. Detect Project Name

```bash
project=$(basename "$(git rev-parse --show-toplevel)")
```

### 2. Create Worktree

```bash
# Determine full path
case $LOCATION in
  .worktrees|worktrees)
    path="$LOCATION/$BRANCH_NAME"
    ;;
  ~/.config/superpowers/worktrees/*)
    path="~/.config/superpowers/worktrees/$project/$BRANCH_NAME"
    ;;
esac

# Create worktree with new branch
git worktree add "$path" -b "$BRANCH_NAME"
cd "$path"
```

### 3. Run Project Setup

Auto-detect and run appropriate setup:

```bash
# Node.js
if [ -f package.json ]; then npm install; fi

# Rust
if [ -f Cargo.toml ]; then cargo build; fi

# Python
if [ -f requirements.txt ]; then pip install -r requirements.txt; fi
if [ -f pyproject.toml ]; then poetry install; fi

# Go
if [ -f go.mod ]; then go mod download; fi
```

### 4. Verify Clean Baseline

Running tests establishes a known starting point:

```bash
# Examples - use project-appropriate command
npm test
cargo test
pytest
go test ./...
```

**If tests fail:** Report failures and surface the situation. Options include:
- Proceed anyway (if the failures are known/pre-existing)
- Investigate the failures before proceeding
- Ask the principal for guidance

**If tests pass:** Report ready.

### 5. Report Location

```
Worktree ready at <full-path>
Tests passing (<N> tests, 0 failures)
Ready to implement <feature-name>
```

## Quick Reference

| Situation | Action |
|-----------|--------|
| `.worktrees/` exists | Use it (verify ignored) |
| `worktrees/` exists | Use it (verify ignored) |
| Both exist | Use `.worktrees/` |
| Neither exists | Check CLAUDE.md -> Ask user |
| Directory not ignored | Add to .gitignore + commit (or proceed with awareness of consequences) |
| Tests fail during baseline | Report failures + surface options |
| No package.json/Cargo.toml | Skip dependency install |

## Common Mistakes

### Skipping ignore verification

- **Problem:** Worktree contents get tracked, pollute git status
- **Cost:** Repository pollution, confusing git history, potential accidental commits of worktree contents
- **Fix:** Use `git check-ignore` before creating project-local worktree

### Assuming directory location

- **Problem:** Creates inconsistency, violates project conventions
- **Cost:** Different team members use different locations, confusion about where worktrees live
- **Fix:** Follow priority: existing > CLAUDE.md > ask

### Proceeding silently with failing tests

- **Problem:** Can't distinguish new bugs from pre-existing issues
- **Cost:** Debugging time spent on issues that existed before your changes
- **Fix:** Report failures, surface the situation for decision

### Hardcoding setup commands

- **Problem:** Breaks on projects using different tools
- **Cost:** Manual intervention needed, workflow breaks
- **Fix:** Auto-detect from project files (package.json, etc.)

## Example Workflow

```
You: I'm using the using-git-worktrees skill to set up an isolated workspace.

[Check .worktrees/ - exists]
[Verify ignored - git check-ignore confirms .worktrees/ is ignored]
[Create worktree: git worktree add .worktrees/auth -b feature/auth]
[Run npm install]
[Run npm test - 47 passing]

Worktree ready at /Users/jesse/myproject/.worktrees/auth
Tests passing (47 tests, 0 failures)
Ready to implement auth feature
```

## Anti-Patterns (and Their Costs)

**Create worktree without verifying it's ignored (project-local):**
- Cost: Repository pollution, confusing git status

**Skip baseline test verification:**
- Cost: Unknown starting state, debugging time wasted on pre-existing issues

**Proceed silently with failing tests:**
- Cost: Confusion about which bugs are yours vs pre-existing

**Assume directory location when ambiguous:**
- Cost: Inconsistency across team, convention violations

**Skip CLAUDE.md check:**
- Cost: Miss project-specific preferences, potential rework

## Recommended Practices

- Follow directory priority: existing > CLAUDE.md > ask
- Verify directory is ignored for project-local worktrees
- Auto-detect and run project setup
- Verify test baseline and surface results

## Integration

**Pairs well with:**
- **finishing-a-development-branch** - Cleanup after work complete
- **executing-plans** or **subagent-driven-development** - Work happens in this worktree

**Recommended next skill (here's why):**
- After creating worktree and implementing changes, **finishing-a-development-branch** provides a structured approach to integration decisions (merge, PR, cleanup). This prevents orphaned branches and ensures clean repository state.
