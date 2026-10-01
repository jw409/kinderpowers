export const meta = {
  name: 'stall-watchdog-probe',
  description: 'Manual diagnostic: characterize the per-agent stall watchdog — what resets it, and whether stallMs raises the ceiling',
  phases: [
    { title: 'GenStream', detail: 'continuous token generation at a tiny stallMs — does streaming reset the watchdog?' },
    { title: 'ToolIdle', detail: 'byte-silent blocking tool call at a tiny stallMs — does stream-idle trip it?' },
    { title: 'Survive', detail: 'same idle inducer at a high stallMs — should complete' },
  ],
}

// MANUAL DIAGNOSTIC — not CI. Run by hand:
//   Workflow({ scriptPath: "${CLAUDE_PLUGIN_ROOT}/workflows/stall-watchdog-probe.workflow.js" })
// Self-contained: no env vars. The per-agent stallMs opt is the only knob varied, so each
// phase's outcome isolates one question. The ToolIdle phase deliberately burns the watchdog
// (the harness retries ~6x), so it is slow by design.
//
// args (optional): { tripMs, surviveMs, idleSec, words }
const cfg = args || {}
const tripMs = Number(cfg.tripMs) || 5000          // tiny — should stall on a real idle gap
const surviveMs = Number(cfg.surviveMs) || 60000   // generous — should tolerate the idle gap
const idleSec = Number(cfg.idleSec) || 30          // length of the byte-silent stretch
const words = Number(cfg.words) || 1000

// Continuous-generation probe: faithful to the "big single artifact" trigger from the field
// report. If streaming tokens reset the watchdog, this survives even a 5s timer.
const genPrompt =
  `Write an original ${words}-word essay on the history of timekeeping, as one continuous message. ` +
  `Do NOT use any tools. Just write prose until you reach about ${words} words.`

// Idle probe: a blocking tool call that emits nothing for idleSec seconds (python sleep, not
// the shell `sleep` builtin, to dodge any foreground-sleep guard). The model is idle, waiting
// on the tool — the cleanest way to manufacture a byte-silent stretch on demand.
const idlePrompt =
  `Use the Bash tool to run exactly this command: python3 -c "import time; time.sleep(${idleSec})". ` +
  `Emit no text before the tool call. After it returns, reply "done".`

async function probe(label, ph, prompt, stallMs) {
  try {
    await agent(prompt, { label, phase: ph, stallMs })
    return `completed, no stall (stallMs=${stallMs}ms)`
  } catch (e) {
    return `STALLED & CATCHABLE (stallMs=${stallMs}ms): ${String(e).slice(0, 140)}`
  }
}

phase('GenStream')
const genStream = await probe('gen-stream', 'GenStream', genPrompt, tripMs)
log(`GenStream — tiny stallMs, continuous output: ${genStream}`)

phase('ToolIdle')
const toolIdle = await probe('tool-idle', 'ToolIdle', idlePrompt, tripMs)
log(`ToolIdle — tiny stallMs, ${idleSec}s silent tool: ${toolIdle}`)

phase('Survive')
const survive = await probe('idle-high-stall', 'Survive', idlePrompt, surviveMs)
log(`Survive — high stallMs, ${idleSec}s silent tool: ${survive}`)

return { tripMs, surviveMs, idleSec, words, genStream, toolIdle, survive }
