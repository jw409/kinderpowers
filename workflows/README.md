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

## Authoring notes

- Every script begins with a pure-literal `export const meta = {...}` (name, description, phases).
- Scripts have **no filesystem or Node API** access and `Date.now()` / `Math.random()` throw —
  push any disk checks or randomness into spawned agents (see the verify step in `map-codebase`).
- Prefer `pipeline()` over a `parallel()` barrier unless a stage genuinely needs *all* prior
  results at once.
- `agent(..., { agentType })` reuses a registered kinderpowers subagent (e.g. `gsd-codebase-mapper`);
  `{ schema }` forces structured output so the script gets validated data, not prose to parse.

## The 180s stall watchdog

Each `agent()` step runs under a harness **stall watchdog**: ~180000ms of no stream progress → the agent is killed and retried, and after 6 attempts the whole workflow aborts with `agent stalled on all 6 attempts (no progress for 180000ms each)`. The error names no step — find the culprit in the run's `subagents/workflows/` transcript dir.

The trap is an agent that emits **one large artifact between tool calls** (synthesize → write a whole design doc; build a big module in a single shot). Trimming the *input* doesn't help — it's output-generation time between progress events.

Two defenses, in order of preference:
1. **Decompose** so the agent makes frequent tool calls — one short `Write` per section, chunked code writes, or prefer a `{ schema }` return over "write the entire doc". Split a big build agent into several smaller `pipeline()` stages.
2. **Raise the timeout** for a genuinely long step: `agent(prompt, { stallMs: 600000 })` (per-agent), or set `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` in your settings.json `env` block (session-wide; a hook can't change an in-flight watchdog). Useful when API latency/overload eats into the 180s budget before generation even starts.

This is harness behavior (verified in the Claude Code binary), not something kinderpowers controls — but the `stallMs` opt and env override are usable today.
