---
name: brainstorm-before-build
enabled: false
event: write
action: warn
conditions:
  - field: new_content_lines
    operator: greater_than
    value: 100
  - field: transcript
    operator: not_contains
    pattern: brainstorm|design|approach|trade-?off|alternative|option
---

**Large code block without design discussion detected.**

You're about to write 100+ lines without evidence of brainstorming or design discussion in this session. Consider:

- Have you explored 2-3 approaches and their trade-offs?
- Has the user approved the design direction?
- Is this the simplest approach, or are you over-engineering?

This rule warns (doesn't block). If you've already discussed the approach verbally, proceed. If this is a known pattern or template, proceed. The goal is to catch unexamined assumptions, not slow down clear work.
