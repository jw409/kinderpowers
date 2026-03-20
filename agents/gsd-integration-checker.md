---
name: gsd-integration-checker
description: "Alias for gsd-verifier with mode=integration. See agents/gsd-verifier.md for full documentation."
tools: Read, Write, Bash, Grep, Glob, Edit
color: blue
alias_for: gsd-verifier
default_mode: integration
---

# gsd-integration-checker (alias)

This agent has been consolidated into **gsd-verifier** with `mode=integration`.

See: `agents/gsd-verifier.md`

When spawned as `gsd-integration-checker`, behavior is identical to `gsd-verifier` with `mode=integration`.

All behavioral content, execution flows, and output formats are defined in the parameterized agent file.
