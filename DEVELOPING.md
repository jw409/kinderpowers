# Developing Kinderpowers Agents

## Agent Architecture

Agents are static spec files: YAML frontmatter + markdown body. Claude Code reads them at spawn time and uses the spec as the agent's system prompt.

Three flavors exist across the ecosystem:

| System | Format | Location |
|--------|--------|----------|
| TalentOS | `.agent.yaml` | `talent-os/agents/` |
| Claude Code | `.md` in `~/.claude/agents/` | User-local |
| GSD (kinderpowers) | `.md` in plugin `agents/` | Plugin-managed |

Kinderpowers agents are the GSD flavor — registered under the `kinderpowers:` namespace, spawned via `subagent_type: "kinderpowers:gsd-executor"` or just `"gsd-executor"`.

## Agent Outcome Logging

Every Agent tool invocation is logged by a PostToolUse hook to:

```
~/.kinderpowers/agent_outcomes.jsonl
```

### Schema

```json
{
  "ts": "2026-03-18T14:30:00+00:00",
  "agent": "gsd-planner",
  "model": "opus",
  "description": "Create phase plan",
  "session_id": "abc-123",
  "bead_id": "game1-x2k7",
  "project": "/home/user/dev/myproject",
  "output_chars": 4200,
  "output_preview": "first 500 chars of output...",
  "variant": "default"
}
```

| Field | Source |
|-------|--------|
| `agent` | `tool_input.subagent_type` or `tool_input.name` |
| `model` | `tool_input.model` (null if not overridden) |
| `description` | `tool_input.description` |
| `session_id` | `CLAUDE_SESSION_ID` env var |
| `bead_id` | `CLAUDE_BEAD_ID` env var |
| `project` | `CLAUDE_PROJECT_DIR` env var or cwd |
| `output_chars` | Length of full result text |
| `output_preview` | First 500 chars of result |
| `variant` | Extracted from prompt `variant:` marker, or `"default"` |

## Running Experiments

### 1. Fork an agent

Copy the agent spec and give it a new name:

```bash
cp agents/gsd-executor.md agents/gsd-executor-slim.md
```

Edit the frontmatter in the copy:

```yaml
---
name: gsd-executor-slim
description: "Slimmed executor variant — fewer protocols, faster completion"
# ... rest of frontmatter
---
```

### 2. Spawn the variant

In your prompt or agent invocation, use the new name:

```
subagent_type: "gsd-executor-slim"
```

### 3. Tag the variant (optional)

If you want explicit variant tagging beyond agent name, add a marker in the prompt:

```
variant: slim

Execute the phase plan at .planning/phase-3/PLAN.md
```

The hook extracts the `variant:` value from the first matching line.

### 4. Run both variants

Run the same task with both agents across multiple sessions. Aim for at least 5 runs per variant to get meaningful signal.

### 5. Compare with DuckDB

```sql
-- Basic comparison: output size and run count by variant
SELECT agent, variant,
       count(*) AS runs,
       avg(output_chars) AS avg_output,
       min(output_chars) AS min_output,
       max(output_chars) AS max_output
FROM read_json_auto('~/.kinderpowers/agent_outcomes.jsonl')
GROUP BY agent, variant
ORDER BY agent, variant;
```

```sql
-- Per-project breakdown
SELECT agent, variant, project,
       count(*) AS runs,
       avg(output_chars) AS avg_output
FROM read_json_auto('~/.kinderpowers/agent_outcomes.jsonl')
GROUP BY agent, variant, project
ORDER BY agent, variant;
```

```sql
-- Recent activity timeline
SELECT date_trunc('day', ts::timestamp) AS day,
       agent, variant, count(*) AS runs
FROM read_json_auto('~/.kinderpowers/agent_outcomes.jsonl')
GROUP BY 1, 2, 3
ORDER BY 1 DESC, 2;
```

```sql
-- Head-to-head: compare two specific variants
SELECT variant,
       count(*) AS runs,
       avg(output_chars) AS avg_output,
       percentile_cont(0.5) WITHIN GROUP (ORDER BY output_chars) AS median_output
FROM read_json_auto('~/.kinderpowers/agent_outcomes.jsonl')
WHERE agent = 'gsd-executor'
  AND variant IN ('default', 'slim')
GROUP BY variant;
```

## Comparing Variants

Output size is the primary signal available from logging alone. Smaller output from an agent that still completes its task successfully suggests a more efficient spec. Combine with:

- **Manual review**: Read `output_preview` for quality checks
- **Bead correlation**: Join on `bead_id` to see which variants led to closed vs stuck issues
- **Session patterns**: Group by `session_id` to see multi-agent interaction effects

## Contributing Improvements

Open a GitHub issue or PR on kinderpowers with:

1. **Which agent** you modified (e.g., `gsd-executor`)
2. **What you changed** and why (diff or summary)
3. **Comparison data** from at least 5 runs per variant (paste the DuckDB query output)
4. **The variant `.md` file** so maintainers can review the full spec

## Agent Spec Anatomy

### Frontmatter

```yaml
---
name: gsd-executor          # Unique identifier, used in subagent_type
description: "One-line summary shown in agent picker"
model: opus                  # Default model (sonnet, opus, haiku)
tools:                       # Tool allowlist
  - Read
  - Write
  - Edit
  - Bash
  - Grep
  - Glob
color: "#4CAF50"             # Optional UI color
---
```

### Common Body Sections

| Section | Purpose |
|---------|---------|
| Role | Who the agent is, what it optimizes for |
| Parameters | Input expectations (file paths, config) |
| Protocols | Step-by-step procedures the agent follows |
| Success criteria | How the agent knows it's done |
| Constraints | What the agent must NOT do |
| Output format | Expected structure of results |
