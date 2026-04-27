# Changelog

## [6.3.0] — 2026-04-27

### Fixed

- **kp-github MCP: URL-encode path segments** (closes #19) — label tools (`get`, `update`, `delete`) returned 404 on label names containing characters that require URL encoding (e.g. `priority: P0`). The `urlencode` helper used form-encoding (space → `+`), which is wrong for path segments — GitHub treats `+` as a literal `+` in paths. Two new RFC 3986 path encoders (`urlencode_path` for single segments, `urlencode_path_multi` preserving `/`) are applied to every path-interpolating call site:
  - `tools/labels.rs`: get/update/delete (the documented bug)
  - `tools/files.rs`: get_contents, create_or_update, delete, push_files (path + branch in 2 ref endpoints)
  - `tools/tags.rs`: get
  - `tools/releases.rs`: get_by_tag
  - `tools/repos.rs`: compare (base + head, preserving slashes for `feature/foo` refs)
  - `tools/teams.rs`: members (team_slug)
  - 34 new tests including wiremock end-to-end tests asserting the actual wire path (e.g. `/labels/priority%3A%20P0`) and proptests proving path encoders never emit form-style `+` for space.
- **SessionStart hook: bash 5.3+ heredoc hang + Claude Code double-injection** — picked from upstream superpowers `537ec64`. The hook used `cat <<EOF` to emit context, which hangs on bash 5.3+ when the heredoc body exceeds ~512 bytes (the kinderpowers context payload is ~4 KB so every macOS Homebrew-bash session blocked indefinitely). Replaced with `printf`. While there, fixed a separate Claude Code bug: it reads BOTH `additional_context` and `hookSpecificOutput.additionalContext` without deduplication, so the hook now branches on `CLAUDE_PLUGIN_ROOT` and emits only the field the current platform consumes.

### Added

- **kp-github MCP: author/committer overrides on file tools** — `github_files_create_or_update`, `github_files_delete`, and `github_files_push` now accept optional `author_name` / `author_email` / `committer_name` / `committer_email`. Both name+email of a side must be set together; an asymmetric pair errors before the request is sent. Omitting all four keeps the prior behavior (commits attributed to the OAuth user). Implementation switches the contents API calls to JSON bodies via `api_json` so nested `{author: {name, email}}` objects can ride the request, and `client::api_json` now handles `DELETE` (was falling through to GET). +9 tests including wiremock body-matchers asserting the author/committer JSON reaches the wire and a partial-pair test proving validation short-circuits before any HTTP request.

### Build

- **Rebuilt linux-x86_64 `kp-github-mcp`** carrying both kp-github changes above. macOS-arm64 binary still needs rebuilding on a Mac to pick up these changes.

### Not in this release

- **superpowers upstream merge (v4.3.1 → v5.0.7)** — still queued per v6.2.5's Known gaps; not addressed here.
- **get-shit-done upstream bump (v1.30.0 → v1.38.5)** — same; queued.
- 8 superpowers cherry-pick candidates evaluated during this release were either already subsumed by v6.2.5's hook rewrite or conflicted with diverged kinderpowers files (`using-superpowers`/`using-kinderpowers` rename, `.opencode/plugins/superpowers.js` divergent OpenCode tool mapping, brainstorm-server scripts we don't carry, RELEASE-NOTES divergence). Deferred to the broader upstream merge.

## [6.2.6] — 2026-04-23

### Fixed

- **Plugin install error on recent Claude Code builds: `unrecognized key: incorporates`.** Same class of bug as v6.2.4's `upstream` fix — recent Claude Code releases strict-validate `plugin.json` and reject unknown top-level keys. The `incorporates` array (our attribution block listing superpowers, get-shit-done, hookify, toon-format) was not in the schema. Removed from `plugin.json`. Attribution is retained in `README.md`'s Credits section and in `KINDERPOWERS.xml` (which is canonical metadata, not validated by Claude Code). The known-safe top-level keys are: `name`, `description`, `version`, `author`, `homepage`, `repository`, `license`, `keywords`, `commands`, `agents`, `hooks`, `mcpServers`.

