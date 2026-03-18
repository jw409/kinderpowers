---
name: verification-required
enabled: false
event: stop
action: block
conditions:
  - field: transcript
    operator: not_contains
    pattern: test|verify|assert|check|confirm|screenshot|curl.*200|expect\(
---

**Verification evidence not found in transcript.**

Before marking work as complete, provide evidence that it works:

- Run tests (`pytest`, `npm test`, `cargo test`, etc.)
- Show command output confirming expected behavior
- Include a screenshot or curl response for UI/API changes
- Reference specific assertion results

This rule blocks stopping without verification. Disable it if you're doing exploratory or documentation-only work.
