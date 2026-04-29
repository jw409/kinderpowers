# GSD v1.30.0 → v1.38.3 Merge Plan

**Verdict**: HEAVY. Do not merge in a single session. Execute in 5 staged phases.

**Fork point**: v1.30.0-kp.1 (commit 5305652, bc052f5; March 2026)
**Target**: upstream v1.38.3 (2 days ago, April 2026)
**Skipped**: v1.39.0-rc.1 (rc gate, stable-only policy)

---

## Materiality Summary

Source: `gh api repos/gsd-build/get-shit-done/compare/v1.30.0...v1.38.3`

| Metric | Value |
| --- | --- |
| Commits | 531 |
| Files touched | 300 |
| Additions | 15,607 |
| Deletions | 2,169 |

### Top-level directory breakdown

| Dir | Files | Notes |
| --- | ---: | --- |
| `get-shit-done/` | 124 | The upstream payload directory — this is what `gsd/` mirrors in our fork. Primary merge target. |
| `commands/` | 57 | Slash-command docs consumed by installer; mostly doc-style. 24 new, 33 modified. |
| `docs/` | 51 | External documentation — NOT consumed by fork. Safe to ignore. |
| `agents/` | 32 | Agent persona `.md` files. 12+ new agents; several conflict with kp parameterized collapse. |
| `.github/` | 23 | Workflows, issue templates — NOT consumed by fork. Safe to ignore. |
| `bin/install.js` | 1 | +2430/-468. Outer installer — fork does not use it; safe to ignore. |

### `get-shit-done/` (primary target) breakdown

| Bucket | Added | Modified | Removed | Total |
| --- | ---: | ---: | ---: | ---: |
| All | 61 | 62 | 1 | 124 |
| `bin/lib/` | 7 | 16 | 0 | 23 |
| `workflows/` | 12 | 20 | 0 | 32 |
| `references/` | ~24 | ~14 | 0 | ~38 |
| `templates/` | ~6 | ~10 | 0 | ~16 |
| `contexts/` | 3 | 0 | 0 | 3 (NEW subdir) |

**Removed**: `get-shit-done/commands/gsd/workstreams.md` (superseded by `/gsd:settings`).

### Structural notes

- Upstream's consumable payload lives at `get-shit-done/` (has since before v1.30.0). Our fork's `gsd/` was merged from the **payload**, not the repo root. This mapping must be preserved: `upstream:get-shit-done/X` → `fork:gsd/X`.
- Upstream added a new `contexts/` subdir (dev.md, research.md, review.md) — fork must create `gsd/contexts/` to match.
- Upstream removed `get-shit-done/commands/` during the window (now installed from root `commands/` instead). Fork currently does NOT have `gsd/commands/`, so this is already aligned.

---

## Conflict Hotspots (kinderpowers divergence from upstream)

### 1. Parameterized agent collapse (v6.1 work, high risk)

Fork's root `agents/` contains kp-specific parameterized agents from PR #15 / `02-agent-collapse` milestone:
- `gsd-planner.md` — absorbs `gsd-roadmapper`; scope=phase|milestone|project
- `gsd-researcher.md` — absorbs `gsd-phase-researcher`, `gsd-project-researcher`, `gsd-research-synthesizer`; mode=phase|project|synthesize
- `gsd-ui.md` — absorbs `gsd-ui-auditor`, `gsd-ui-checker`, `gsd-ui-researcher`; mode=spec|audit|validate
- `gsd-verifier.md` — parameterized (4 modes); see PR #15
- Thin-alias stubs for 10 legacy agent names (see commit 4bff9aa)

**Upstream added 12+ new agents at v1.31–v1.38** that kinderpowers does not yet have:
  `gsd-doc-writer` (+615), `gsd-code-fixer` (+517), `gsd-pattern-mapper` (+335), `gsd-intel-updater` (+334), `gsd-debug-session-manager` (+314), `gsd-doc-synthesizer` (+204), `gsd-doc-verifier` (+201), `gsd-eval-auditor`, `gsd-eval-planner`, `gsd-debugger` (significant changes), `gsd-integration-checker`, `gsd-framework-selector`, `gsd-domain-researcher`, `gsd-advisor-researcher`, `gsd-ai-researcher`, `gsd-user-profiler`, `gsd-security-auditor`.

