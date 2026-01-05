---
name: finishing-a-development-branch
description: Use when implementation is complete, all tests pass, and you need to decide how to integrate the work - guides completion of development work by presenting structured options for merge, PR, or cleanup
---

# Finishing a Development Branch

## Overview

Guide completion of development work by presenting clear options and handling chosen workflow.

**Core principle:** Verify tests → Present options → Execute choice → Clean up.

**Announce at start:** "I'm using the finishing-a-development-branch skill to complete this work."

## The Process

### Step 1: Verify Tests

**Before presenting options, strongly recommended to verify tests pass:**

```bash
# Run project's test suite
npm test / cargo test / pytest / go test ./...
```

**Why this matters:** Merging or creating a PR with failing tests creates rework, blocks CI pipelines, and may break other developers' workflows.

**If tests fail:**
```
Tests failing (<N> failures). Recommended to fix before completing:

[Show failures]

Proceeding with merge/PR while tests fail may cause:
- CI blocking the merge anyway
- Other developers pulling broken code
- Reviewer time wasted on known issues
```

Consider stopping here and fixing tests first.

**If tests pass:** Continue to Step 2.

### Step 2: Determine Base Branch

```bash
# Try common base branches
git merge-base HEAD main 2>/dev/null || git merge-base HEAD master 2>/dev/null
```

Or ask: "This branch split from main - is that correct?"

### Step 3: Present Options

Present exactly these 4 options:

```
Implementation complete. What would you like to do?

1. Merge back to <base-branch> locally
2. Push and create a Pull Request
3. Keep the branch as-is (I'll handle it later)
4. Discard this work

Which option?
```

**Structured options are highly valuable because** they prevent ambiguity and ensure clear communication of intent.

### Step 4: Execute Choice

#### Option 1: Merge Locally

```bash
# Switch to base branch
git checkout <base-branch>

# Pull latest
git pull

# Merge feature branch
git merge <feature-branch>

# Verify tests on merged result
<test command>

# If tests pass
git branch -d <feature-branch>
```

Then: Cleanup worktree (Step 5)

#### Option 2: Push and Create PR

```bash
# Push branch
git push -u origin <feature-branch>

# Create PR
gh pr create --title "<title>" --body "$(cat <<'EOF'
## Summary
<2-3 bullets of what changed>

## Test Plan
- [ ] <verification steps>
EOF
)"
```

Then: Cleanup worktree (Step 5)

#### Option 3: Keep As-Is

Report: "Keeping branch <name>. Worktree preserved at <path>."

Leave worktree in place.

#### Option 4: Discard

**Confirmation is highly valuable because** deleted work cannot be recovered from git once the branch is gone:

```
This will permanently delete:
- Branch <name>
- All commits: <commit-list>
- Worktree at <path>

Type 'discard' to confirm.
```

Wait for explicit confirmation.

If confirmed:
```bash
git checkout <base-branch>
git branch -D <feature-branch>
```

Then: Cleanup worktree (Step 5)

### Step 5: Cleanup Worktree

**For Options 1, 2, 4:**

Check if in worktree:
```bash
git worktree list | grep $(git branch --show-current)
```

If yes:
```bash
git worktree remove <worktree-path>
```

**For Option 3:** Keep worktree.

## Quick Reference

| Option | Merge | Push | Keep Worktree | Cleanup Branch |
|--------|-------|------|---------------|----------------|
| 1. Merge locally | ✓ | - | - | ✓ |
| 2. Create PR | - | ✓ | ✓ | - |
| 3. Keep as-is | - | - | ✓ | - |
| 4. Discard | - | - | - | ✓ (force) |

## Anti-patterns to Watch

**Skipping test verification**
- **What happens:** Merge broken code, create failing PR, block CI
- **Cost:** Rework, reviewer frustration, potential rollback
- **Alternative:** Run tests first - 30 seconds now saves hours later

**Open-ended questions**
- **What happens:** "What should I do next?" → ambiguous responses, miscommunication
- **Cost:** Extra round-trips, potential wrong action
- **Alternative:** Present 4 structured options

**Automatic worktree cleanup on Option 2/3**
- **What happens:** Remove worktree when PR still needs edits or work continues
- **Cost:** Need to recreate worktree, lose local state
- **Alternative:** Only cleanup for Options 1 and 4

**No confirmation for discard**
- **What happens:** Accidentally delete work
- **Cost:** Permanent loss of commits, potentially days of work
- **Alternative:** Require typed "discard" confirmation

## Costs of Skipping This Skill

| Skipped Step | Consequence |
|--------------|-------------|
| Test verification | Broken code merged, CI failures, teammate breakage |
| Base branch check | Merge into wrong branch, release confusion |
| Structured options | Miscommunication, wrong action taken |
| Discard confirmation | Permanent work loss |
| Worktree cleanup | Orphaned worktrees accumulate, disk space waste |

## Guidelines

**Avoid:**
- Proceeding with failing tests (CI will likely block anyway)
- Merging without verifying tests on result (merge conflicts can break things)
- Deleting work without confirmation (no recovery possible)
- Force-pushing without explicit request (can lose others' commits)

**Strongly recommended:**
- Verify tests before offering options
- Present exactly 4 options (clarity)
- Get typed confirmation for Option 4
- Clean up worktree for Options 1 & 4 only

## Integration

**Recommended next skill (here's why):**
- **using-git-worktrees** - If starting new work, isolate it cleanly
- **verification-before-completion** - If uncertain about test coverage

**Often called after:**
- **subagent-driven-development** (Step 7) - After all tasks complete
- **executing-plans** (Step 5) - After all batches complete
