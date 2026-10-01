---
name: dispatching-to-runtimes
description: Use when writing prompts for non-Claude agent dispatch — structures prompts so Gemini, GPT, or other runtimes execute effectively
---

# Dispatching to Runtimes

## Overview

When dispatching work to non-Claude runtimes (Gemini, GPT, local models, or any external agent), prompt structure determines success. Each runtime has different execution modes, failure patterns, and context handling.

**Announce at start:** "I'm using the dispatching-to-runtimes skill to structure the prompt for [runtime]."

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `target_runtime` | auto | auto, gemini, gpt, local, claude | Which runtime to structure the prompt for — auto detects from context |
| `prompt_style` | structured | structured, conversational, minimal | Structured=full template with headers, conversational=natural language, minimal=task+files only |
| `fallback_strategy` | escalate | escalate, retry, self_execute | What to do when dispatched agent fails — escalate to stronger model, retry with adjusted prompt, or self-execute |
| `verification` | required | required, optional, skip | Whether to include verification commands in the dispatch prompt |

## Universal Prompt Template

All runtimes benefit from this structure:

```markdown
# Task: [Clear task description]

## Context
[Brief background — what problem, why this approach]

## Files
- /absolute/path/to/file.ts
- /absolute/path/to/test.ts

## Steps
1. Read files from ## Files section
2. [Specific change with enough detail to act]
3. Run verification: [test command]

## Success Criteria
- [ ] [Measurable outcome]
- [ ] Tests pass
```

## Runtime-Specific Guidance

### Gemini

**Key rules**:
- `# Task:` header triggers Gemini's execution mode
- Absolute paths only — relative paths cause failures
- Explicit verification commands with timeout: `timeout 120 [cmd]`
- Success criteria as checkboxes

**Tool name mapping** (Gemini uses different names):
- `Read` → `read_file` | `Grep` → `search_file_content`
- `Edit` → `replace` | `Bash` → `run_shell_command`
- `WebSearch` → `google_web_search` | `WebFetch` → `web_fetch`
- `Agent` → spawn via `gemini --prompt` | MCP tools work identically

**Full mapping**: See `docs/gemini-compatibility.md`

**Known anti-patterns** (from empirical observation):
- Interactive commands (`python`, `node`, `bash` with no args) hang forever
- Relative paths (`./src/file.py`) produce file-not-found errors
- Unscoped searches (`grep -r .`, `find .`) cause context explosion
- Replace without reading first produces "0 occurrences"

### GPT / OpenAI

**Key rules**:
- Prefers conversational framing over directive headers
- Benefits from explicit role setting in system prompt
- Handles long context well but may lose detail in the middle
- Code blocks with language tags improve output quality

### Local Models (Small Language Models)

**Key rules**:
- Shorter prompts perform better — minimize context
- Structured output (JSON schema) dramatically improves reliability
- Break complex reasoning into atomic tasks
- Temperature 0.1-0.3 for deterministic tasks

### General Principles (Any Runtime)

1. **Absolute paths** — never relative
2. **Explicit verification** — tell the agent how to check its own work
3. **Scope the search space** — "check auth.ts lines 40-80" not "find the bug"
4. **Timeout uncertain commands** — `timeout 60 <cmd>`
5. **One task per dispatch** — multi-task prompts degrade quality

## When to Dispatch vs Self-Execute

| Situation | Action |
|-----------|--------|
| Task needs your full conversation context | Self-execute |
| Task is independent and well-defined | Dispatch to runtime |
| Task is CPU-bound or deterministic | Dispatch to local model |
| Task needs web search or browsing | Dispatch to capable runtime |
| Task needs deep reasoning | Self-execute or dispatch to strong model |

## Monitoring Dispatched Work

After dispatching:
1. Check for completion (don't assume success)
2. Verify output against success criteria
3. If failed, diagnose whether it's a prompt structure issue or a capability gap
4. Adjust prompt and retry, or escalate to a more capable runtime

## Skip Cost

If you dispatch without structuring the prompt, expect: wrong file paths, incomplete work, missing verification, and wasted compute. A 2-minute prompt structure pass saves 20 minutes of debugging bad output.

## Adaptive Work Sizing

Assess scope before committing to a work plan. If the task is larger than you can complete in your current context:

1. **Do what you can** — complete a meaningful chunk with clear boundaries
2. **Document what remains** — leave a concrete continuation prompt, not vague notes
3. **Spawn a follow-on agent** — or tell the caller to. The continuation agent should be able to pick up from your output without re-reading everything

Never assume you can finish everything. A completed chunk is more valuable than an incomplete sweep.
