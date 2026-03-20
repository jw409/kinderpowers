---
name: gsd-plan-checker
description: "Alias for gsd-verifier with mode=plan-quality. See agents/gsd-verifier.md for full documentation."
tools: Read, Write, Bash, Grep, Glob, Edit
color: green
alias_for: gsd-verifier
default_mode: plan-quality
---

# gsd-plan-checker (alias)

This agent has been consolidated into **gsd-verifier** with `mode=plan-quality`.

See: `agents/gsd-verifier.md`

When spawned as `gsd-plan-checker`, behavior is identical to `gsd-verifier` with `mode=plan-quality`.

All behavioral content, execution flows, and output formats are defined in the parameterized agent file.
