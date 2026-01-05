---
name: systematic-debugging
description: Use when encountering any bug, test failure, or unexpected behavior, before proposing fixes
---

# Systematic Debugging

## Overview

Random fixes waste time and create new bugs. Quick patches mask underlying issues.

**Core principle:** Find root cause before attempting fixes. Symptom fixes are rarely sufficient.

**The spirit matters more than the letter. Here's why:** This phased approach exists because debugging is fundamentally a scientific process. Skipping investigation means you're guessing, and guessing at complex systems has a poor success rate. The phases aren't bureaucracy—they're the structure that makes debugging tractable.

## The Iron Principle

```
INVESTIGATE BEFORE FIXING
```

Understanding why this matters:
- **Fixes without understanding often create new bugs** - You're changing code you don't fully understand
- **Symptom fixes mask the real problem** - The root cause will resurface, often worse
- **Investigation is faster than thrashing** - 20 minutes of analysis beats 3 hours of guess-and-check
- **Each fix attempt teaches something** - But only if you're learning, not just trying random changes

## When to Use

Use for ANY technical issue:
- Test failures
- Bugs in production
- Unexpected behavior
- Performance problems
- Build failures
- Integration issues

**Use this ESPECIALLY when:**
- Under time pressure (emergencies make guessing tempting)
- "Just one quick fix" seems obvious
- You've already tried multiple fixes
- Previous fix didn't work
- You don't fully understand the issue

**Tempting to skip when:**
- Issue seems simple (but simple bugs have root causes too)
- You're in a hurry (systematic is actually faster than thrashing)
- Pressure to fix NOW (one right fix beats five wrong ones)

## COSTS OF SKIPPING INVESTIGATION

Real examples of shotgun debugging that made things worse:

### The Config Cascade
**Situation:** Build failing with cryptic error message.
**Shotgun approach:** Changed three config values that "looked related."
**Result:** Build passed but broke production deployment. One of the changes disabled validation that caught bad data. Took 4 hours to untangle which change caused what.
**Root cause (found later):** Environment variable wasn't being passed to subprocess. 5-minute fix once understood.

### The Race Condition Whack-a-Mole
**Situation:** Intermittent test failure.
**Shotgun approach:** Added sleep(100ms) where the test was failing.
**Result:** Test passed locally, failed in CI. Added more sleeps. Eventually had 500ms of arbitrary waits, tests still flaky.
**Root cause (found later):** Missing await on async operation. Proper fix: condition-based waiting.

### The Dependency Downgrade
**Situation:** New library version breaking a feature.
**Shotgun approach:** Downgraded the library.
**Result:** Broke three other features that depended on new version. Spent 2 days on version compatibility matrix.
**Root cause (found later):** Library's breaking change was documented in CHANGELOG. Migration path was one line.

### The Multi-Fix Disaster
**Situation:** API returning wrong data.
**Shotgun approach:** Changed query, changed parser, changed serializer simultaneously.
**Result:** Different wrong data. Now impossible to tell which change did what.
**Root cause (found later):** Timezone handling in one specific code path. Would have been obvious with data flow tracing.

**Pattern:** In every case, investigation time would have been less than recovery time.

## The Four Phases

Each phase builds on the previous. Skipping risks fixing symptoms instead of causes, creating new bugs, and spending more total time than systematic investigation would have taken.

### Phase 1: Root Cause Investigation

**BEFORE attempting ANY fix:**

1. **Read Error Messages Carefully**
   - Don't skip past errors or warnings
   - They often contain the exact solution
   - Read stack traces completely
   - Note line numbers, file paths, error codes

2. **Reproduce Consistently**
   - Can you trigger it reliably?
   - What are the exact steps?
   - Does it happen every time?
   - If not reproducible, gather more data—guessing at intermittent bugs rarely works

3. **Check Recent Changes**
   - What changed that could cause this?
   - Git diff, recent commits
   - New dependencies, config changes
   - Environmental differences