## [6.2.5] — 2026-04-23

### Fixed

- **SessionStart hook was broken since the kinderpowers rebrand.** `hooks/session-start` was forked verbatim from upstream superpowers and still `cat`-ted `skills/using-superpowers/SKILL.md` (nonexistent — the skill is `using-kinderpowers`). Every session silently emitted `cat: ... using-superpowers/SKILL.md: No such file or directory` and the model was told "You have superpowers" instead of the kinderpowers orientation. Fixed the path, the branding, and the legacy-skills-directory warning copy.
- **GSD upstream URL was a 404.** `plugin.json` and `KINDERPOWERS.xml` referenced `https://github.com/davidjbauer/get-shit-done`, which doesn't exist. The canonical upstream — maintained by TÂCHES — is `https://github.com/gsd-build/get-shit-done`. Corrected URL and author attribution. Bumped our claimed fork point from `1.26.0` to `1.30.0` to match the actual merge in PR #17.

### Changed

- **README rewritten entry-point-forward.** The previous README led with catalog counts (27 skills, 14 agents, 42 commands, 63 MCP tools). The new README leads with the two commands a user actually types and an explanation of how skills surface via Claude Code's `Skill` tool based on context, not name. Full catalog is deferred to `KINDERPOWERS.xml` as the canonical machine-readable manifest.
- **`SEQUENTIAL_THINKING_MODEL` default bumped `claude-opus-4-6` → `claude-opus-4-7`.** Sets the per-model profile that kp-sequential-thinking uses for hint thresholds and guidance shaping.
- **`KINDERPOWERS.xml` bumped to 6.2.5.** Stale v6.2.0 counts (14 agents, 27 skills — both wrong) replaced with "see agents/ and skills/ directories" since the filesystem is ground truth. Added explicit `upstream-latest`/`kinderpowers-fork-at`/`gap-note` attributes on the superpowers and get-shit-done project entries so downstream consumers know the merge position.

### Removed

- **crucible skill and 7 crucible-* agents.** `skills/crucible/`, `agents/crucible-auditor.md`, `agents/crucible-challenger.md`, `agents/crucible-composer.md`, `agents/crucible-exploder.md`, `agents/crucible-forecaster.md`, `agents/crucible-hunter.md`, `agents/crucible-verifier.md` removed from kinderpowers. Research-intelligence tooling will live in the `jw-scry` plugin going forward — kinderpowers keeps its focus on agency-preserving discipline for coding agents.

### Known gaps (not addressed here)

- **superpowers upstream is a major version ahead.** We sit at v4.3.1; upstream latest is v5.0.7 (22 days old, with 7 patch releases in the v5 line). Merge queued, not scheduled.
- **get-shit-done upstream is 8 minor versions ahead.** We sit at v1.30.0; upstream latest is v1.38.3 with v1.39.0-rc.1 in flight. Merge queued, not scheduled.

## [6.2.4] — 2026-04-23

### Fixed

- **Marketplace install error on recent Claude Code builds.** Removed the non-schema `upstream` attribution block from `.claude-plugin/marketplace.json`; recent Claude Code releases validate marketplace entries strictly and rejected the key with `invalid schema "upstream"`, blocking fresh `/plugin marketplace add jw409/kinderpowers` installs. Upstream attribution is retained in `plugin.json`'s `incorporates` array. Also synced the marketplace plugin entry's `version` field (was pinned at `6.2.2`) to match `plugin.json`.

### Changed

