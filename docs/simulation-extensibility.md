# Simulation Client Extensibility

Kinderpowers supports plugging in external simulation clients for user-archetype testing, behavioral modeling, and wargaming. This is an extensibility point — kinderpowers provides the interface, you bring the simulation engine.

## Architecture

```
kinderpowers skill/agent
    ↓ (structured scenario)
simulation client (external)
    ↓ (structured results)
kinderpowers analysis
```

Kinderpowers defines **scenarios** (structured descriptions of user interactions). A simulation client executes them and returns **results** (what happened, where the user got stuck, what succeeded).

## Scenario Format

A simulation scenario is a JSON object:

```json
{
  "archetype": {
    "name": "The Beginner",
    "description": "Barely codes, just got Claude Code, doesn't know git",
    "goals": ["Learn to use Claude Code", "Build a simple web app"],
    "pain_points": ["Unfamiliar with terminal", "Doesn't understand git"],
    "success_criteria": "Can describe what Claude helped them learn"
  },
  "product": "kinderpowers",
  "entry_point": "install and first use",
  "steps": [
    {"action": "install kinderpowers", "context": "fresh Claude Code install"},
    {"action": "try /gsd:new-project", "context": "first project"},
    {"action": "encounter error", "context": "missing git init"}
  ],
  "evaluation_dimensions": [
    "task_completion",
    "confusion_points",
    "error_recovery",
    "time_to_value"
  ]
}
```

## Result Format

The simulation client returns:

```json
{
  "archetype": "The Beginner",
  "overall_score": 0.65,
  "dimensions": {
    "task_completion": {"score": 0.7, "notes": "Completed 3/5 steps"},
    "confusion_points": {"score": 0.5, "notes": "Got stuck at git init, skill discovery"},
    "error_recovery": {"score": 0.8, "notes": "Error messages were helpful"},
    "time_to_value": {"score": 0.6, "notes": "20 min to first useful output"}
  },
  "failure_points": [
    {"step": 2, "description": "No guidance when git not initialized", "severity": "high"},
    {"step": 3, "description": "Error message didn't suggest fix", "severity": "medium"}
  ],
  "recommendations": [
    "Add git-init check to /gsd:new-project",
    "Improve error messages with actionable suggestions"
  ]
}
```

## Integration Points

### 1. Skill-level integration

Create a custom skill that calls your simulation client:

```markdown
# skills/wargame/SKILL.md
---
name: wargame
description: Run user archetype simulations against kinderpowers
---

## Usage
1. Define archetypes in `var/archetypes/`
2. Run simulation: `your-simulation-client --scenario scenario.json`
3. Analyze results with kinderpowers analysis tools
```

### 2. Agent-level integration

Create a custom agent that orchestrates simulation:

```markdown
# agents/simulation-runner.md
---
name: simulation-runner
model: opus
tools: Read, Write, Bash, Grep, Glob
---

You run user archetype simulations. Read scenario files, execute the simulation
client, and produce analysis reports.
```

### 3. MCP server integration

If your simulation client exposes an MCP server, kinderpowers agents can call it directly via MCP tools.

## Standard Archetypes

Kinderpowers suggests 5 standard archetypes for product testing:

1. **The Beginner** — barely codes, learning. Success = "Claude is helping me learn"
2. **The Senior Dev** — skeptical of frameworks. Success = "This saves me time without overhead"
3. **The Team Lead** — needs to scale across a team. Success = "My team ships faster"
4. **The Open Source Maintainer** — needs to triage and review. Success = "I process PRs in half the time"
5. **The AI-Native Builder** — builds on top of AI tools. Success = "This gives me capabilities I couldn't build alone"

Archetype definitions live in `var/archetypes/` as JSON files following the scenario format above.

## Example: Connecting romancer4

[romancer4](https://github.com/jw409/romancer4) provides multi-agent behavioral simulation that can serve as a simulation client:

```bash
# Generate scenario
cat var/archetypes/beginner.json | romancer4 simulate --product kinderpowers

# Batch all archetypes
for f in var/archetypes/*.json; do
  romancer4 simulate --product kinderpowers --scenario "$f" --output "var/simulation-results/$(basename $f)"
done
```

The integration is through structured JSON — any simulation engine that reads the scenario format and writes the result format works.
