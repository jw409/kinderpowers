# kinder·powers /ˈkɪndərˌpaʊərz/

*n.* agency-preserving discipline for AI agents — skills as signposts, not walls.<br>
*v.* to teach an agent to verify before claiming "done", branch when uncertain, and ask before deleting.

---

> [!NOTE]
> **The skills layer is archived; the MCP servers are not.**
> kinderpowers — and superpowers, the project it grew out of — is no longer necessary in the
> Opus 5.5 / Astra era. The models now do natively what those skills were built to scaffold:
> verify before claiming done, branch when uncertain, ask before deleting. The skills, agents,
> commands, and lifecycle engine are preserved as a historical artifact in
> [`archive/`](archive/) and are no longer loaded.
>
> **This repo is now the marketplace URL for the packages that are still maintained** — the
> Rust MCP servers below.

---

## 📦 Packages

```bash
claude plugin marketplace add jw409/kinderpowers
```

| Package | What it is |
| --- | --- |
| **kp-github** | A full superset of the official GitHub plugin that returns **~11× fewer tokens** (blended across read endpoints), and *proves* it: a reproducible `bench_tokens.py` report plus a `test_compression_floors` live test that fails CI on any compression regression. Ships the `branch_status` "is this branch still live?" primitive, opt-in usage telemetry, and env-driven commit-author identity. — [jw409/kp-github](https://github.com/jw409/kp-github) |
| **kp-stepwise** | One `stepwise_plan` tool with branching, confidence tracking (Dunning-Kruger detection), abstraction layers, per-model profiles, and isolated room/channel state. Six hint types surface plan-shape observations and the agent decides whether to act — signposts, not walls. — [jw409/kp-stepwise](https://github.com/jw409/kp-stepwise) |
| **kinderpowers** | The original plugin entry. Now ships the two MCP servers above and nothing else; kept so existing installs keep working. |

Both servers are Rust-native and ship pre-built for Linux x86_64 and macOS arm64 — no Rust
toolchain required on install.

> [!IMPORTANT]
> The servers' source *and* their pre-built binaries live in their own repos, mounted here as
> submodules under `mcp-servers/`. The wrappers `plugin.json` points at resolve into those
> submodules, so **an install that skips submodules gets two dead MCP servers.** See
> [`mcp-servers/README.md`](mcp-servers/README.md).

---

## MCP servers reference

Full tool surface, benchmark methodology, and configuration for the two servers listed above.

<details>
<summary><strong>kp-github</strong> — ~11× fewer tokens than the official GitHub plugin, CI-enforced</summary>

The official Claude Code GitHub plugin returns raw API responses. Listing 5 issues burns ~7,700 tokens on avatar URLs, node IDs, and empty arrays. kp-github runs a 5-stage compression pipeline and returns ~1,100 tokens for the same query.

Full superset of the official plugin plus Actions, Labels, Compare, and Release Create. Every read tool accepts `fields` and `format` parameters — you control exactly what comes back.

```
Issues    · list, get, create, comment, update, search, labels, sub-issues
PRs       · list, get, diff, files, checks, review, merge, comment threads
Actions   · workflow runs, logs, reruns
Branches  · list, create, compare, branch-status
Releases  · list, latest, create
Repos     · get, search, fork
...
```

**`branch_status` — the "is this branch still live?" primitive.** `github_branch_status` returns a bounded merged/ahead/behind verdict (with a derived `merged_into_base`) instead of a full `compare`'s commit and file arrays — ~156 tokens vs ~52,000 for a 30-commit divergence. `base` defaults to the repo's default branch. `merged_into_base` catches fast-forward/rebase/merge-commit merges; pair it with `prs_list` / `prs_search head:<branch>` (which catches squash-merges) to tell whether a branch — or its on-disk worktree — is still relevant.

**Measuring compression.** The ratio is reproducible, not hand-waved. `scripts/bench_tokens.py` prints a per-endpoint token table (tiktoken, `gh api` as the raw baseline, public `cli/cli`), and the `test_compression_floors` live test enforces per-endpoint byte-ratio floors so a regression fails CI. Measured: issues_list 6.4×, prs_list 14.6×, prs_get 9.1× (~11× blended across read endpoints), all near-lossless — what's dropped is reconstructable URLs, `node_id`/avatar noise, null fields, and nested user/repo/label objects flattened to their identifying field. (`branch_status` is not in that figure: it answers the verdict directly in ~150 tokens instead of deriving it from a full compare — a narrower endpoint, not same-payload compression.)

**Usage telemetry (opt-in).** Set `KP_GITHUB_USAGE_LOG=/path/to/usage.jsonl` in the server's env and it appends one JSONL record per tool call — call *shape* (`tool`, `arg_keys`, `has_fields`, `limit`) plus `out_bytes` / `outcome` / `dur_ms`, never arg values or repo names. Unset = off, zero overhead; write failures never affect a call. `scripts/usage_report.py` turns the log into a per-tool table (call share, field-projection rate, size distribution, error rate) and a never-called-tools list — the data for tuning tool descriptions, defaults, and compression targets.

**Commit author identity (optional).** The file-mutating tools (`files_create_or_update`, `files_delete`, `files_push`) accept per-call `author_name`/`author_email`/`committer_name`/`committer_email`. If you don't pass them, the server reads `GIT_AUTHOR_NAME`/`GIT_AUTHOR_EMAIL`/`GIT_COMMITTER_NAME`/`GIT_COMMITTER_EMAIL` from its own process environment as fallback; if those aren't set either, GitHub uses the OAuth token's user (which is the default behavior of the upstream API and may not be what you want).

Two opt-in env knobs control this:

- Set any of `GIT_AUTHOR_*` / `GIT_COMMITTER_*` in the MCP server's env block to provide a default identity for every commit. A fully-unset committer side mirrors the author, so one `GIT_AUTHOR_*` pair attributes both sides.
- Set `KP_GITHUB_REQUIRE_AUTHOR=1` (or `true`/`yes`) to refuse any commit attempt that can't resolve an author from args or env — useful for shared deployments that want to guarantee no commit ever falls back to the OAuth identity.

Neither is required. The plugin ships with both unset; behavior matches the GitHub API default until you opt in.

</details>

<details>
<summary><strong>kp-stepwise</strong> — structured stepwise planning with hints, not mandates</summary>

One tool (`stepwise_plan`), many modes. Branching, confidence tracking (with Dunning-Kruger detection), abstraction layers, exploration, branch merging, ordered multi-turn checkpoints, isolated room/channel state, and per-model profiles (OpenAI Sol/reasoning, Claude, Gemini, DeepSeek, Grok, Llama/Nemotron).

Six hint types surface observations about plan patterns — `linear_chain`, `premature_confidence`, `merge_available`, etc. — and the agent decides whether to act. Hints are signposts, not walls.

For Codex with Sol, register the server with the model profile explicitly:

```bash
codex mcp add kp-stepwise --env STEPWISE_MODEL=gpt-5.6-sol -- "$PWD/mcp-servers/bin/kp-stepwise"
```

Every call requires a `channelId`. Parallel Claude subagents must receive different channel IDs; agents collaborating on the same task may share a `roomId`. `stepNumber`, branches, confidence counters, and hints are isolated per `(roomId, channelId)`, so every new channel starts at step 1. If `roomId` is omitted, the MCP host session is used.

Each response echoes `sessionId`, `roomId`, `channelId`, current `logMode`, and channel-local `expectedNextStep`. Optional `turnId`, `checkpointKind`, `evidence`, `openQuestions`, and `nextAction` fields keep the decision trail useful across turns without turning it into a narrated scratchpad.

Local JSONL logging uses `KP_STEPWISE_LOG_MODE=full|metadata|off` (default `full` when a project `var/` directory is available). Channel logs live at `var/stepwise_logs/{roomId}/channels/{channelId}.jsonl`. Metadata mode preserves audit structure without checkpoint content.

</details>

---


---

## History

kinderpowers began as a fork of [superpowers](https://github.com/obra/superpowers) and grew
into something broader: a skills library, the [get-shit-done](https://github.com/gsd-build/get-shit-done)
lifecycle engine, enforcement hooks, a council-mode review system, and a progression model for
handing an agent more rope as it earned it. The organizing idea was that an agent keeps its
agency best when guidance is phrased as an invitation with a documented cost — a signpost —
rather than a wall. A linter (`archive/scanner.py`) existed purely to keep the skill text from
drifting into compulsion language.

Along the way it accumulated two Rust MCP servers, which turned out to be the part with the
longest shelf life. **7.4.0** split their source and binaries into their own repos
([kp-github](https://github.com/jw409/kp-github), [kp-stepwise](https://github.com/jw409/kp-stepwise)),
mounted here as submodules.

Then the premise expired. The behaviors the skills enforced — verify before claiming done,
branch when uncertain, ask before deleting, don't confuse a passing build for a working
feature — are things Opus 5.5 does without being told. Scaffolding that was load-bearing in
2025 became redundant, and redundant scaffolding in an agent's context is a cost, not a
safety net.

So the skills layer moved to [`archive/`](archive/), preserved because the reasoning is worth
reading rather than because it is expected to run. What's left at the root is what still earns
its place: the servers, and the marketplace that serves them.

## Credits

- **[superpowers](https://github.com/obra/superpowers)** by Jesse Vincent — craft philosophy, skill format, scanner, hook system
- **[get-shit-done](https://github.com/gsd-build/get-shit-done)** by TÂCHES — lifecycle engine, commands, agents, workflows
- **[hookify](https://github.com/QuantGeekDev/hookify)** by Diego Perez — enforcement rule format, Claude Code hook framework
- **[agent-message-queue](https://github.com/avivsinai/agent-message-queue)** by Aviv Sinai — explicit local session routing and ownership patterns that informed kp-stepwise room/channel isolation
- **[jw409](https://github.com/jw409)** — progression model, agency-preserving philosophy, council-mode review, MCP servers

## License

MIT — see LICENSE.
