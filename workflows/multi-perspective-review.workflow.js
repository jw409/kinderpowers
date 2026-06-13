export const meta = {
  name: 'multi-perspective-review',
  description: 'Review an artifact through N independent lenses, adversarially verify each finding, synthesize a verdict',
  phases: [
    { title: 'Review', detail: 'one agent per lens, independent, no groupthink' },
    { title: 'Verify', detail: 'adversarially refute each finding before it counts' },
  ],
}

// Deterministic port of the multi-perspective-review agent's council flow,
// built on the Workflow tool's flagship pattern: dimensions -> find -> adversarially
// verify. pipeline() means each lens's findings start verifying the moment that lens
// finishes — a slow lens never blocks a fast one. A finding only survives if a skeptic
// agent, prompted to refute it, fails to.
//
// args (all optional): { paths: string[], artifact, lens_count, custom_lenses,
//                         pedanticness, intensity }
const cfg = args || {}
const paths = Array.isArray(cfg.paths) ? cfg.paths : cfg.paths ? [cfg.paths] : []
const pedanticness = cfg.pedanticness || 'medium'
const intensity = cfg.intensity || 'normal'

const DEFAULT_LENSES = [
  { name: 'CONTRACT', prompt: 'Does it actually do what it claims? Test every documented guarantee and example. If a doc or comment says X, find where the code does not.' },
  { name: 'EDGE CASE', prompt: 'What breaks on unusual inputs — empty, huge, unicode, injection, boundary values, type confusion, missing arguments?' },
  { name: 'RESILIENCE', prompt: 'What happens when things go sideways — partial failure, an agent returning null, timeout, concurrent writes, missing files, malformed data?' },
  { name: 'EMPATHY', prompt: 'What jargon or implicit knowledge is unexplained? Where would a newcomer get stuck? Are names, errors, and docs clear?' },
  { name: 'MAINTAINER', prompt: 'Is this consistent with surrounding conventions? Look for duplication, dead code, and things that will rot or surprise the next editor.' },
]

const requested = Array.isArray(cfg.custom_lenses)
  ? cfg.custom_lenses.filter((n) => n && String(n).trim())
  : []
// Clamp lens_count to [1, DEFAULT_LENSES.length] so negative/zero/string/over-count values can't drop or over-request lenses.
const lensCount = Math.max(1, Math.min(Number(cfg.lens_count) || 5, DEFAULT_LENSES.length))
const lenses = requested.length
  ? requested.map((n) => ({ name: String(n).toUpperCase(), prompt: `Review strictly from the ${n} perspective.` }))
  : DEFAULT_LENSES.slice(0, lensCount)

const target = paths.length
  ? `Review these files (read them first):\n${paths.map((p) => '- ' + p).join('\n')}`
  : (cfg.artifact || 'Review the current uncommitted work. Run `git diff HEAD` to see it.')

const FINDINGS_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    lens: { type: 'string' },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        properties: {
          title: { type: 'string' },
          location: { type: 'string', description: 'file:line or file path' },
          severity: { type: 'string', enum: ['blocking', 'important', 'minor', 'nit'] },
          evidence: { type: 'string' },
        },
        required: ['title', 'location', 'severity', 'evidence'],
      },
    },
  },
  required: ['lens', 'findings'],
}

const VERDICT_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    real: { type: 'boolean' },
    reasoning: { type: 'string' },
    adjusted_severity: { type: 'string', enum: ['blocking', 'important', 'minor', 'nit', 'false-positive'] },
  },
  required: ['real', 'reasoning'],
}

const results = await pipeline(
  lenses,
  // Stage 1 — review through this lens, independently.
  (lens) => agent(
    `You are the ${lens.name} review lens. ${lens.prompt}\n\n${target}\n\n` +
    `Pedanticness=${pedanticness} (low=blocking only, medium=material issues, high=everything, maximum=even style). Intensity=${intensity}.\n` +
    `Report concrete findings with file:line evidence. If the work is sound from your angle, return an empty findings array — never invent issues to fill a quota.`,
    { label: `review:${lens.name}`, phase: 'Review', schema: FINDINGS_SCHEMA },
  ),
  // Stage 2 — adversarially verify each finding from this lens (starts as soon as the lens returns).
  (review) => parallel(((review && review.findings) || []).map((f) => () =>
    agent(
      `Adversarially verify this review finding. Try to REFUTE it. Read the cited location and its surrounding context first.\n\n` +
      `Lens: ${review.lens}\nTitle: ${f.title}\nLocation: ${f.location}\nClaimed severity: ${f.severity}\nEvidence: ${f.evidence}\n\n` +
      `Default to real=false if the finding is speculative, already handled elsewhere, or not supported by the actual code. ` +
      `Set real=true only if you can confirm it against the code. Give an adjusted_severity.`,
      { label: `verify:${f.title.slice(0, 40)}`, phase: 'Verify', schema: VERDICT_SCHEMA },
    ).then((v) => ({ ...f, lens: review.lens, verdict: v })),
  )),
)

const all = results.flat().filter(Boolean)
// A skeptic that tags a finding 'false-positive' has refuted it — drop it even if it returned real=true.
const confirmed = all.filter((f) => f.verdict && f.verdict.real && f.verdict.adjusted_severity !== 'false-positive')

// Consensus: a location flagged by 2+ distinct lenses is high-confidence.
const byLoc = {}
for (const f of confirmed) {
  const key = (f.location || '').split(':')[0] || f.title
  byLoc[key] = byLoc[key] || []
  byLoc[key].push(f)
}
const consensus = Object.entries(byLoc)
  .filter(([, v]) => new Set(v.map((x) => x.lens)).size >= 2)
  .map(([k]) => k)

const rank = (s) => ({ blocking: 0, important: 1, minor: 2, nit: 3 }[(s || '').toLowerCase()] ?? 4)
const finalSev = (f) => f.verdict.adjusted_severity && f.verdict.adjusted_severity !== 'false-positive'
  ? f.verdict.adjusted_severity
  : f.severity
confirmed.sort((a, b) => rank(finalSev(a)) - rank(finalSev(b)))

const blocking = confirmed.filter((f) => finalSev(f) === 'blocking')

return {
  lenses: lenses.map((l) => l.name),
  raw_findings: all.length,
  confirmed_findings: confirmed.length,
  consensus_locations: consensus,
  verdict: blocking.length ? 'Needs fixes — blocking issues' : confirmed.length ? 'Minor issues only' : 'Ready',
  findings: confirmed.map((f) => ({
    lens: f.lens,
    title: f.title,
    location: f.location,
    severity: finalSev(f),
    evidence: f.evidence,
    why_real: f.verdict.reasoning,
  })),
}
