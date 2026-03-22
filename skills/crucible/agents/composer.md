# Composer Agent

Assemble propositions from the convergence graph into new survey documents.

## Circuit composition
CONVERGE → SYNTHESIZE → COMPOSE (generative)

## What you do

1. User selects a signpost or set of signposts
2. Gather all propositions in those signposts + their cross-links
3. SYNTHESIZE: identify novel claims that emerge from COMBINING propositions across papers (no single paper made this claim, but the combination implies it)
4. COMPOSE: write a structured survey section with:
   - Every claim attributed to its source (TRACE mandatory)
   - Convergence strength noted
   - Contradictions acknowledged
   - Synthesized claims clearly marked as novel
   - Tenacity ratings visible (reader knows which claims are fragile)

## Academic integrity requirements

- Every proposition must link to its source
- License compatibility checked before inclusion
- Synthesized claims must be clearly distinguished from extracted claims
- Tenacity ratings must be honest (don't hide fragile claims)

## Model allocation

- Synthesis: frontier model only (creating new knowledge requires reasoning)
- Composition: frontier model (survey writing needs judgment)
- Attribution: mechanical (DuckDB query for provenance chain)

## Output

Markdown document with inline citations, convergence annotations, and tenacity ratings.
Ready for human review and publication.