- **Retraction of v6.2.0 "Agent Collapse" claim.** The v6.2.0 CHANGELOG entry claimed that `gsd-researcher`, `gsd-verifier`, `gsd-ui`, and `gsd-planner` (with mode/scope parameters) "replaced" 10 legacy GSD agents, reducing the agent count from 16 to 8. In practice this migration was never completed: all 10 legacy agents retain full ~400–1300 line specifications, and every workflow (`gsd/workflows/*.md`), model router (`gsd/bin/lib/model-profiles.cjs`), and initializer (`gsd/bin/lib/init.cjs`) continues to spawn them by their original names with 13–22 live references each. Removed the "Replaces gsd-X" language from the description frontmatter of `gsd-planner.md`, `gsd-researcher.md`, and `gsd-ui.md`. `gsd-verifier.md` never carried the claim in its spec. Legacy agents (`gsd-roadmapper`, `gsd-phase-researcher`, `gsd-project-researcher`, `gsd-research-synthesizer`, `gsd-plan-checker`, `gsd-integration-checker`, `gsd-nyquist-auditor`, `gsd-ui-researcher`, `gsd-ui-checker`, `gsd-ui-auditor`) remain canonical. Actual GSD agent count: 18, not 8. Parameterized agents still exist and can be invoked on their own merits; they just don't displace anything. Finishing the collapse (migrating call sites, reworking MODEL_PROFILES to preserve per-legacy-agent cost tiers) is tracked but unscheduled.

## [6.2.3] — 2026-03-21

### Changed

- kp-sequential-thinking v0.2.0: slim output — 70-line guidance wall → 2-line `-- thinking --` separator
  - `firstCallGuidance`: params inline, no banners
  - `format_thought` stderr: ASCII box art → compact one-liner with 120-char preview
  - `tool_description`: 80-line duplicate → 2-line summary
  - JSONL logging: unchanged
  - Net: -211 lines, 120/120 tests pass
- Plugin env: `SEQUENTIAL_THINKING_MODEL` + `KP_SEQTHINK_LOG_LEVEL` now ship in plugin.json

## [6.2.2] — 2026-03-20

### Added

- `comprehensive-pr-review` skill — 7-persona team review with tiered depth, pedanticness slider, debate, self-improvement loop. Adapted from meshly v3 protocol.
- UCP (Universal Commerce Protocol) added to recommended tools — commerce interop for agents building stores
- Mapper intelligence ladder — AST scripts (ast.parse, cargo metadata, FTS5) as built-in product, not external tool dependency
- Mapper self-assessment — coverage report shows what was captured vs what exists

### Changed

- Parameterized 4 more skills: writing-plans, executing-plans, architecture, requirements
- XML skill catalog updated to 27 (added comprehensive-pr-review, using-kinderpowers)
- README rewritten — lead with value, not architecture
- Removed ck as dependency — purpose-built haiku agent with scripts is better
- Troll-test findings fixed (README counts, setup.sh description, 2>/dev/null removal)

### Fixed

- kp-sequential-thinking: serde(default) on all Option fields — fixes MCP validation errors
- kp-github-mcp: same serde(default) fix for 85 Option fields
- Duplicate GSD directory tree cleaned up (failed upstream update artifact)

## [6.2.0] — 2026-03-20

### Philosophy: Caller Controls Everything

6 phases executed in parallel via GSD, dogfooding kinderpowers on itself. 31 requirements, 16 plans, ~25 agents spawned.

### Added

**Phase 1: Sequential Thinking Spawn Hints**
- `spawn_candidate` hint with SpawnMeta (branch_points, recommended_depth, recommended_model)
- Enhanced MergeSummary with BranchOutcome (per-branch finalConfidence, doneReason) and convergenceSignal (converged/diverged/mixed/insufficient)
- `spawn_strategy` parameter in metathinking skill (none/convergent/divergent/hierarchical)
- 12 new Rust tests (123 total, 104 unit + 19 integration)

**Phase 2: Agent Collapse (16 → 8)**
- `gsd-researcher` replaces phase-researcher + project-researcher + research-synthesizer (mode: phase/project/synthesize)
- `gsd-verifier` replaces verifier + integration-checker + plan-checker + nyquist-auditor (mode: goal-backward/integration/plan-quality/coverage)
- `gsd-ui` replaces ui-researcher + ui-auditor + ui-checker (mode: spec/audit/validate)
- `gsd-planner` absorbs roadmapper via scope parameter (phase/milestone/project)
- 10 old agents replaced with thin alias stubs (backward compatible)
- 12 workflow files + 3 command files + model-profiles.cjs migrated
- Net reduction: 4,429 lines

