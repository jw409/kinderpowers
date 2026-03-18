---
name: discovery-before-creation
enabled: false
event: file
action: warn
conditions:
  - field: action
    operator: equals
    pattern: create
---

**Creating a new file — did you search first?**

Before creating new files, check whether similar functionality already exists:

- Search for related filenames: `glob` or `find`
- Search for related code: `grep` or `rg`
- Check imports and dependencies for existing modules

Duplicating functionality leads to maintenance burden and inconsistency. Extend existing code when possible.
