# Merge Plan: obra/superpowers v4.3.1 → v5.0.7 into kinderpowers

**Author:** gsd-planner (evidence-based)
**Date:** 2026-04-23
**Status:** DRAFT — awaiting human decision
**Base commit:** kinderpowers `main` @ 92958d8 (v6.2.5)
**Upstream target:** obra/superpowers @ `v5.0.7` (1f20bef)

---

## TL;DR

The v4→v5 bump looks major but **86% of the 94 changed files are either purely additive (new brainstorm-server, new platform integrations, new docs/specs) or tests/one-line touch-ups irrelevant to our fork**. The real conflict surface is **7 files**, of which only **2 are sharp** (brainstorming/SKILL.md, writing-plans/SKILL.md — kinderpowers has significantly diverged philosophy). Recommend **selective cherry-pick, not full merge**. Two phases, a few hours of work.

---

## 1. Materiality Verdict: **MODERATE** (leaning TRIVIAL-for-our-purposes)

### Evidence (actual diff, not version-number delta)

`gh api compare v4.3.1...v5.0.7`:

- **94 files changed**, 130 commits, ahead_by 130
- **~8,900 insertions, ~1,100 deletions** — but the *shape* matters, not the total
- Category breakdown (hand-classified from the file list):

| Category | File count | Lines | Relevance to fork |
|----------|-----------:|------:|-------------------|
| **Purely additive** (new brainstorm-server, new platform configs for Cursor/Copilot/Gemini, new specs in `docs/superpowers/`, new issue templates) | **51** | ~7,400 add | Low — we don't ship those platforms; we can ignore or cherry-pick |
| **Cosmetic / test one-liners** (`#!/bin/bash` → `#!/usr/bin/env bash`, one-char string edits in test prompts, test scaffold touch-ups) | **29** | ~200 add, ~30 del | None — ignore |
| **Upstream deletions we don't have** (`lib/skills-core.js`, `tests/opencode/test-skills-core.sh`) | **2** | −648 | None — already absent from fork |
| **CONFLICTING with fork** (files we also modified) | **7** | see §3 | Real — must decide per-file |
| **Mixed (upstream-only infra we might want: hook quoting, resume fix)** | **5** | see §3 | Moderate — cherry-pick worth considering |

So of 94 files, **only 7 genuinely touch ground kinderpowers has modified**, and of those 7, **two are deeply divergent** (brainstorming, writing-plans) while **five are small fixes we can adopt cleanly**.

**Verdict:** MODERATE — but only because of those two deeply-divergent skill files. If you treat those as "stay kinderpowers", the rest is TRIVIAL.

---

## 2. What's Actually In v5.0 (the real feature list)

Sourced from `RELEASE-NOTES.md` + `releases` API + the file diff:

### v5.0.0 — Breaking Changes (real ones)

