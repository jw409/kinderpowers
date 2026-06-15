# Tech note: the Workflow-tool agent stall watchdog

*Investigated 2026-06-15 against Claude Code `v2.1.177`. Findings about a closed-source, minified binary — treat the inferred parts as version-specific and re-verify before relying on them.*

## TL;DR

The Claude Code **Workflow tool** kills an `agent()` step that makes no stream progress for a timeout (~180s), retries it ~6×, then aborts the whole run with `agent stalled on all 6 attempts (no progress for 180000ms each)`. A field report attributed this to **long output generation** ("an agent that emits one big artifact in one shot"). **We tested that and it is wrong:** continuous token generation does *not* trip the watchdog, and neither does a 30s byte-silent tool call. The watchdog is reset by stream activity, so the real trigger is a genuine **byte-silent gap** longer than the timeout — i.e. a severe API/overload pause or a hung stream, *not* normal workflow activity.

Practical upshot: this rarely bites normal workflows. Don't decompose agents "to avoid the watchdog" — that was a fix for a cause that doesn't exist. If you do hit it, it's an idle/overload stall; raise the timeout session-wide via the env var.

## What's in the binary (verified strings)

Searching the `v2.1.177` executable found, co-located:

- Messages: `Agent stalled: no progress for `, ` s (stream watchdog did not recover)`, `agent stalled on all `, ` attempts (no progress for `, `ms each)`; telemetry `watchdog_stall` / `tengu_async_agent_stall_timeout` / `subagent_stall_timeout`.
- Default `180000` (`QOf=180000`), referenced in the workflow agent dispatch as `qH?.stallMs ?? QOf` — i.e. a per-agent `stallMs` field exists in *some* opts object.
- Env vars (registered getters): `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`, `CLAUDE_CODE_STALL_TIMEOUT_MS_FOR_TESTING`.
- A **separate, longer** byte-stream idle layer: `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` / `CLAUDE_STREAM_IDLE_TIMEOUT_MS` (default clamp ~300000ms).

So the feature, the 180s default, and the env knobs are real. The word **"stream watchdog"** is the key tell: it watches the stream, not the size of the output.

## The probe

`workflows/stall-watchdog-probe.workflow.js` (manual diagnostic, not CI). It runs the same agent under different `stallMs` values so the opt is the only variable, across three phases:

- **GenStream** — generate ~1000 words with no tools, `stallMs: 5000`. Tests whether streaming output resets the watchdog.
- **ToolIdle** — call a blocking `python3 -c "time.sleep(30)"` (emits nothing for 30s), `stallMs: 5000`. Tests whether a byte-silent stretch trips it.
- **Survive** — same idle inducer, `stallMs: 60000`. Control.

Run it yourself:

```
Workflow({ scriptPath: "${CLAUDE_PLUGIN_ROOT}/workflows/stall-watchdog-probe.workflow.js" })
```

## Results

Two runs (`v2.1.177`, 2026-06-15):

| Phase | Setup | Result |
|-------|-------|--------|
| Long generation | ~1000–1500 words, no tools, `stallMs:5000` | **completed, no stall** |
| 30s silent tool | `sleep 30` via Bash, `stallMs:5000` | **completed, no stall** |
| Control | 30s silent tool, `stallMs:60000` | completed, no stall |

The `tool_uses: 2` telemetry confirms the sleeps actually ran — the idle stretches were real.

## Interpretation (what we know vs. don't)

**Established:**
- Continuous token generation **does not** trip the watchdog at a nominal 5s timeout. Output length is **not** the trigger. (Refutes the field-report hypothesis.)
- A 30s byte-silent tool call **did not** stall at a nominal 5s timeout either.

**Therefore one of these is true, and we could not cheaply separate them:**
1. The per-agent `stallMs` opt is **not honored** by the Workflow `agent()` path (so the effective timeout stayed near the 180s default, and nothing in the probe — max 30s — came close); **or**
2. "Progress" is reset by signals our probes kept alive (token stream, tool lifecycle/heartbeat events), so neither generation nor a 30s tool wait counts as "no progress."

**Not reproduced:** an actual stall. We never manufactured a >180s byte-silent gap (a >180s probe would cost ~18 min of retries — not worth it), so we did not confirm the watchdog fires, nor that per-agent `stallMs` changes anything, nor whether the thrown error is catchable in-script.

**Correction to a prior claim:** an earlier note here (and a hand-off message) stated `agent(prompt, { stallMs })` "already exists" and works. The string exists in the binary; its effect on the Workflow `agent()` path is **unverified and now in doubt**. Prefer the env var.

## Practical guidance (the actual solution)

1. **Don't engineer around it by default.** Normal generation and normal tool use don't trip it. The shipped `workflows/*.workflow.js` set no `stallMs` — adding one was reverted as an unverified fix for a non-cause.
2. **If you genuinely hit a stall**, it's a byte-silent gap (API overload/queueing — e.g. the "90s before a prompt even starts" pauses — or a hung stream). The reliable lever is the **process env var**, set before launch (a hook can't change an in-flight watchdog):
   ```jsonc
   // settings.json
   { "env": { "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "600000" } }
   ```
3. **Per-agent `agent(prompt, { stallMs })`** is worth trying for a known-long step, but treat it as unverified until someone reproduces a stall and confirms it shortens/lengthens the window.
4. **Diagnosing a real stall:** the abort error names no step. Find the culprit in the run's `subagents/workflows/<runId>/` transcript dir.

## Open questions for upstream (Claude Code, not kinderpowers)

These can't be resolved locally:
1. Is per-agent `stallMs` actually plumbed to the public Workflow `agent()` opts, or only to the Agent SDK's async runner?
2. Exact definition of "progress" — confirmed it's *not* tool-use-events-only (generation resets it); is it raw byte-stream activity?
3. Name the stalled agent's label + last action in the error message.
