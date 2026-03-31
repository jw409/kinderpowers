# GSD Upstream Evolution Strategy

**Current**: kinderpowers vendors GSD v1.26.0-kp.1
**Upstream**: gsd-build/get-shit-done v1.30.0 (as of 2026-03-31)

Check current delta: `gh api repos/gsd-build/get-shit-done/compare/v1.26.0...main --jq '{ahead_by, files: (.files | length)}'`

## Strategy: Selective Merge + Diverge

Kinderpowers doesn't just track upstream — it adds value. The strategy is:
1. **Cherry-pick** high-value upstream features that benefit kinderpowers users
2. **Skip** runtime-specific changes we don't need (Windsurf, Antigravity, Copilot)
3. **Diverge** where kinderpowers has better approaches (LSP integration, MCP-native tools)

## Upstream Releases to Evaluate

### v1.27.0 — CHERRY-PICK (high value)

| Feature | Priority | Rationale |
|---------|----------|-----------|
| Multi-repo workspace support | P1 | Kinderpowers users often work across repos |
| `/gsd:fast` — trivial inline tasks | P1 | Removes friction for small tasks |
| `/gsd:review` — cross-AI peer review | P2 | Kinderpowers already has multi-perspective-review, evaluate overlap |
| Worktree-aware `.planning/` resolution | P1 | Critical for parallel agent work |
| Context window size awareness (1M+) | P1 | Opus 4.6 1M context is our primary model |
| Decision IDs for discuss-to-plan traceability | P2 | Improves plan quality |
| Stub detection in verifier/executor | P1 | Catches incomplete implementations |
| Security hardening (prompt injection guards) | P2 | Good hygiene |
| Consolidated `planningPaths()` helper | P1 | Reduces code duplication |

**Skip**: Cursor CLI runtime support (not a kinderpowers target)

### v1.28.0 — SELECTIVE

| Feature | Priority | Rationale |
|---------|----------|-----------|
| Workstream namespacing | P2 | Parallel milestone work is useful |
| `/gsd:forensics` | P3 | Nice-to-have post-mortem tool |
| CLAUDE.md compliance as plan-checker dim | P1 | Directly relevant to kinderpowers |
| Data-flow tracing in verification | P2 | Improves verification quality |
| Temp file reaper | P1 | Prevents /tmp accumulation |
| Wave-specific execution | P2 | Better parallel execution |

**Skip**: Multi-project workspace commands (different from multi-repo)

### v1.29.0 — SELECTIVE

| Feature | Priority | Rationale |
|---------|----------|-----------|
| Agent skill injection | P1 | Core kinderpowers capability — inject skills into subagents |
| Brownfield detection expanded | P2 | More ecosystem coverage |
| Frontmatter parser fixes | P1 | Bug fixes are always welcome |
| Agent workflows include `<available_agent_types>` | P1 | Improves agent spawning |

**Skip**: Windsurf runtime, i18n translations, repo rename references

### v1.30.0 — EVALUATE

| Feature | Priority | Rationale |
|---------|----------|-----------|
| GSD SDK (headless TypeScript) | P3 | Interesting for programmatic use but not urgent |
| Repo-local installation resolution fix | P1 | Bug fix kinderpowers already has |

## Kinderpowers Differentiators (Don't Merge These Away)

These are areas where kinderpowers diverges from upstream on purpose:

1. **MCP-native tooling** — kp-github and kp-sequential-thinking are Rust MCP servers. Upstream uses shell-based equivalents.
2. **LSP brownfield mapping** — `gsd-codebase-mapper` has LSP integration that upstream lacks. This is the primary differentiator.
3. **Parameterized agents/skills** — Upstream agents are fixed. Kinderpowers adds slider-based tuning.
4. **Multi-perspective review** — Upstream has `/gsd:review`, but kinderpowers has a richer council-based review system.
5. **Beads integration** — Persistent tracking across sessions. Upstream uses `.planning/STATE.md` only.
6. **Sequential thinking MCP** — Per-model tuning profiles, subagent spawn hints.

## Merge Process

For each cherry-picked feature:
1. Read the upstream diff for that feature
2. Adapt to kinderpowers directory structure (`gsd/bin/` not `get-shit-done/bin/`)
3. Preserve kinderpowers-specific modifications
4. Update `gsd/VERSION` to reflect selective merge (e.g., `1.26.0-kp.2`)
5. Update `gsd/CHANGELOG.md` with cherry-picked features

## Priority Order for Merge

1. Bug fixes and parser fixes (v1.29 frontmatter, v1.28 worktree)
2. P1 features from v1.27 (workspace, worktree, 1M context, stub detection)
3. P1 features from v1.28-v1.29 (CLAUDE.md compliance, agent skill injection, temp reaper)
4. P2 features as time permits