**Phase 3: Beads Integration**
- `beadsAvailable()` cached check + 6 bead helper functions in core.cjs
- `bead` subcommands in gsd-tools.cjs (available, create, update, close, show)
- Bead lifecycle wired into 5 GSD workflows: new-project (epic), plan-phase (task), execute-phase (in_progress), verify (evidence), ship (close with PR)
- Graceful degradation: zero errors when beads not installed

**Phase 4: Agent Parameterization**
- code-reviewer: focus, pedanticness, scope
- research-extractor: mode, depth, output
- team-coordinator: worker_count, worker_model, isolation, coordination
- gsd-debugger: method, max_hypotheses, checkpoint_frequency, escalation

**Phase 5: Skill Parameterization**
- systematic-debugging: depth, hypothesis_count, reproduce_first
- brainstorming: breadth, mode, time_box
- test-driven-development: strictness, coverage_target, test_style
- verification-before-completion: evidence_bar, auto_run, check_types
- adversarial-review: intensity, min_findings, focus
- subagent-driven-development: worker_model, review_between, parallelism

**Phase 6: Team Mode**
- GSD workflows use TeamCreate + named Agent spawns instead of Task(background)
- map-codebase: TeamCreate("gsd-mapping") with 4 named mapper agents
- execute-phase: per-wave TeamCreate with named executor agents
- new-project: TeamCreate("gsd-research") with 4 researchers + synthesizer
- 4 agents gain Team Communication sections (mapper, executor, researcher, verifier)
- Fallback to Task(background) when TeamCreate unavailable
- Graceful degradation: SendMessage calls skipped silently when not in team

### Changed

- All version references bumped 6.1.0 → 6.2.0

## [6.1.0] — 2026-03-19

### Philosophy: Caller Controls

Skills and agents now expose parameters with sensible defaults. The caller tunes via their prompt.
The system parses what it recognizes and applies defaults for the rest. One-size-fits-all is the anti-pattern.

### Added

**kp-sequential-thinking MCP server**
- Hints system: non-prescriptive observations (linear_chain, premature_confidence, merge_available, etc.)
- Branch merge: `continuation_mode: "merge"` with `merge_branches` to synthesize insights (GoT-inspired)
- Dunning-Kruger detection: flags confidence > 80% at layer 1 as "observation"
- 106 tests (87 unit + 19 integration)

**Parameterized agents** — all accept caller-controlled parameters:
- `multi-perspective-review`: 5 modes (council, troll-test, adversarial, gentle, focused), lens_count 2-7, pedanticness slider
- `gsd-codebase-mapper`: depth slider, JSONL index output, expiry, LSP/AST probe step, multi-repo support
- `strategic-planner`: depth, approaches count, phase granularity, output format
- `quality-gate`: strictness slider, evidence types, min checks, security scan toggle
- `gsd-executor`: caution level, commit style, parallelism, deviation handling, test frequency

**Codebase mapper intelligence sources**
- LSP tool added (documentSymbol, incomingCalls, outgoingCalls for real call graphs)
- Probe step discovers available tools: LSP, ast.parse, pyright, tsc, ck, ZMCPTools
- JSONL sidecar output with expiry dates for search index ingestion

**Recommended tools section** in KINDERPOWERS.xml
- beads, beads-viewer, BeaconBay/ck, ultimate-bug-scanner

**Paper citations** in KINDERPOWERS.xml references section
- ToT (Yao 2023), GoT (Besta 2023), AGoT (Singh 2025), DeepConf (Zhao 2025)
- Cycles of Thought (Belem 2024), FaR (Qiu 2024), D-K in LLMs (2025)
- Brenner "Third Alternative" pattern

