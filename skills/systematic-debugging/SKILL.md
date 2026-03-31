---
name: systematic-debugging
description: Use when encountering any bug, test failure, or unexpected behavior, before proposing fixes
---

# Systematic Debugging

## Overview

Find root cause before attempting fixes. Symptom fixes waste time and can create new problems.

**Core principle:** Understand the problem before changing the code.

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `depth` | standard | quick, standard, exhaustive | Investigation thoroughness. quick=Phase 1+3 only (skip pattern analysis), standard=all 4 phases, exhaustive=all phases with mandatory multi-component evidence gathering |
| `hypothesis_count` | 3 | 2-8 | Number of hypotheses to generate before investigating any. Higher=more thorough, lower=faster |
| `reproduce_first` | true | true, false | Whether to require consistent reproduction before investigating. false=proceed on reported symptoms alone |

**Parsing hints:** Parse from caller prompt. "Quick debug" -> depth=quick. "Be thorough" -> depth=exhaustive. "I can describe it but can't reproduce" -> reproduce_first=false. "Generate lots of theories" -> hypothesis_count=6.

## When to Use

Use for ANY technical issue:
- Test failures
- Bugs in production
- Unexpected behavior
- Performance problems
- Build failures
- Integration issues

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `hypothesis_limit` | 3 | 1-5 | Maximum fix attempts before questioning architecture and escalating to human |
| `auto_fix` | false | true/false | Whether to implement the fix after confirming root cause, or just report findings |
| `scope` | local | local, module, system | Investigation breadth — local=single file/function, module=containing module, system=cross-cutting trace |
| `evidence_level` | standard | minimal, standard, thorough | How much evidence to gather — minimal=reproduce+check changes, thorough=full data flow trace across component boundaries |

**Especially valuable when:**
- Under time pressure (guessing wastes more time than investigating)
- "Just one quick fix" seems obvious
- You've already tried multiple fixes
- You don't fully understand the issue

## The Four Phases

Complete each phase before proceeding to the next, unless you have a clear reason to skip ahead.

### Phase 1: Root Cause Investigation

**BEFORE attempting ANY fix:**

1. **Read Error Messages Carefully**
   - Don't skip past errors or warnings
   - Read stack traces completely
   - Note line numbers, file paths, error codes

2. **Reproduce Consistently**
   - Can you trigger it reliably?
   - What are the exact steps?
   - If not reproducible → gather more data, don't guess
   - When reproduce_first=false, proceed to Step 3 using reported symptoms. Note: this reduces confidence in findings.

3. **Check Recent Changes**
   - Git diff, recent commits
   - New dependencies, config changes
   - Environmental differences

4. **Gather Evidence in Multi-Component Systems**

   When depth=exhaustive, ALWAYS perform multi-component boundary logging regardless of system complexity.

   ```
   For EACH component boundary:
     - Log what data enters component
     - Log what data exits component
     - Verify environment/config propagation

   Run once to gather evidence showing WHERE it breaks
   THEN investigate that specific component
   ```

   **Example (multi-layer system):**
   ```bash
   # Layer 1: Workflow
   echo "=== Secrets available in workflow: ==="
   echo "IDENTITY: ${IDENTITY:+SET}${IDENTITY:-UNSET}"

   # Layer 2: Build script
   echo "=== Env vars in build script: ==="
   env | grep IDENTITY || echo "IDENTITY not in environment"

   # Layer 3: Signing script
   echo "=== Keychain state: ==="
   security list-keychains
   security find-identity -v

   # Layer 4: Actual signing
   codesign --sign "$IDENTITY" --verbose=4 "$APP"
   ```

   **This reveals:** Which layer fails (secrets → workflow ✓, workflow → build ✗)

5. **Trace Data Flow**

   See `root-cause-tracing.md` in this directory for the complete backward tracing technique.

   **Quick version:**
   - Where does bad value originate?
   - What called this with bad value?
   - Keep tracing up until you find the source
   - Fix at source, not at symptom

### Phase 2: Pattern Analysis

When depth=quick, skip this phase entirely -- go from Phase 1 directly to Phase 3.

1. **Find Working Examples** — locate similar working code in the codebase
2. **Compare Against References** — read reference implementations completely, don't skim
3. **Identify Differences** — list every difference, however small
4. **Understand Dependencies** — what components, settings, assumptions does this need?

### Phase 3: Hypothesis and Testing

1. **Generate Hypotheses** — Generate {hypothesis_count} hypotheses before investigating any. Default 3 -- increase for complex systems, decrease for obvious bugs. Form each as: "I think X is the root cause because Y"
2. **Test Minimally** — smallest possible change, one variable at a time
3. **Verify Before Continuing** — worked? Phase 4. Didn't? Form new hypothesis. Don't stack fixes.
4. **When You Don't Know** — say so. Ask for help. Research more.

### Phase 4: Implementation

1. **Create Failing Test Case** — simplest reproduction, automated if possible
2. **Implement Single Fix** — one change, no "while I'm here" improvements
3. **Verify Fix** — test passes? No other tests broken? Issue resolved?
4. **If Fix Doesn't Work**
   - Count: How many fixes have you tried?
   - If < {hypothesis_count}: Return to Phase 1 with new information
   - **If >= {hypothesis_count}: Question the architecture** — each fix revealing new problems in different places suggests a structural issue, not a local bug
   - Discuss with your human partner before attempting more fixes

## Signals to Return to Phase 1

If you catch yourself thinking:
- "Quick fix for now, investigate later"
- "Just try changing X and see if it works"
- "I don't fully understand but this might work"
- Proposing solutions before tracing data flow
- "One more fix attempt" (when already tried 2+)

These are signals to slow down and gather more evidence.

## Your Human Partner's Redirections

When your partner says:
- "Is that not happening?" — you assumed without verifying
- "Will it show us...?" — add evidence gathering
- "Stop guessing" — return to Phase 1
- "Ultrathink this" — question fundamentals, not just symptoms

## Quick Reference

| Phase | Key Activities | Success Criteria | Parameter Effects |
|-------|---------------|------------------|-------------------|
| **1. Root Cause** | Read errors, reproduce, check changes, gather evidence | Understand WHAT and WHY | reproduce_first=false: skip Step 2; depth=exhaustive: mandatory boundary logging |
| **2. Pattern** | Find working examples, compare | Identify differences | depth=quick: skip entirely |
| **3. Hypothesis** | Form {hypothesis_count} theories, test minimally | Confirmed or new hypothesis | hypothesis_count: controls how many hypotheses before investigation |
| **4. Implementation** | Create test, fix, verify | Bug resolved, tests pass | hypothesis_count: threshold for "question the architecture" |

## When Investigation Reveals "No Root Cause"

If investigation shows the issue is truly environmental, timing-dependent, or external:

1. Document what you investigated
2. Implement appropriate handling (retry, timeout, error message)
3. Add monitoring/logging for future investigation

## Supporting Techniques

- **`root-cause-tracing.md`** — Trace bugs backward through call stack
- **`defense-in-depth.md`** — Add validation at multiple layers
- **`condition-based-waiting.md`** — Replace arbitrary timeouts with condition polling

**Related skills:**
- **kinderpowers:test-driven-development** — For creating failing test case
- **kinderpowers:verification-before-completion** — Verify fix before claiming success
