export const meta = {
  name: 'map-codebase',
  description: 'Map a codebase with 4 parallel mapper agents, then verify the 7 output documents',
  phases: [
    { title: 'Map', detail: '4 gsd-codebase-mapper agents write .planning/codebase/ docs in parallel' },
    { title: 'Verify', detail: 'confirm every expected document exists and is non-empty' },
  ],
}

// Deterministic port of the /gsd:map-codebase fan-out. The slash-command version
// asks the orchestrator model to remember to spawn 4 agents, wait, and collect.
// Here the fan-out is code: parallel() guarantees all four run and are collected,
// and a final agent verifies the outputs (scripts have no filesystem access).
//
// args (all optional): { focus, depth, emit_jsonl, target_repo }
const cfg = args || {}
const focusNote = cfg.focus
  ? `Focus specifically on this subsystem/area: ${cfg.focus}.`
  : 'Map the whole codebase.'
const depth = cfg.depth || 'standard'
const emit = cfg.emit_jsonl ? 'emit_jsonl=true' : 'emit_jsonl=false'
const repo = cfg.target_repo ? `target_repo=${cfg.target_repo}` : 'target_repo=cwd'

const FOCI = [
  { key: 'tech', docs: ['STACK.md', 'INTEGRATIONS.md'] },
  { key: 'arch', docs: ['ARCHITECTURE.md', 'STRUCTURE.md'] },
  { key: 'quality', docs: ['CONVENTIONS.md', 'TESTING.md'] },
  { key: 'concerns', docs: ['CONCERNS.md'] },
]

const CONFIRM_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    focus: { type: 'string' },
    docs_written: { type: 'array', items: { type: 'string' } },
    summary: { type: 'string' },
  },
  required: ['focus', 'docs_written'],
}

phase('Map')
const confirmations = await parallel(FOCI.map((f) => () =>
  agent(
    `You are a GSD codebase mapper with focus=${f.key}. ${focusNote}\n\n` +
    `Parameters: depth=${depth}, ${emit}, ${repo}.\n` +
    `Explore thoroughly, then write your focus-area document(s) DIRECTLY to .planning/codebase/: ${f.docs.join(', ')}.\n` +
    `Follow the gsd-codebase-mapper protocol exactly. Return only a confirmation (focus + docs_written + one-line summary) — do NOT return document contents.`,
    { label: `map:${f.key}`, phase: 'Map', agentType: 'gsd-codebase-mapper', schema: CONFIRM_SCHEMA },
  ),
))

const done = confirmations.filter(Boolean)
const written = done.flatMap((c) => c.docs_written || [])
log(`Mappers completed ${done.length}/${FOCI.length} foci; reported ${written.length} documents written`)

phase('Verify')
const expected = FOCI.flatMap((f) => f.docs)
const VERIFY_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    present: { type: 'array', items: { type: 'string' } },
    missing: { type: 'array', items: { type: 'string' } },
    empty: { type: 'array', items: { type: 'string' } },
    ok: { type: 'boolean' },
  },
  required: ['present', 'missing', 'ok'],
}
const verdict = await agent(
  `Check the directory .planning/codebase/. Report which of these expected documents exist and are non-empty: ${expected.join(', ')}.\n` +
  `List present, missing, and present-but-empty files. Set ok=true only if every expected document is present and non-empty.`,
  { label: 'verify:docs', phase: 'Verify', schema: VERIFY_SCHEMA },
)

return {
  // Single unambiguous signal: every focus completed AND the verifier found every doc present and non-empty.
  ok: done.length === FOCI.length && !!(verdict && verdict.ok),
  expected_docs: expected,
  foci_completed: done.length,
  reported_written: written,
  verification: verdict,
}
