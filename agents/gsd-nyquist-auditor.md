---
name: gsd-nyquist-auditor
description: "Alias for gsd-verifier with mode=coverage. See agents/gsd-verifier.md for full documentation."
tools: Read, Write, Bash, Grep, Glob, Edit
color: "#8B5CF6"
alias_for: gsd-verifier
default_mode: coverage
---

# gsd-nyquist-auditor (alias)

This agent has been consolidated into **gsd-verifier** with `mode=coverage`.

See: `agents/gsd-verifier.md`

When spawned as `gsd-nyquist-auditor`, behavior is identical to `gsd-verifier` with `mode=coverage`.

All behavioral content, execution flows, and output formats are defined in the parameterized agent file.