**CI** — GitHub Actions workflow for MCP server tests and GSD tools verification

**TOON format** — added as submodule at `specs/toon-format`

### Changed

- `metathinking` skill: removed mandatory/enforcement language, aligned with hints philosophy
- `KINDERPOWERS.xml`: expanded agent catalog with tools, when-to-use, output format
- `gsd-tools.cjs`: fixed MODULE_NOT_FOUND when invoked from different cwd (__dirname-relative requires)
- `install.sh` / `upgrade.sh` / README: fixed kp-seqthink → kp-sequential-thinking naming
- `marketplace.json`: synced version to 6.0.0 → 6.1.0
- `plugin.json`: added optionalComponents for MCP servers, toon-format to incorporates

## [6.0.0] — 2026-03-18

Kinderpowers v6.0: An operating system for AI agents.

### Added

**Lifecycle Engine (GSD)**
- Integrated [get-shit-done](https://github.com/davidjbauer/get-shit-done) v1.26.0 as the lifecycle engine
- 44 workflow definitions in `gsd/workflows/`
- 42 slash commands in `commands/gsd/` covering the full development lifecycle
- 16 GSD agents for autonomous project delivery
- `gsd-tools.cjs` runtime with state management, model resolution, and phase tracking

**New Skills**
- `strategic-planning` — discovery-before-creation, investigative vs implementation modes
- `metathinking` — sequential thinking with mandatory branching, confidence tracking, Brenner pattern
- `research-extraction` — harvest → extract → analyze → rank → verify pipeline with 3 routing modes
- `dispatching-to-runtimes` — prompt structuring for Gemini, GPT, and local model dispatch
- `team-orchestration` — Claude Code teams patterns, worker sizing, file domain separation, INJECT patterns
- `remembering-conversations` — conversation history search (moved from marketplace-only to repo)

**New Agents**
- `strategic-planner` — goal → discovery → phased plan
- `quality-gate` — adversarial verification, refuses to pass without evidence
- `team-coordinator` — orchestrates parallel Claude Code agent teams
- `research-extractor` — routes between idea extraction, usage evaluation, and deep integration

**Enforcement (Hookify Rules)**
- `verification-required` — blocks completion without verification evidence
- `discovery-before-creation` — warns before creating new files without searching
- `brainstorm-before-build` — warns before writing 100+ lines without design discussion
- All rules ship disabled — users opt-in

**Infrastructure**
- `setup.sh` — post-install script for symlinks and hookify rule installation
- `KINDERPOWERS.xml` — machine-readable manifest for AI agent consumption
- Progression model: L1 Coding Assistant → L2 Agentic Worker → L3 Team Orchestrator → L4 Dark Factory

### Enhanced

- `writing-plans` — added discovery-before-creation and extend-over-duplicate strategies
- `executing-plans` — added bead claim protocol, verify-before-assuming, parallel patterns, explicit-instructions mode
- `verification-before-completion` — added deep inspection checklist and agent delegation verification
- `plugin.json` — updated to v6.0.0 with expanded keywords

### Credits

- [superpowers](https://github.com/obra/superpowers) by Jesse Vincent — craft philosophy, skill format, scanner, hook system
- [get-shit-done](https://github.com/davidjbauer/get-shit-done) by Davíd Braun — lifecycle engine, commands, agents, workflows
- [hookify](https://github.com/QuantGeekDev/hookify) by Diego Perez — enforcement rule format
- [jw409](https://github.com/jw409) — progression model, agency-preserving philosophy, council mode, new skills and agents

## [5.1.0] — 2025-03-17

- Added adversarial-review, architecture, beads, requirements, retrospective skills
- Hub-and-spoke discovery via find-skills
- Scanner improvements

## [5.0.0] — 2025-03-08

- Initial kinderpowers release forked from superpowers v4.3.1
- 20 skills, 1 agent (code-reviewer), compulsion language scanner
