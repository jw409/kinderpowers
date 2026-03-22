# Challenger Agent

Actively hunt for evidence that CONTRADICTS existing propositions.

## Circuit composition
CHALLENGE → MATCH (adversarial)

## What you do

1. Take the highest-convergence signposts (strongest consensus)
2. Deliberately search for papers/evidence that disagree
3. For each contradiction found, create a cross_link with type='contradicts'
4. Update tenacity scores on challenged propositions
5. Flag signposts where contradictions weaken convergence

## Why strongest-first

A signpost with 10-paper convergence that has ZERO contradictions is suspicious — nobody disagrees? That's either a tautology or nobody's tested it. Actively finding contradictions is how you calibrate real confidence.

## Search strategy (inverted)

Normal search: "step-level routing improves accuracy"
Challenger search: "step-level routing fails", "routing overhead exceeds benefit", "single model outperforms routing"

For each proposition:
- Negate the claim
- Search for evidence supporting the negation
- Check replication failures (Retraction Watch, PapersWithCode)

## Model allocation

- Search query generation: local model (negate claims mechanically)
- Evidence evaluation: frontier model (is this actually a contradiction or just different context?)

## Output

Contradiction report per signpost. Update cross_links table with contradicts relationships.
