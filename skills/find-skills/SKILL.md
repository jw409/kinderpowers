---
name: find-skills
description: Discover and install agent skills from the open ecosystem when users ask "how do I do X", "find a skill for X", or want to extend capabilities. Hub skill that connects kinderpowers to the broader skills.sh marketplace.
---

# Find Skills

Discover and install skills from the open agent skills ecosystem. This is the hub skill — it connects kinderpowers' built-in workflow skills to the broader world of community and vendor skills.

## When to Use

- User asks "how do I do X" where X might have an existing skill
- User says "find a skill for X" or "is there a skill for X"
- User wants to extend capabilities beyond what's installed
- User mentions a domain (design, testing, deployment) that might have dedicated skills

## Parameters (caller controls)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `search_scope` | all | kinderpowers, ecosystem, all | Where to search — kinderpowers built-ins first, ecosystem via skills CLI, or both |
| `auto_install` | false | true/false | Whether to install found skills automatically or just recommend |
| `source_filter` | any | any, verified, popular | Filter results — any source, verified publishers only, or 1K+ installs |

## The Skills CLI

The Skills CLI (`npx skills`) is the package manager for the open agent skills ecosystem.

**Key commands:**

- `npx skills find [query]` — Search for skills by keyword
- `npx skills add <package>` — Install a skill
- `npx skills check` — Check for updates
- `npx skills update` — Update all installed skills

**Browse at:** https://skills.sh/

## Process

### 1. Check kinderpowers first

Before searching externally, check if a kinderpowers skill already covers the need. Kinderpowers skills are tuned for agency-preserving workflows — they document tradeoffs and escape hatches that ecosystem skills may not.

| Domain | Kinderpowers skill |
|--------|-------------------|
| Starting work | brainstorming |
| Planning | writing-plans |
| Execution | executing-plans, subagent-driven-development |
| Testing | test-driven-development |
| Debugging | systematic-debugging |
| Code review | requesting-code-review, receiving-code-review |
| Branching | using-git-worktrees, finishing-a-development-branch |
| Parallel work | dispatching-parallel-agents |
| Verification | verification-before-completion |
| Architecture | architecture |
| Requirements | requirements |
| Retrospective | retrospective, adversarial-review |
| Issue tracking | beads |
| Creating skills | writing-skills |

### 2. Search the ecosystem

If kinderpowers doesn't cover it:

```bash
npx skills find [query]
```

Examples:
- "how do I make my React app faster?" → `npx skills find react performance`
- "can you help with PR reviews?" → `npx skills find pr review`
- "I need a changelog" → `npx skills find changelog`

### 3. Verify before recommending

**Recommended:** Check quality signals before suggesting a skill.

- **Install count** — Prefer 1K+ installs. Under 100 warrants caution.
- **Source reputation** — `vercel-labs`, `anthropics`, `microsoft` are well-known. Unknown authors deserve a repo check.
- **Skip cost:** Installing an unvetted skill is low-risk (skills are just markdown), but recommending a poor one wastes the user's time and erodes trust.

### 4. Present options

Include: skill name, what it does, install count/source, install command.

```
The "react-best-practices" skill provides React and Next.js performance
optimization guidelines from Vercel Engineering. (185K installs)

To install:
npx skills add vercel-labs/agent-skills@react-best-practices
```

### 5. Install

```bash
npx skills add <owner/repo@skill> -g -y
```

`-g` installs globally (user-level), `-y` skips confirmation.

## Common Categories

| Category | Example Queries |
|----------|----------------|
| Web Development | react, nextjs, typescript, css, tailwind |
| Testing | testing, jest, playwright, e2e |
| DevOps | deploy, docker, kubernetes, ci-cd |
| Documentation | docs, readme, changelog, api-docs |
| Code Quality | review, lint, refactor, best-practices |
| Design | ui, ux, design-system, accessibility |
| Productivity | workflow, automation, git |

## When Nothing Is Found

1. Acknowledge it — no skill exists for this
2. Offer to help directly with general capabilities
3. Mention the user could create their own: `npx skills init my-skill`

## Relationship to Kinderpowers

Kinderpowers ships workflow skills that compose into a development lifecycle. This skill is the bridge to everything else — when a user needs domain knowledge (React patterns, Kubernetes config, design systems) that workflow skills don't cover, find-skills connects them to the ecosystem.

Think of it as: kinderpowers handles *how you work*, ecosystem skills handle *what you work with*.