4. **Gather Evidence in Multi-Component Systems**

   **When system has multiple components (CI -> build -> signing, API -> service -> database):**

   **Before proposing fixes, add diagnostic instrumentation:**
   ```
   For EACH component boundary:
     - Log what data enters component
     - Log what data exits component
     - Verify environment/config propagation
     - Check state at each layer

   Run once to gather evidence showing WHERE it breaks
   THEN analyze evidence to identify failing component
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

   **This reveals:** Which layer fails (secrets -> workflow OK, workflow -> build FAILED)

5. **Trace Data Flow**

   **When error is deep in call stack:**

   See `root-cause-tracing.md` in this directory for the complete backward tracing technique.

   **Quick version:**
   - Where does bad value originate?
   - What called this with bad value?
   - Keep tracing up until you find the source
   - Fix at source, not at symptom

### Phase 2: Pattern Analysis

**Find the pattern before fixing:**

1. **Find Working Examples**
   - Locate similar working code in same codebase
   - What works that's similar to what's broken?

2. **Compare Against References**
   - If implementing pattern, read reference implementation COMPLETELY
   - Don't skim—read every line
   - Understand the pattern fully before applying

3. **Identify Differences**
   - What's different between working and broken?
   - List every difference, however small
   - Don't assume "that can't matter"

4. **Understand Dependencies**
   - What other components does this need?
   - What settings, config, environment?
   - What assumptions does it make?

### Phase 3: Hypothesis and Testing

**Scientific method:**

1. **Form Single Hypothesis**
   - State clearly: "I think X is the root cause because Y"
   - Write it down
   - Be specific, not vague

2. **Test Minimally**
   - Make the SMALLEST possible change to test hypothesis
   - One variable at a time
   - Don't fix multiple things at once

3. **Verify Before Continuing**
   - Did it work? Yes -> Phase 4
   - Didn't work? Form NEW hypothesis
   - DON'T add more fixes on top

4. **When You Don't Know**
   - Say "I don't understand X"
   - Don't pretend to know
   - Ask for help
   - Research more

### Phase 4: Implementation

**Fix the root cause, not the symptom:**

1. **Create Failing Test Case**
   - Simplest possible reproduction
   - Automated test if possible
   - One-off test script if no framework
   - Having a test before fixing proves you understand the bug
   - Consider TDD approach for writing proper failing tests

2. **Implement Single Fix**
   - Address the root cause identified
   - ONE change at a time
   - No "while I'm here" improvements
   - No bundled refactoring

3. **Verify Fix**
   - Test passes now?
   - No other tests broken?
   - Issue actually resolved?

4. **If Fix Doesn't Work**
   - STOP
   - Count: How many fixes have you tried?
   - If < 3: Return to Phase 1, re-analyze with new information
   - **If >= 3: Time to question the architecture (see step 5)**
   - Another fix attempt without deeper analysis rarely helps

5. **3+ Fixes Failed: Question Architecture**

   **Pattern indicating architectural problem:**
   - Each fix reveals new shared state/coupling/problem in different place
   - Fixes require "massive refactoring" to implement
   - Each fix creates new symptoms elsewhere

   **Time to question fundamentals:**
   - Is this pattern fundamentally sound?
   - Are we "sticking with it through sheer inertia"?
   - Should we refactor architecture vs. continue fixing symptoms?

   **Discuss with your human partner before attempting more fixes**

   This isn't a failed hypothesis—this is evidence of a wrong architecture. The debugging process worked; it revealed something important.

## Signs You're Guessing Instead of Investigating

If you catch yourself thinking:
- "Quick fix for now, investigate later"
- "Just try changing X and see if it works"
- "Add multiple changes, run tests"
- "Skip the test, I'll manually verify"
- "It's probably X, let me fix that"
- "I don't fully understand but this might work"
- "Pattern says X but I'll adapt it differently"
- "Here are the main problems: [lists fixes without investigation]"
- Proposing solutions before tracing data flow
- **"One more fix attempt" (when already tried 2+)**
- **Each fix reveals new problem in different place**

These are signals to pause and return to Phase 1. Not because a rule says so, but because these patterns correlate strongly with wasted time.

## Human Partner's Signals Worth Heeding

**Watch for these redirections:**
- "Is that not happening?" - Assumption wasn't verified
- "Will it show us...?" - Evidence gathering was needed
- "Stop guessing" - Fixes proposed without understanding
- "Ultrathink this" - Question fundamentals, not just symptoms
- "We're stuck?" (frustrated) - Current approach isn't working

These are data points suggesting the investigation phase needs more attention.

## Common Rationalizations (and their track record)

| Rationalization | Historical Outcome |
|-----------------|-------------------|
| "Issue is simple, don't need process" | Simple issues have root causes too. Process is fast for simple bugs. |
| "Emergency, no time for process" | Systematic debugging is typically FASTER than guess-and-check thrashing. |
| "Just try this first, then investigate" | First fix sets the pattern. Starting sloppy tends to stay sloppy. |
| "I'll write test after confirming fix works" | Untested fixes have lower stick rate. Test first proves understanding. |
| "Multiple fixes at once saves time" | Can't isolate what worked. Often causes new bugs. |
| "Reference too long, I'll adapt the pattern" | Partial understanding has high bug correlation. Read it completely. |
| "I see the problem, let me fix it" | Seeing symptoms != understanding root cause. |
| "One more fix attempt" (after 2+ failures) | 3+ failures typically indicate architectural problems. |

## Quick Reference

| Phase | Key Activities | What You Learn |
|-------|---------------|----------------|
| **1. Root Cause** | Read errors, reproduce, check changes, gather evidence | WHAT and WHY |
| **2. Pattern** | Find working examples, compare | What's different |
| **3. Hypothesis** | Form theory, test minimally | Whether you're right |
| **4. Implementation** | Create test, fix, verify | Bug resolved, tests pass |

## When Investigation Reveals "No Root Cause"

If systematic investigation reveals issue is truly environmental, timing-dependent, or external:

1. You've completed the process
2. Document what you investigated
3. Implement appropriate handling (retry, timeout, error message)
4. Add monitoring/logging for future investigation

**Caveat:** Most "no root cause" cases are incomplete investigation. If you're concluding this, double-check.

## Supporting Techniques

These techniques are part of systematic debugging and available in this directory:

- **`root-cause-tracing.md`** - Trace bugs backward through call stack to find original trigger
- **`defense-in-depth.md`** - Add validation at multiple layers after finding root cause
- **`condition-based-waiting.md`** - Replace arbitrary timeouts with condition polling

## Why This Works

From debugging sessions:
- Systematic approach: 15-30 minutes to fix
- Random fixes approach: 2-3 hours of thrashing
- First-time fix rate: 95% vs 40%
- New bugs introduced: Near zero vs common

The numbers aren't rules—they're observations. The pattern holds because debugging is fundamentally about understanding before acting.
