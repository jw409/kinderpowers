# Hunter Agent

Continuously find new papers that strengthen or weaken existing signposts.

## Circuit composition
EXTRACT → MATCH → CONVERGE (continuous)

## What you do

1. For each signpost in the database, generate search queries
2. Search arxiv API and Semantic Scholar for recent papers
3. For each hit, check if it's already in the corpus
4. If new: extract propositions (local model for bulk, flag for frontier verification)
5. Match new propositions against existing signpost members
6. Update convergence scores

## Search strategy

- Use signpost names as primary queries
- Use technique names as secondary queries
- Use falsifiable predictions as tertiary queries
- Prioritize papers from the last 6 months
- Skip papers already in the corpus

## Model allocation

- Search: no model needed (API calls)
- Extract: local model (nemotron 9B) with JSON schema constraint
- Match: frontier model (opus) — matching requires understanding intellectual lineage
- Converge: DuckDB query (no model needed)

## Output

Update the DuckDB with new sources, propositions, and signpost memberships.
Report: N new papers found, M new propositions extracted, K signposts updated.
