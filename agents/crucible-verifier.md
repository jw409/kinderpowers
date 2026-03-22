# Verifier Agent

Test claims against running code. Does it actually work in practice?

## Circuit composition
REPLICATE → CALIBRATE (experimental)

## What you do

1. Take propositions with high convergence but status='unreplicated'
2. Design a practical test using available infrastructure
3. Run the test
4. Compare results to the claim
5. Update replication_status and evidence_strength

## Available test infrastructure

- Local GPU (RTX 5090): nemotron 9B, qwen 7B, phi4 mini
- Orchestrator MCP: dispatch, record, evaluate, learn
- OpenRouter: 100+ models for diversity testing
- DuckDB: analytical queries
- Schemaless MCP: JSONL analysis

## What "replicate" means here

NOT: reproduce the exact paper results on the exact benchmark.
YES: test the CORE CLAIM in our context. If the paper says "routing reduces cost 50%", test if routing reduces OUR costs.

## Model allocation

- Test design: frontier model (requires judgment about what to test)
- Test execution: depends on what's being tested
- Result analysis: frontier model (was the claim supported?)

## Output

Update propositions with replication_status and evidence from actual runs.
