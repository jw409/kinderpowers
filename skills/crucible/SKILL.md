---
name: crucible
description: Crucible — research intelligence framework — ingest papers, extract propositions, find convergence, explode weak hypotheses, compose surveys
triggers:
  - crucible
  - signpost
  - paper analysis
  - research intelligence
  - proposition extraction
  - convergence analysis
  - explode paper
---

# Crucible: Research Intelligence Framework

12 circuits in 3 tiers composing into 7 offensive agents.

## File Structure (N levels)

A crucible instance is a directory of JSONL files + a DuckDB index. Each level is a file:

```
my-corpus/
  crucible.duckdb          # index — all levels queryable via SQL
  domains.jsonl            # L0: domain partitions
  sources.jsonl            # L1: papers, interviews, repos, markets
  propositions.jsonl       # L2: atomic falsifiable claims
  signposts.jsonl          # L3: convergence clusters
  cross_links.jsonl        # L2↔L2: supports/contradicts/extends
  projections.jsonl        # L4: falsifiable predictions
  paper_weakpoints.jsonl   # L1→L2: weakest hypothesis per source
  corpus_audits.jsonl      # meta: health check history
```

Each level references the one above by ID. The DuckDB file is a materialized index — you can always rebuild it from the JSONL files. The JSONL files are the source of truth (portable, diffable, git-friendly).

**N levels means**: you can add levels. A `surveys.jsonl` (L5) could reference projections. A `market_positions.jsonl` (L5) could track bets. The schema extends downward.

## Circuits

### Tier 1 — READ (feed-forward)
| Circuit | What it does |
|---------|-------------|
| `EXTRACT` | Source → atomic propositions (local model bulk, frontier verify) |
| `GROUND` | Proposition → Wikipedia canonical reference |
| `MATCH` | Proposition × proposition → relationship (supports/contradicts/extends) |
| `CONVERGE` | Set of matches → signal strength (convergence clustering) |
| `PROJECT` | High convergence → falsifiable prediction |
| `COMPOSE` | Selected propositions → new survey document |

### Tier 2 — LEARN (feedback loops)
| Circuit | What it does |
|---------|-------------|
| `DECAY` | Proposition × time → updated confidence (half-life, retraction cascade) |
| `CALIBRATE` | Prediction × outcome → weight update (did it come true?) |
| `SYNTHESIZE` | Propositions from multiple papers → novel proposition (new knowledge) |

### Tier 3 — ATTACK (adversarial)
| Circuit | What it does |
|---------|-------------|
| `CHALLENGE` | Proposition → search for negation (hunt disconfirming evidence) |
| `REPLICATE` | Proposition × new context → test result (does it hold here?) |
| `TRACE` | Proposition → evidence chain backward (provenance, citation ring detection) |

Plus: `EXPLODE` — find the weakest hypothesis in each paper (not a primitive, composes TRACE + tenacity rating)

## Agents

| Agent | Circuits | Mode |
|-------|----------|------|
| **Hunter** | EXTRACT → MATCH → CONVERGE | Continuous paper ingestion |
| **Challenger** | CHALLENGE → MATCH | Adversarial evidence search |
| **Verifier** | REPLICATE → CALIBRATE | Test claims against running code |
| **Composer** | CONVERGE → SYNTHESIZE → COMPOSE | Write new surveys |
| **Auditor** | TRACE + DECAY + health checks | Corpus immune system |
| **Forecaster** | PROJECT → CALIBRATE | Predict and track outcomes |
| **Exploder** | TRACE + tenacity rating | Find weakest hypothesis per paper |

## CLI

```bash
crucible extract <arxiv_url|jsonl_file> --domain <name>
crucible ground --limit 50
crucible converge --domain <name>
crucible explode [--source-id <id>] [--domain <name>]
crucible decay
crucible trace <prop_id>
crucible audit
```

## Schema

DuckDB-backed, domain-partitioned. Key tables:
- `domains` — partition key
- `sources` — papers with license, retraction, institution tracking
- `propositions` — atomic claims with tenacity, replication status, decay
- `signposts` — convergence clusters
- `cross_links` — supports/contradicts/extends between propositions
- `paper_weakpoints` — explode results (weakest hypothesis per paper)
- `corpus_audits` — health check history

## Model allocation

| Stage | Model | Why |
|-------|-------|-----|
| Bulk extraction | Local (nemotron 9B) | Free, JSON schema constraint |
| Verification | Frontier (opus/gemini) | Intellectual lineage, canonicalization |
| Tenacity rating | Frontier (opus) | Requires judgment about evidence quality |
| Cross-linking | Frontier (opus) | Understanding relationships |
| Clustering | DuckDB/schemaless | Statistical, no LLM needed |
| Survey writing | Frontier + human | Judgment + editorial direction |

## Key principle

**A paper doesn't fall when you explode it.** Every paper has a weakest claim. Finding it doesn't invalidate the paper — it tells you WHERE the paper is most likely wrong and what happens if that claim breaks. The strong propositions survive.