- **Spec/plan output paths moved:** `docs/plans/` → `docs/superpowers/specs/` (specs) and `docs/superpowers/plans/` (plans). *Every internal skill path reference updated in lockstep.*
- **`executing-plans` no longer batches** — removed the "3-task then review" checkpoint pattern. Continuous execution now.
- **Slash commands deprecated:** `/brainstorm`, `/write-plan`, `/execute-plan` now show deprecation notices pointing to skills. (We don't ship these as core commands anyway — our commands/ dir is kinderpowers-specific.)
- **`subagent-driven-development` made mandatory** on capable harnesses (Claude Code, Codex). Partially reverted in v5.0.5 — "recommended but no longer mandatory."

### v5.0.0 — New Features

- **Visual brainstorming companion** — browser-based mockup/diagram companion for brainstorming sessions. (`skills/brainstorming/visual-companion.md`, 287 new lines.)
- **Document review system** — subagent-dispatched spec/plan reviewers. *Killed in v5.0.6 — replaced with inline self-review.* Net effect: simpler skill prose, no subagent round-trips.

### v5.0.1 — Agentskills compliance + Gemini

- `lib/brainstorm-server/` → `skills/brainstorming/scripts/` (agentskills.io spec compliance; `lib/` dir removed entirely)
- Gemini CLI extension (`gemini-extension.json`, `GEMINI.md`, `references/gemini-tools.md`)
- **SessionStart hook single-emit fix** — hook now emits EITHER `additional_context` OR `hookSpecificOutput`, not both, depending on platform env vars. (We fixed similar in `6c7fc0e`, but kinderpowers' version still emits **both** — see §3.)
- **Double-quote the `${CLAUDE_PLUGIN_ROOT}` in hooks.json** — fix for Windows cmd.exe + Linux var expansion. *kinderpowers still uses single quotes.* (§3)

### v5.0.2 — Zero-dep brainstorm server

- Removed ~1,200 lines of vendored `node_modules`, replaced with native Node `http`/`fs`/`crypto`. Auto-exit after 30min idle. Owner-PID tracking.
- **Subagent context isolation principle** added to all delegation skills (brainstorming, dispatching-parallel-agents, requesting-code-review, subagent-driven-development, writing-plans).

### v5.0.3 — Cursor + hook robustness

- Cursor hook config (`hooks/hooks-cursor.json`, `.cursor-plugin/plugin.json`)
- **SessionStart no longer fires on `--resume`** — matcher changed from `startup|resume|clear|compact` to `startup|clear|compact`. *kinderpowers still has `resume`.* (§3)
- **bash 5.3 heredoc hang fix** — `printf` replaces `cat <<EOF` in `hooks/session-start`. *kinderpowers still uses heredoc.* (§3)
- **POSIX-safe** `$0` replaces `${BASH_SOURCE[0]:-$0}` (kinderpowers still uses the bash-ism)
- **Portable shebangs** `#!/usr/bin/env bash` everywhere

### v5.0.4 — Review loops refined

- Single whole-plan review instead of chunk-by-chunk. Max iterations 5→3. "Calibration" sections raise the blocking-issue bar. *kinderpowers doesn't ship a plan-review loop at all, so this is moot except as prose-style guidance.*
- OpenCode one-line plugin install (we don't target OpenCode — ignore).

### v5.0.5 — Bug fixes

- Brainstorm server `server.js` → `server.cjs` (Node 22+ ESM fix)
- `stop-server.sh` SIGTERM + 2s + SIGKILL fallback
- **Execution handoff made user choice again** (subagent-driven recommended, not mandatory)

### v5.0.6 — Inline self-review replaces subagent review loops

- Brainstorming Spec Review Loop → inline Spec Self-Review checklist
- Writing-plans Plan Review Loop → inline Self-Review checklist + explicit "No Placeholders" section
- Brainstorm server session dir split: `content/` (browser-facing) + `state/` (server internals) — security fix
- Owner-PID on WSL and Tailscale SSH — EPERM treated as "alive", startup validation

### v5.0.7 — Copilot CLI

- Copilot CLI SessionStart context injection via `additionalContext` SDK-standard format
- `references/copilot-tools.md` tool-mapping table
- OpenCode bootstrap moved from `system.transform` to `messages.transform` (Qwen compatibility)

### Summary of genuinely-new-things-a-kinderpowers-user-might-want

1. **Zero-dep brainstorm server** (~1700 lines under `skills/brainstorming/scripts/`) — standalone, agentskills-compliant. Would work as an opt-in skill addition.
2. **SessionStart robustness fixes** (hook quoting, bash 5.3, POSIX, resume, portable shebangs) — we want ALL of these.
3. **Inline self-review** for brainstorming + writing-plans — philosophically *compatible* with kinderpowers' "no subagent compulsion" principle; worth adopting as prose.
4. **"No Placeholders" section** in writing-plans — low-risk prose addition.
5. **Copilot/Cursor/Gemini platform support** — we don't target these. Skip unless we want to broaden reach.
6. **Visual brainstorming companion** — cool but orthogonal to our agency-preservation thesis. Can live alongside. Optional.

---

## 3. Conflict Map

The 7 files where upstream changes overlap with kinderpowers modifications:

| Our file | Upstream change (v5.x) | Severity | Resolution |
|----------|-----------------------|----------|------------|
| `hooks/session-start` | Platform-dispatched emit (one of 3 formats), `printf` replaces heredoc, POSIX `$0`, Copilot CLI env detection, generic "superpowers" prose | **LOW-MEDIUM** — robustness fixes we want; but prose is kinderpowers-branded and hard-coded to `skills/using-kinderpowers/` | **Cherry-pick**: adopt the dispatch structure, printf, POSIX-safe patterns; keep kinderpowers branding and skill path. We already have dual-emit from `6c7fc0e` — replace with platform dispatch. |
| `hooks/hooks.json` | Matcher changed `startup\|resume\|clear\|compact` → `startup\|clear\|compact`; command arg double-quoted (was single) | **LOW** — straight adoption | **Adopt wholesale** — kinderpowers' matcher re-fires on resume, which is wasted context. Double-quoting is a real bug fix. |
| `skills/brainstorming/SKILL.md` | HARD-GATE block; added scope-decomposition logic; 9-step checklist (vs our 6); visual companion offer; inline spec self-review; path change `docs/plans/` → `docs/superpowers/specs/` | **HIGH** — we added parameterization (breadth/mode/time_box/depth/constraint_level) upstream does not have; we also have a duplicate `## Parameters` block (real bug in our file at lines 14-22 vs 28-35) | **Kinderpowers-wins on structure**; selectively adopt: (a) scope-decomposition prose, (b) inline self-review as a kinderpowers-style checklist item, (c) path update to `docs/superpowers/specs/` if we want upstream-path compatibility. Fix the duplicate Parameters table while we're there. Skip HARD-GATE (clashes with agency-preservation philosophy). Skip visual companion unless we also ship the server. |
| `skills/writing-plans/SKILL.md` | Added "Scope Check" section; "File Structure" section; checkbox task syntax `- [ ]`; "No Placeholders" section; path → `docs/superpowers/plans/`; execution handoff prose tightened | **HIGH** — kinderpowers has its own execution-handoff block with our two-option choice (subagent-driven + parallel session), adaptive work sizing addendum, and references to `kinderpowers:subagent-driven-development` | **Kinderpowers-wins on handoff**; cherry-pick: (a) Scope Check prose, (b) File Structure prose, (c) "No Placeholders" section verbatim — all additive and philosophy-compatible. Optionally adopt checkbox `- [ ]` syntax. Keep our two-option handoff. |
| `skills/executing-plans/SKILL.md` | Removed batching; now "load, review, execute all, report when complete" (much shorter) | **MEDIUM** — kinderpowers' version is already philosophically closer to the upstream "continuous" model AND keeps the parameter table, work-item claim protocol, verify-before-assuming, parallel execution patterns — all value-adds. | **Keep kinderpowers**. Upstream's new version is *simpler than ours, not better.* Maybe adopt their tightened "Announce at start" wording. No material change needed. |
| `skills/subagent-driven-development/SKILL.md` | Added subagent-context-isolation principle; minor prose additions | **LOW** — additive | **Cherry-pick** the context-isolation principle prose. |
| `skills/requesting-code-review/SKILL.md` | Context-isolation principle (2-line addition) | **LOW** | **Cherry-pick** (trivial). |
| `skills/dispatching-parallel-agents/SKILL.md` | Context-isolation principle (2-line addition) | **LOW** | **Cherry-pick** (trivial). |
| `skills/writing-skills/SKILL.md` | Frontmatter correction: "two required fields" (vs earlier "only two fields"), agentskills.io link | **TRIVIAL** | **Adopt** the corrected sentence. |

**What is NOT in conflict** (and therefore not our problem):
- `scanner.py`, MCP servers, `gsd/`, `agents/`, `hookify-rules/`, `docs/` kinderpowers-native files — upstream touched none of them.
- `skills/using-kinderpowers/` — upstream has `using-superpowers/SKILL.md` changes (+22 lines adding Copilot CLI info); no relevance, we don't need to merge.
- Our 16+ kinderpowers-only skills (beads, metathinking, strategic-planning, team-orchestration, adversarial-review, etc.) — untouched by upstream.
- Everything under `commands/` — upstream deprecated `/brainstorm` `/write-plan` `/execute-plan`; our commands directory is kinderpowers-native.

### SessionStart Hook: close-read

| | upstream v5.0.7 | kinderpowers main |
|--|--|--|
| Shebang | `#!/usr/bin/env bash` | `#!/usr/bin/env bash` ✓ |
| `$0` vs `${BASH_SOURCE[0]:-$0}` | `$0` (POSIX-safe) | `${BASH_SOURCE[0]:-$0}` (not sh-compatible) |
| Warning legacy-skills path | `~/.config/superpowers/skills` | `~/.config/superpowers/skills` (same — fine) |
| JSON emit method | `printf` (bash 5.3 safe) | `cat <<EOF` heredoc (hangs on bash 5.3+) |
| JSON shape | Platform dispatch: Cursor→`additional_context`, Claude Code→`hookSpecificOutput`, else→`additionalContext` | Emits **BOTH** `additional_context` AND `hookSpecificOutput` in same JSON |
| hooks.json matcher | `startup\|clear\|compact` | `startup\|resume\|clear\|compact` (fires on resume, re-injecting context) |
| hooks.json quoting | `\"${CLAUDE_PLUGIN_ROOT}/...\"` (escaped double) | `'${CLAUDE_PLUGIN_ROOT}/...'` (single — breaks on Windows cmd + Linux) |

**These are genuine bugs in our fork's hook stack.** Adopt all six changes.

---

## 4. Phased Merge Plan

Two phases. Nothing more is warranted.

### Phase 1 — "Upstream robustness wins" (1-2 hours)

Adopt all the low-severity upstream fixes that have no philosophical conflict.

**Files changed:**
- `hooks/session-start` — rewrite to match upstream v5.0.7 structure (platform dispatch, printf, POSIX `$0`), but with kinderpowers branding and `using-kinderpowers` skill path
- `hooks/hooks.json` — matcher + quote fix
- `skills/writing-plans/SKILL.md` — add "Scope Check", "File Structure", "No Placeholders" sections (additive); leave execution handoff + adaptive work sizing untouched
- `skills/writing-skills/SKILL.md` — fix "two required fields" sentence, link agentskills.io
- `skills/subagent-driven-development/SKILL.md` — add context-isolation principle prose
- `skills/requesting-code-review/SKILL.md` — add context-isolation principle
- `skills/dispatching-parallel-agents/SKILL.md` — add context-isolation principle
- `skills/brainstorming/SKILL.md` — fix duplicate Parameters table bug (kinderpowers-local bug, surfaced during review); add scope-decomposition prose; leave HARD-GATE alone

**Success criteria:**
- [ ] SessionStart hook runs cleanly on bash 5.3, dash, and Windows cmd/bash
- [ ] `--resume` sessions no longer re-inject `using-kinderpowers` content
- [ ] Single-quote bug in hooks.json fixed (verifiable with Windows Git Bash test)
- [ ] No regression in existing test suite (`test_scanner.py` etc.)
- [ ] `skills/brainstorming/SKILL.md` has exactly one Parameters table

**Verifiable:** run a test session, check hook output JSON, grep for duplicate Parameters table.

### Phase 2 — "Brainstorm server + path alignment" (optional, 3-5 hours)

If the user wants to adopt upstream's brainstorm server and `docs/superpowers/` path convention:

- Copy `skills/brainstorming/scripts/` verbatim from v5.0.7 (~1,700 lines, zero-dep)
- Copy `skills/brainstorming/visual-companion.md` (287 lines) with kinderpowers branding edits
- Update path in `skills/brainstorming/SKILL.md` and `skills/writing-plans/SKILL.md`: `docs/plans/` → `docs/superpowers/specs|plans/`
- Update any internal references in kinderpowers docs

**Success criteria:**
- [ ] `skills/brainstorming/scripts/start-server.sh` starts a server and serves HTML without Express/Chokidar deps
- [ ] Plans written to new path location, old-path references updated
- [ ] Visual companion can be invoked and reaches the browser

**Phase 2 is optional.** Recommendation: skip unless the user specifically wants the visual companion — it adds ~2000 lines of code that's orthogonal to kinderpowers' core thesis (agency-preserving skip costs, scanner, MCP servers, progression model).

### What we are NOT doing

- Not adopting Cursor/Gemini/Copilot/OpenCode platform configs — out of scope for kinderpowers
- Not adopting the subagent-mandatory handoff shape in `writing-plans` — our two-option handoff is better aligned with agency preservation
- Not restructuring our `executing-plans` — ours is materially richer (parameters, work-item claim, parallel patterns)
- Not adopting HARD-GATE language in brainstorming — conflicts with agency-preservation principle (we don't use compulsion language; see fix in `efc0328`)
- Not bumping the incorporates.superpowers version until Phase 1 lands — incorporated version stays at 4.3.1 until we actually merge

---

## 5. Risks

### What could break

1. **SessionStart hook rewrite is the only real risk.** A bad JSON emit breaks every session. Test manually before committing:
   - Run `hooks/session-start` directly; verify JSON is valid (`jq .`)
   - Test under each env: `CLAUDE_PLUGIN_ROOT` set alone (Claude Code), `CURSOR_PLUGIN_ROOT` set (Cursor), `COPILOT_CLI=1` (Copilot)
   - Confirm `--resume` path no longer fires (matcher change)

2. **SKILL.md frontmatter parsing — NOT changed.** Upstream v5.0.6 writing-skills fix was just prose correction ("only two" → "two required"). No parsing change. The frontmatter spec still takes name+description as required; other fields optional. No risk to our skills.

3. **Hook schema — NOT changed since our `6c7fc0e` fix.** The `tool_result → tool_response` rename we already handled. v5.0 did not further rename hook fields. What DID change: session-start output format for Copilot CLI (`additionalContext` top-level) and the `--resume` matcher. Both additive, neither breaks us.

4. **`session-start.sh` restructure — only `hooks/session-start` (no `.sh`).** Upstream did restructure the emit logic (platform dispatch), but file location + invocation contract stayed identical. No risk to our `run-hook.cmd` wrapper or `hooks.json` command string shape.

5. **Prose additions to writing-plans are low-risk**, but check that "No Placeholders" doesn't conflict with kinderpowers' "adaptive work sizing" principle (it doesn't — both are about honesty).

### What does NOT put kinderpowers' value prop at risk

- `scanner.py` compulsion detection — untouched; upstream has no analogous feature
- MCP servers (`kp-github`, `kp-sequential-thinking`) — untouched
- GSD lifecycle engine — untouched
- Hookify integration — untouched
- Progression model (L1-L4) — untouched
- "Skip-cost" documentation philosophy — untouched
- Agency-preservation principles — untouched (upstream HARD-GATE and "MUST" compulsion language is what we explicitly moved away from in `efc0328`)

**Conclusion: kinderpowers' core value prop is NOT at risk from any upstream v5.x change.** The only philosophical collision is upstream's HARD-GATE and "You MUST" language in brainstorming — and we already decided (commit `efc0328`) not to use compulsion language. Don't adopt it.

---

## 6. Recommendation

**Selective cherry-pick — Phase 1 only — then stop.**

Evidence:
- 94 files changed sounds big, but only **7 overlap with our modifications** and only **2 deeply conflict** (brainstorming, writing-plans).
- The 51 "purely additive" files are for platforms we don't target (Cursor, Gemini, Copilot, OpenCode) or for the brainstorm server (optional, orthogonal to our thesis).
- The real value of merging v5.x is ~6 robustness fixes to `hooks/session-start` and `hooks/hooks.json`, plus ~3 prose additions to skills. **These are hours of work, not days.**
- Full merge would drag in compulsion-language skill rewrites that contradict kinderpowers' agency-preservation thesis.

**Action:**
1. Do Phase 1 now (the 8 files above, ~1-2 hours). Commit as `chore: selective merge of upstream v5.0.7 hook + skill robustness fixes`.
2. Bump `incorporates.superpowers.version` in `plugin.json` to `5.0.7` with a comment noting *what* we selectively took (not the whole thing).
3. Add a note in `CHANGELOG.md` explaining the selective-merge policy for future reference.
4. Skip Phase 2 unless there's demand for the visual brainstorming companion.
5. Set up a GitHub watch on `obra/superpowers/releases` so future upstream releases get this same evidence-based review, not a reflexive "v6.x → v7.x is major" reaction.

**TL;DR (two sentences):**

> The v4→v5 upstream bump is mostly additive platform support (Cursor/Gemini/Copilot) and a new brainstorm server — none of which threaten kinderpowers' core thesis. Cherry-pick ~8 files worth of real robustness fixes (hook quoting, `--resume` matcher, bash 5.3 heredoc, POSIX `$0`, "No Placeholders" prose) in ~2 hours; skip the rest; never do a blanket merge that would drag in compulsion-language skill rewrites we explicitly moved away from.
