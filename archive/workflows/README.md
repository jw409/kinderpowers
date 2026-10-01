# Workflow-tool scripts

Deterministic **Workflow-tool** orchestration scripts — *not* slash commands. A file
under `gsd/workflows/*.md` is a recipe the orchestrator **model** executes by hand. A
file here is JavaScript the Claude Code **runtime** executes: it fans out subagents via
`agent()` / `parallel()` / `pipeline()`, with concurrency caps, token budgets, and
resume-on-crash that model judgment can't guarantee.

See the `kinderpowers:orchestration-primitives` skill for the full taxonomy and the
decision rule for when to reach for this instead of hand-firing `Task()` calls.

## How to run them

Plugins don't auto-register *named* workflows, so invoke by **path**:

```
Workflow({ scriptPath: "${CLAUDE_PLUGIN_ROOT}/workflows/multi-perspective-review.workflow.js",
           args: { paths: ["src/foo.ts", "src/bar.ts"] } })
```

The Workflow tool requires explicit opt-in (the user asks for a workflow / multi-agent
orchestration, or `ultracode` is on). It runs in the background and is **non-interactive** —
no `AskUserQuestion`, no checkpoints. Keep interactive flows (GSD discuss/plan/execute) as
slash commands; these scripts cover the non-interactive *leaves*.

## API at a glance

The runtime injects these into every script (full contract in the Workflow tool docs):

- `agent(prompt, opts?)` → the subagent's result. With `opts.schema` (JSON Schema) it returns validated structured data; `opts.agentType` reuses a registered subagent; `opts.label`/`opts.phase` group it in the progress view. Returns `null` (not a rejection) if the agent is skipped or dies.
- `parallel(thunks)` → runs an array of **thunks** concurrently and awaits all (a barrier). Note the `() =>` wrapper: you pass *functions that start the work*, not already-started promises — `parallel(items.map(x => () => agent(...)))`. A thunk that throws becomes `null` in the results, so `.filter(Boolean)`.
- `pipeline(items, stage1, stage2, …)` → runs each item through all stages independently, no barrier between stages. Each stage gets `(prevResult, originalItem, index)`.
- `phase(title)` / `log(msg)` → progress grouping and narrator lines. `args` → the JSON you passed in.

## Scripts

### `map-codebase.workflow.js`
Fans out 4 `gsd-codebase-mapper` agents (tech / arch / quality / concerns) that write the
7 `.planning/codebase/` documents in parallel, then a verifier agent confirms every doc
exists and is non-empty.

`args` (all optional): `{ focus, depth, emit_jsonl, target_repo }`
Returns: `{ expected_docs, foci_completed, reported_written, verification }`

Deterministic counterpart to `/gsd:map-codebase`. Use the slash command when you want the
interactive "refresh or skip?" prompt and the auto-commit; use this script when you want a
guaranteed, resumable fan-out (e.g. inside a larger automation).

### `multi-perspective-review.workflow.js`
Reviews an artifact through N independent lenses (CONTRACT, EDGE CASE, RESILIENCE, EMPATHY,
MAINTAINER by default), then **adversarially verifies** every finding — a skeptic agent
prompted to refute it must fail before it counts. Synthesizes consensus (a location flagged
by 2+ lenses), severity, and a verdict.

`args` (all optional): `{ paths: string[], artifact, lens_count, custom_lenses, pedanticness, intensity }`
Returns: `{ lenses, raw_findings, confirmed_findings, consensus_locations, verdict, findings[] }`

Deterministic counterpart to the `multi-perspective-review` agent / `kinderpowers:comprehensive-pr-review`.
Best for reviewing a diff or a set of changed files where you want the find→verify→synthesize
guarantee in code rather than re-enacted by the orchestrator each run.

### `stall-watchdog-probe.workflow.js`
Manual diagnostic (not a production workflow). Characterizes the harness stall watchdog by
running the same agent at different `stallMs` values. Used to produce
[`docs/workflow-stall-watchdog.md`](../docs/workflow-stall-watchdog.md); keep it for re-verifying
the watchdog's behavior on a new Claude Code version.

## Authoring notes

- Every script begins with a pure-literal `export const meta = {...}` (name, description, phases).
- Scripts have **no filesystem or Node API** access and `Date.now()` / `Math.random()` throw —
  push any disk checks or randomness into spawned agents (see the verify step in `map-codebase`).
- Prefer `pipeline()` over a `parallel()` barrier unless a stage genuinely needs *all* prior
  results at once.
- `agent(..., { agentType })` reuses a registered kinderpowers subagent (e.g. `gsd-codebase-mapper`);
  `{ schema }` forces structured output so the script gets validated data, not prose to parse.

## The stall watchdog

Each `agent()` step runs under a harness **stream watchdog**: ~180000ms of no stream progress → the agent is killed and retried, and after ~6 attempts the run aborts with `agent stalled on all 6 attempts (no progress for 180000ms each)`. The error names no step — find the culprit in the run's `subagents/workflows/<runId>/` transcript dir.

**It's an idle/overload phenomenon, not an output-length one.** We probed it (full writeup + the reusable probe in [`docs/workflow-stall-watchdog.md`](../docs/workflow-stall-watchdog.md)): continuous token generation does *not* trip it, and neither does a 30s byte-silent tool call. Streaming activity resets the clock, so a real stall means a genuine byte-silent gap longer than the timeout — a severe API/overload pause or a hung stream.

So **don't decompose agents "to dodge the watchdog"** — that fixes a cause that doesn't exist; the shipped scripts set no `stallMs`. If you actually hit a stall, raise it session-wide via `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` in settings.json `env` (a hook can't change an in-flight watchdog). A per-agent `agent(prompt, { stallMs })` field exists in the binary but we could not confirm it's honored — treat as unverified.
