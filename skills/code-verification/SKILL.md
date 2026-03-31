---
name: code-verification
description: "Verify beads, commits, or codebases against actual code using ZMCPTools AST resources — repeatable evidence-based extraction"
---

# Code Verification

## Overview

Use ZMCPTools MCP resources to verify claims against actual code. Works at three levels: verify a single bead/issue, verify a commit, or audit an entire codebase. Every verification produces evidence, not opinions.

**Announce at start:** "I'm using the code-verification skill to verify [target]."

## Prerequisites

ZMCPTools must be running as an MCP server. All resources are `ReadMcpResourceTool(server="zmcptools", uri="...")` calls.

Check index health first:

```
ReadMcpResourceTool(server="zmcptools", uri="symbols://stats")
```

If stale or empty, reindex before verifying.

## Three Verification Modes

### Mode 1: Verify a Bead

**Input:** A bead ID (e.g., `game1-x8jw`)
**Question:** "Did the work described in this bead actually get done?"

**Steps:**

1. **Read the bead** — get title, description, acceptance criteria
2. **Extract claims** — what files, functions, behaviors does it claim to have created/modified?
3. **Check each claim against code:**

```
# Does the file exist?
ReadMcpResourceTool(server="zmcptools", uri="file://path/to/claimed/file.ts/symbols?depth=1")

# Does the claimed function exist?
ReadMcpResourceTool(server="zmcptools", uri="symbols://search?name=claimedFunction&type=function")

# Is it wired to anything? (not dead code)
ReadMcpResourceTool(server="zmcptools", uri="graph://symbol/path/to/file.ts::claimedFunction/callers")

# Is it exported? (public API claim)
ReadMcpResourceTool(server="zmcptools", uri="file://path/to/file.ts/exports")
```

4. **Check git for the implementation commit:**

```bash
git log --all --oneline --grep="bead-id" -- path/to/file.ts
git log --all --oneline --since="2026-03-01" --until="2026-03-15" -- path/to/file.ts
```

5. **Verdict:**
   - **VERIFIED** — code exists, is wired, matches description
   - **PARTIAL** — some claims verified, others missing (list gaps)
   - **UNVERIFIED** — no code evidence found
   - **STALE** — code existed but was since removed/refactored

### Mode 2: Verify a Commit

**Input:** A commit SHA or range
**Question:** "What did this commit actually do, and is it still live?"

**Steps:**

1. **Read the commit:**

```bash
git show --stat <sha>
git diff <sha>~1 <sha> --name-only
```

2. **For each modified file, check current state:**

```
# What does the file look like now?
ReadMcpResourceTool(server="zmcptools", uri="file://path/to/modified/file.ts/symbols?depth=1")

# Are the added functions still present?
ReadMcpResourceTool(server="zmcptools", uri="symbols://search?name=addedFunction")

# Are they connected to anything?
ReadMcpResourceTool(server="zmcptools", uri="graph://symbol/path::addedFunction/context")
```

3. **Check for subsequent changes:**

```bash
git log <sha>..HEAD -- path/to/file.ts
```

4. **Verdict:** Same as bead mode — VERIFIED/PARTIAL/UNVERIFIED/STALE

### Mode 3: Audit a Codebase (or subdirectory)

**Input:** A directory path
**Question:** "What's the real state of this code? What's dead, what's load-bearing, what's fragile?"

**Steps:**

1. **Project-level discovery:**

```
# Entry points, hubs, hot zones
ReadMcpResourceTool(server="zmcptools", uri="project://./discovery?top=20")

# Dead code
ReadMcpResourceTool(server="zmcptools", uri="project://./deadcode?confidence=HIGH")

# Circular deps
ReadMcpResourceTool(server="zmcptools", uri="project://./circular-deps")

# Call graph health
ReadMcpResourceTool(server="zmcptools", uri="graph://stats")

# Hotspots (most-called, highest fan-out)
ReadMcpResourceTool(server="zmcptools", uri="graph://hotspots")
```

2. **For hub files (top 5 by import count):**

```
# Full symbol inventory
ReadMcpResourceTool(server="zmcptools", uri="file://hub/file.ts/symbols?depth=1")

# What depends on this?
ReadMcpResourceTool(server="zmcptools", uri="project://hub/file.ts/dependents")

# Blast radius
ReadMcpResourceTool(server="zmcptools", uri="project://hub/file.ts/impact-analysis")
```

3. **Output:** Structured audit with:
   - Hub files (load-bearing, high blast radius)
   - Dead code (safe to remove)
   - Circular dependencies (architectural debt)
   - Hot zones (recently active, high churn)
   - Entry points (where execution starts)

## Using Verification in Bulk (The Swizzle)

When verifying many beads (e.g., auditing all 1000+ closed beads):

1. **Export bead list** with titles and descriptions
2. **Extract claimed artifacts** from each bead (files, functions, behaviors)
3. **Batch symbol searches** — search for all claimed functions at once:

```
ReadMcpResourceTool(server="zmcptools", uri="symbols://search?name=functionName&type=function")
```

4. **Cross-reference:** For each bead, count verified vs unverified claims
5. **Flag beads** where >50% of claims are unverified — these are candidates for reopening
6. **For flagged beads**, do deep verification (Mode 1) to confirm

This avoids doing full Mode 1 on every bead — the symbol search acts as a fast filter.

## Evidence Standards

Every verification claim must cite its source:

| Claim | Evidence Required |
|-------|------------------|
| "Function exists" | `symbols://search` result showing it |
| "File exists" | `file://path/symbols` returns data (not error) |
| "Code is wired" | `graph://symbol/path::name/callers` shows >0 callers |
| "Code is dead" | `project://./deadcode` lists it, AND `graph://` shows 0 callers |
| "Commit landed" | `git log` shows SHA on current branch |
| "Work is complete" | ALL acceptance criteria have evidence, not just some |

**Never say "verified" without citing which resource confirmed it.**

## Integration with Beads

After verification, update beads directly:

```bash
# Reopen a prematurely closed bead
bd reopen <id>
bd update <id> --notes="Verification found: [specific gaps]"

# Close with evidence
bd close <id> --reason="Verified: function X exists (symbols://search), wired (graph://callers), tests pass"
```

## Integration with Research Extraction

This skill complements `research-extraction`. Use research-extraction to harvest ideas from a codebase. Use code-verification to check if those ideas were actually implemented, or if claimed implementations are real.

Pipeline: `research-extraction` (what could we use?) → `code-verification` (did we actually build it?)

## Quick Reference: Common URIs

```
# Symbol discovery (cheap, fast)
file://path/symbols?depth=1          # ~500 tokens
file://path/exports                   # ~40 tokens
symbols://search?name=X&type=Y       # ~100 tokens

# Dependency tracing (medium)
file://path/imports                   # ~200 tokens
project://path/dependencies           # from cache
project://path/dependents             # reverse deps

# Call graph (rich, targeted)
graph://symbol/path::name/callers     # who calls this
graph://symbol/path::name/callees     # what this calls
graph://symbol/path::name/context     # 360° view
graph://hotspots                      # most-called

# Project health (broad)
project://./discovery?top=10          # entry points, hubs
project://./deadcode?confidence=HIGH  # safe to remove
project://./circular-deps             # architectural debt
graph://stats                         # edge counts
symbols://stats                       # index health
```
