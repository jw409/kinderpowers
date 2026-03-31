---
name: research-extraction
description: "Deep extraction and gap analysis from open-source projects, papers, or codebases — harvest, extract, analyze, rank, verify pipeline for grabbing good ideas"
---

# Research Extraction

## Overview

Point at a codebase, repo, paper collection, or reference implementation. Extract the good ideas. Rank them against your project goals. Verify feasibility. Create actionable work items.

**Announce at start:** "I'm using the research-extraction skill to analyze [target]."

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `mode` | idea_extraction | idea_extraction, usage_evaluation, deep_integration | Pipeline depth — full 6-phase, phases 1-2 only, or 1-3 with integration mapping |
| `extraction_depth` | L2 | L0, L1, L2, L3 | Maximum extraction level — L0=structure, L1=positioning, L2=capabilities, L3=killer insights |
| `relevance_filter` | moderate | none, moderate, strict | How aggressively to filter findings against project goals |
| `output_format` | ranked_list | ranked_list, action_items, comparison_table | How to present results |
| `max_source_files` | 20 | 5-50 | Maximum source files to analyze per target (controls cost) |

## Core Pipeline

```
harvest → extract (L0-L3) → analyze → rank → verify → action items
```

Papers have abstracts. Projects have READMEs and source code. The extraction levels differ; the pipeline is the same.

## Routing: Three Modes

Before starting, identify which mode fits your goal:

| Mode | Use When | Depth | Output |
|------|----------|-------|--------|
| **Idea Extraction** | "What can I learn from this?" | Full pipeline (all 6 phases) | Ranked ideas with implementation sketches |
| **Usage Evaluation** | "Should I use this library/tool?" | Phases 1-2 only (harvest + extract) | Capabilities inventory, limitations, verdict |
| **Deep Integration** | "How do I integrate this into my project?" | Phases 1-3 + custom mapping | Integration plan with dependency analysis |

## Phase 1: Harvest (Single Agent)

Collect structured source material from the target:

1. **README/docs** — overview, positioning, value proposition
2. **Architecture docs** — design decisions, ADRs, system diagrams
3. **Key source files** — entry points, core modules (read package manifests for structure)
4. **Config/schema files** — understand the data model
5. **Test structure** — what do they test? Reveals real capabilities vs marketing
6. **CHANGELOG/releases** — trajectory signal

**Size limit**: Skip files >500 lines. For large files, extract first 100 lines + doc comments + exported symbols.

## Phase 2: Extract (Parallel Agents)

Cyclonic extraction levels:

| Level | What It Captures |
|-------|-----------------|
| L0 | Project structure, dependencies, tech stack |
| L1 | One-line value prop + positioning (50 words) |
| L2 | Capabilities inventory, architecture patterns, design decisions |
| L3 | UX innovations, ecosystem features, killer insights, limitations |

**For usage evaluation mode**: Stop here. Present L0-L3 summary with verdict.

## Phase 3: Analyze (Parallel Agents)

Score each capability/pattern against your project's goals or vision document.

For each domain in your project:
- Score alignment (0-1)
- Identify current gaps the idea would fill
- Sketch implementation approach
- Estimate effort and impact

## Phase 4: Rank (Single Agent)

Composite score: `(alignment × 0.4) + (impact × 0.4) + (inverse_effort × 0.2)`

Group into technique clusters — multiple sources proposing similar ideas increases confidence.

## Phase 5: Verify (Single Agent)

For top ideas:
1. **Feasibility check** — does something similar already exist in your project?
2. **Prior art check** — was this attempted before? Why did it succeed/fail?
3. **Dependency check** — what would need to change first?
4. **Contradiction check** — do any top ideas conflict with each other?

## Phase 6: Action Items (With Approval)

Present top ideas. For each approved one, create an actionable work item with title, implementation sketch, source evidence, alignment score, and priority.

## Execution Constraints (Strongly Recommended)

- **Scope per agent**: Keep focused — one extraction target per agent avoids drift
- **Discovery before creation**: Check if extraction already exists before re-running
- **Use strong models for extraction/analysis**: Reasoning required — weak models miss nuance
- **Non-overlapping file domains**: Each extractor writes to its own target directory

**Skip cost**: Running without constraints produces unfocused extractions that bury the good ideas in noise. A 5-minute setup pass (check for prior extractions, scope the target) saves an hour of re-reading mediocre output.

## Reusability

This skill is generic. It works for competitive analysis, technology scouting, pattern mining, upgrade planning, and paper review. Swap the vision document for your project's equivalent and adjust the analysis domains.

## Quick Start

```
# Single target
"Analyze references/some-project using research-extraction"

# Usage evaluation (quick mode)
"Evaluate whether we should use library-x — use research-extraction in usage mode"

# Deep integration
"How would we integrate project-y into our codebase? Use research-extraction"
```