**Conflict**: upstream's `gsd-planner.md` diff is +241/-347 — it was rewritten. Kp's parameterized version must not be blindly overwritten. Same for researcher, ui, verifier.

**Resolution strategy**: For the 4 parameterized agents, 3-way diff with v1.30.0-era upstream as base. Hand-merge upstream improvements into kp parameterized structure. DO NOT accept upstream version wholesale.

### 2. Bead subsystem in `bin/gsd-tools.cjs` + `bin/lib/core.cjs`

Fork integrated `beads` tracker via PR #14/#16 (commits 162e309, cfb4954, c3b8af1, 7ad108d). Adds:
- `beadsAvailable()` + helpers in `core.cjs`
- `gsd bead <subcmd>` router in `gsd-tools.cjs`
- Bead create/update/close calls in new-project, plan-phase, execute, verify, ship workflows

**Conflict**: upstream's `gsd-tools.cjs` has been modified (status=modified) and `core.cjs` too. Bead router and helpers are kp-exclusive — they must survive the merge. Line-level 3-way diff required.

### 3. New upstream lib modules (7 new)

- `audit.cjs` — new audit primitive
- `docs.cjs` — new docs command subsystem
- `graphify.cjs` — new graph/relation builder (`/gsd:graphify`)
- `gsd2-import.cjs` — v2 import helper
- `intel.cjs` — intel-updater subsystem
- `learnings.cjs` — learnings subsystem (likely relates to kp's own "learning pipeline" PR #13 — **check for conflicts with our L1-L4 progression work**)
- `schema-detect.cjs` — schema detection

**Learnings concern**: We shipped our own learning pipeline in PR #13. Upstream shipped `learnings.cjs` separately. These likely DO NOT align. Audit required.

### 4. New workflows (12 new)

ai-integration-phase, analyze-dependencies, audit-fix, code-review-fix, code-review, discuss-phase-power, docs-update, eval-review, explore, extract_learnings, import, inbox. Most appear low-risk (additive), but `code-review.md` and `extract_learnings.md` may overlap with kp's `code-review` skill and learning pipeline — audit both.

### 5. `--help`, workstream `--clear`, init error wrappers (kp fixes in bc052f5/fd89c1a)

Fork added `--help` handling, proper error messages for init undefined, `workstream set --clear` flag. These are inside `gsd-tools.cjs`, `init.cjs`, `workstream.cjs` — all 3 of which have upstream modifications.

**Verify**: each kp fix must be re-applied or preserved post-merge.

### 6. Kinderpowers-specific customizations in `gsd/` files

`grep -r "kinderpowers\|kp-" gsd/` returned **zero** matches. The fork's customizations are NOT tagged with a `kp-` marker. They must be identified via git blame against commits on top of the v1.30.0 baseline (see §2 above). This makes conflict resolution more costly than expected.

---

## Phased Execution Plan

### Phase 1 — Preparation & payload baseline (~2 hrs, low risk)

- Create worktree: `.worktrees/gsd-bump-v1.38`
- Add upstream remote, fetch `v1.30.0` + `v1.38.3` tags
- Extract reference copies of the upstream payload at both tags into temp dirs for 3-way diffing:
  - `.tmp/gsd-upstream-v1.30/` (base) ← upstream:get-shit-done/ at v1.30.0
  - `.tmp/gsd-upstream-v1.38/` (theirs) ← upstream:get-shit-done/ at v1.38.3
  - `gsd/` is "ours"
- Tooling: write a helper script `.planning/gsd-v1.38-merge/3way.sh` that runs `git merge-file` per file with the three versions. Dry-run mode first.

**Deliverable**: merge infrastructure + baseline diff report.

### Phase 2 — Low-risk additions (contexts, references, templates, safe workflows) (~3 hrs)

Apply all **purely additive** upstream changes where no kp file exists:
- `gsd/contexts/` (new dir): copy dev.md, research.md, review.md
- 24 added references: copy whole-file
- Added templates (AI-SPEC.md, DEBUG.md, SECURITY.md, VALIDATION.md, debug-subagent-prompt.md, dev-preferences.md, discovery.md, planner-subagent-prompt.md, research.md, spec.md, state.md): copy whole-file
- 12 new workflows (ai-integration-phase, analyze-dependencies, audit-fix, code-review-fix, code-review, discuss-phase-power, docs-update, eval-review, explore, extract_learnings, import, inbox): copy whole-file

**Verify**: `node gsd/bin/gsd-tools.cjs --help` still exits 0; no referenced-but-missing files.

**Checkpoint commit**: "feat(gsd): v1.38 phase 2 — add new contexts/references/templates/workflows"

### Phase 3 — Modified workflows, templates, references (~4 hrs, moderate risk)

- 20 modified workflows (add-phase, add-tests, add-todo, audit-milestone, audit-uat, autonomous, check-todos, cleanup, complete-milestone, diagnose-issues, discovery-phase, discuss-phase-assumptions, discuss-phase, do, execute-phase, execute-plan, fast, forensics, health, help): 3-way merge.
- ~10 modified templates: 3-way merge; flag claude-md.md, config.json for careful review.
- ~14 modified references: 3-way merge; most should be clean.

**Watch for**: workflows that reference agents kp has collapsed (e.g. `gsd-roadmapper` calls → `gsd-planner scope=project`). Apply rewrites.

**Verify**: grep the merged workflows for `gsd-roadmapper`, `gsd-ui-auditor`, `gsd-ui-checker`, `gsd-ui-researcher`, `gsd-phase-researcher`, `gsd-project-researcher`, `gsd-research-synthesizer` — all must be rewritten to the collapsed parameterized form.

**Checkpoint commit**: "feat(gsd): v1.38 phase 3 — merge modified workflows/templates/refs"

### Phase 4 — Library code (`bin/lib/`) (~5 hrs, high risk)

Per file:
1. Copy upstream v1.38.3 version into tmp, copy kp current into tmp.
2. Run `diff -u` both vs v1.30.0 baseline.
3. Hand-merge the two diffs.
4. Unit-test each module's exports via a small harness script.

**Files requiring careful handling**:
- `core.cjs` — must preserve `beadsAvailable()` + bead helpers (PR #14/#16)
- `commands.cjs` — must preserve bead router (commit 162e309)
- `init.cjs` — must preserve undefined-leak fix (fd89c1a)
- `workstream.cjs` — must preserve `--clear` flag + name-required footgun (fd89c1a)
- `gsd-tools.cjs` — must preserve --help support + bead subcommand routing

**New files (copy as-is, no conflict)**: `audit.cjs`, `docs.cjs`, `graphify.cjs`, `gsd2-import.cjs`, `intel.cjs`, `schema-detect.cjs`.

**Special audit**: `learnings.cjs` — check overlap with kp's PR #13 learning pipeline. May need to rename/namespace one of them to avoid collision. **Potential blocker**.

**Verify**:
- `node gsd/bin/gsd-tools.cjs --help` exits 0
- `node gsd/bin/gsd-tools.cjs bead list` works
- `node -e "require('./gsd/bin/lib/core.cjs')"` loads without error for each lib
- Existing tests (if any) pass

**Checkpoint commit**: "feat(gsd): v1.38 phase 4 — merge bin/lib/ modules"

### Phase 5 — Agent integration (~3 hrs, high risk — touches kp collapse)

- 12+ new upstream agents: decide per-agent whether kp wants them at all. Many are SDK-specific or doc-pipeline agents that may duplicate kp skills (e.g. `gsd-doc-writer` vs kp doc-writing skills, `gsd-code-fixer` vs kp code-review skill). **Default: do not import** unless a specific upstream workflow references them and we've preserved that workflow.
- Modified agents that match kp collapse targets (`gsd-planner`, `gsd-researcher`, `gsd-ui`, `gsd-verifier`, `gsd-plan-checker`, `gsd-executor`, `gsd-codebase-mapper`, etc.):
  - 3-way merge preserving kp parameterization
  - Accept upstream's prose/guidance improvements
  - Reject upstream's structural reverts (unsplitting our collapses)
- Thin-alias stubs (commit 4bff9aa) must remain valid pointers.

**Verify**: every agent referenced in any workflow (ours + newly-merged) resolves to an existing file. No `subagent_type` dangles.

**Checkpoint commit**: "feat(gsd): v1.38 phase 5 — reconcile agent catalog with kp collapse"

### Phase 6 — Metadata, docs, release (~1 hr)

- `gsd/VERSION`: `1.30.0-kp.1` → `1.38.3-kp.1`
- `gsd/CHANGELOG.md`: append v1.31..v1.38 entries (copy from upstream CHANGELOG) + kp-specific notes
- Root `CHANGELOG.md`: add `[6.2.6]` or `[6.3.0]` section
- `.claude-plugin/plugin.json`: bump any GSD version claim
- `KINDERPOWERS.xml`: update `kinderpowers-fork-at` attribute, re-audit the 42-command catalog (new commands were added)
- `README.md`: re-audit GSD command count if it cites a specific number

**Final verification**:
- Full installer smoke test
- `gsd:help` output sanity
- CI green

**Release commit**: "feat: merge GSD upstream v1.30.0 → v1.38.3 (v6.3.0)"

---

## Estimated effort

| Phase | Hours | Risk |
| --- | ---: | --- |
| 1 Prep | 2 | low |
| 2 Adds | 3 | low |
| 3 Modifies (docs-like) | 4 | moderate |
| 4 Lib code | 5 | high |
| 5 Agents | 3 | high |
| 6 Release | 1 | low |
| **Total** | **18** | — |

Realistic with review+testing: **2-3 focused days** or a single long autonomous session with compaction risk.

---

## Blockers to raise with the main session

1. **`learnings.cjs` collision**: Upstream shipped a learnings subsystem in ~v1.34. Kinderpowers shipped its own learning pipeline in PR #13. Decide: (a) adopt upstream's, retire ours; (b) keep ours, skip upstream's; (c) namespace both. Recommend (b) — ours is more mature.

2. **`bin/install.js` rewrite**: Upstream rewrote its outer installer (+2430/-468). This installer is NOT shipped to users of the kinderpowers plugin. But if kinderpowers uses `scripts/update-gsd.sh` or similar upstream-snapshot tooling, that script may need updates.

3. **SDK split**: Upstream added an `sdk/` dir at root and package.json/tsconfig.json. Kinderpowers does not consume the upstream SDK. Explicitly skip.

4. **Upstream deprecation of `get-shit-done/commands/`**: Upstream removed `get-shit-done/commands/gsd/workstreams.md`. Our fork already does not have `gsd/commands/`, so we're aligned.

5. **Agent bloat**: 12+ new upstream agents. Decision point: adopt all, adopt selectively, or adopt none. Recommend "selective, only if a preserved workflow references it." This must be decided before Phase 5.

6. **v1.39.0-rc.1**: Skipped per policy. Document the policy in CHANGELOG so the next merger knows why the jump went to v1.38.3 not latest-tag.

---

## Non-goals (explicit exclusions)

- Not merging upstream `docs/`, `.github/`, `assets/`, `README.*`, `SECURITY.md`, `CONTRIBUTING.md`, `LICENSE` — these are upstream-project metadata, not fork payload.
- Not merging upstream `sdk/`, `package.json`, `package-lock.json`, `vitest.config.ts`, `tests/` — not consumed by fork.
- Not merging upstream `bin/install.js` or `scripts/` at the repo root — fork has its own install flow.
