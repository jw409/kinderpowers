---
name: worktree-census
description: Use when git worktrees have accumulated and you need to know which are still relevant — merged, stale, drifted, or live. Joins local worktree state against remote PR and branch status (via kp-github) and proposes safe cleanup. Pairs with using-git-worktrees and finishing-a-development-branch.
---

# Worktree Census

## Overview

Worktrees pile up: feature branches, rebase attempts, and agent-spawned trees (`.claude/worktrees/*`) that were never cleaned up. After a while nobody knows which are still live and which are dead weight — and the guess is usually wrong, because the truth lives on the remote (is the PR merged? is the branch behind main?), not on disk.

This skill answers "is this worktree still relevant?" by joining each worktree's **local** state (branch + HEAD sha) against its **remote** state (PR status + branch-vs-base status), then proposing cleanup. It reads the remote through kp-github's compressed tools so a census over dozens of worktrees across several repos fits in one context.

**Core principle:** Discover locally → classify against remote truth → propose removal, never auto-remove.

**Announce at start:** "I'm using the worktree-census skill to audit which worktrees are still relevant."

## Why kp-github

`github_prs_list` at ~15× compression lets you pull PR state for a whole repo in one cheap call, and `github_branch_status` returns a bounded merged/ahead/behind verdict (~150 tokens) instead of a full compare. At 50+ worktrees that is the difference between fitting the census in context and not. If kp-github is unavailable, the same joins work with `gh pr list --json headRefName,headRefOid,state,mergedAt` and `gh api .../compare/...` — just heavier.

## The Process

### Step 1: Discover worktrees (local, free)

```bash
git worktree list --porcelain
```

Parse each block into `{path, branch, head_sha}`. Blocks with no `branch` line are detached HEADs. If the repo has submodules that also carry worktrees, repeat per submodule. Separate out **harness worktrees** — paths under `.claude/worktrees/` or branches like `agent-*` / `wf_*`. These are ephemeral (spawned by the agent runtime) and get their own disposition; don't treat them like feature branches.

### Step 2: Resolve repo identity (local, free)

```bash
git remote get-url origin      # -> host + owner/repo, for any host
```

Group worktrees by their repo. You'll make one bulk remote call per distinct repo, not one per worktree.

### Step 3: Pull remote PR state (one cheap call per repo)

```
github_prs_list  owner=<o> repo=<r> state=all limit=<n>
                 fields=["number","head_ref","head_sha","state","merged_at","draft"]
```

Build a map `branch -> {number, head_sha, state, merged_at}`. This one call covers every worktree branch in the repo.

### Step 4: Classify each worktree

Join local `{branch, head_sha}` against the PR map. Fall through in order:

| Signal | Disposition | Meaning |
|--------|-------------|---------|
| PR `merged_at` set | **MERGED** | Done — worktree is dead weight. Removable. |
| PR `state=closed`, not merged | **CLOSED** | Abandoned PR — likely dead. Confirm. |
| PR open, `head_sha == local HEAD` | **LIVE** | In sync with its PR. Leave alone. |
| PR open, `head_sha != local HEAD` | **DRIFTED** | Local is ahead of / behind the PR (unpushed work, or PR advanced). Needs a look. |
| no PR row | → Step 4b | resolve via branch status |
| harness worktree | **EPHEMERAL** | Runtime-managed; remove if its task is finished. |

**Step 4b — no PR for this branch.** A missing PR doesn't mean "in progress" — it may be merged without a surviving PR ref, or merged by squash. Resolve:

```
github_branch_status  owner=<o> repo=<r> branch=<b>      # base defaults to default branch
```

- `merged_into_base: true` → **MERGED** (fast-forward / rebase / merge-commit merge; the base already contains this branch).
- `merged_into_base: false` → check for a squash-merge, which leaves the branch's commits absent from base:
  ```
  github_prs_search  query="repo:<o>/<r> is:pr head:<b>"
  ```
  A hit with `merged_at` → **MERGED**. No hit → **LOCAL-ONLY** (genuine in-progress or abandoned local work — human judgment, never auto-remove).

> The two signals are complementary: `branch_status` catches ff/rebase/merge-commit merges; PR `merged_at` catches squash-merges. You need both to be sure a branch is dead.

### Step 5: Report the census

Print a derived table — never a saved snapshot (it rots the moment a PR merges). Order by disposition so dead weight is obvious:

```
disposition   path                              branch                     evidence
MERGED        .wt/old-feature                   feat/old-feature           PR #123 merged 2026-06-10
MERGED        .wt/rebase-x                       rebase/x                   branch_status: merged_into_base
DRIFTED       .wt/active                         feat/active                PR #140 open, local HEAD ahead
LIVE          .wt/current                        feat/current               PR #141 open, in sync
LOCAL-ONLY    .wt/experiment                     spike/idea                 no PR, not in base
EPHEMERAL     .claude/worktrees/agent-abc        agent-abc                  harness worktree
```

For MERGED / CLOSED / finished EPHEMERAL rows, propose the removal commands — **do not run them**:

```bash
git worktree remove .wt/old-feature   # + git branch -d feat/old-feature (if fully merged)
```

### Step 6: Cleanup (only what's confirmed)

Before removing any worktree, check it isn't carrying unsaved work:

```bash
git -C <path> status --porcelain        # uncommitted changes?
git -C <path> log --oneline @{upstream}.. 2>/dev/null   # unpushed commits?
```

- Clean + MERGED → `git worktree remove <path>` is safe after the user confirms the batch.
- Dirty or unpushed → **stop and surface it**. `git worktree remove` refuses without `--force`; do not pass `--force` without explicit per-worktree confirmation. A MERGED classification is about the branch, not about local edits made after the merge.

## Common Mistakes

**Trusting local git alone.** `git branch --merged` misses squash-merges and can't see PR state. The remote is the source of truth — that's why this skill joins against it.

**Looping a full compare per worktree.** A raw `compare` is thousands of tokens each and will blow context over 50 worktrees. Use `github_prs_list` (bulk) first and `github_branch_status` (bounded) only for the residue.

**Auto-removing.** Never remove a worktree without showing the census and getting confirmation. MERGED is a strong signal, not a license.

**Classifying `no PR` as `in progress`.** Squash-merged and ref-deleted branches also have no PR row — fall through to `branch_status` + `prs_search` before calling something live.

## Watch For

- A branch never pushed has no remote counterpart at all — it can only be LOCAL-ONLY; the remote tools are blind to it. Say so rather than guessing.
- DRIFTED usually means real unpushed local work. Investigate before removing.
- Harness worktrees can share a HEAD sha (spawned from the same base) — dedupe by path, not sha.

## Integration

**Pairs with:**
- **using-git-worktrees** — creates the worktrees this skill later audits.
- **finishing-a-development-branch** — the per-branch finisher; worktree-census is the fleet-wide GC that finds the branches that were never finished.

## Adaptive Work Sizing

Assess scope before committing to a work plan. If the census is larger than you can complete in your current context:

1. **Do what you can** — audit and report one repo (or one batch of worktrees) with clear boundaries.
2. **Document what remains** — leave the exact next repo/branch set to check, not vague notes.
3. **Spawn a follow-on agent** — or tell the caller to. The continuation agent should be able to resume from your census table without re-discovering everything.

Never assume you can finish everything. A completed, confirmed batch is more valuable than an incomplete sweep.
