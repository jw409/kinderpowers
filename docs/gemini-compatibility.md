# Gemini CLI Compatibility Guide

Kinderpowers agents and skills are designed for Claude Code but can work with Gemini CLI with adaptations.

## Tool Name Mapping

Kinderpowers agents declare tools using Claude Code names. Gemini CLI uses different names.

| Claude Code (kinderpowers) | Gemini CLI | Notes |
|---------------------------|------------|-------|
| `Read` | `read_file` | Same semantics |
| `Write` | `write_file` | Same semantics |
| `Edit` | `edit_file` | Gemini uses search/replace |
| `Bash` | `run_shell_command` | Same semantics |
| `Grep` | `run_shell_command` + grep | No dedicated grep tool |
| `Glob` | `run_shell_command` + find | No dedicated glob tool |
| `WebSearch` | `google_search` | Different parameter names |
| `WebFetch` | `run_shell_command` + curl | No dedicated fetch tool |
| `LSP` | Not available | Use shell-based LSP clients |
| `Agent` | `run_shell_command` + gemini | Spawn via CLI |
| `AskUserQuestion` | Not available | Use inline prompts |
| `TodoWrite` | Not available | Use shell-based tracking |

## Agent Definition Adaptation

Kinderpowers agent `.md` files use frontmatter:

```yaml
---
name: code-reviewer
model: opus
tools: Read, Grep, Glob, Bash
---
```

For Gemini, translate the `tools:` line mentally — the agent prompt still works because Gemini understands the *intent* (read a file, search code, run commands) even if the tool names differ. The key adaptations:

### 1. Tool References in Prompts

Agent prompts that say "Use the Read tool to..." should be interpreted as "Use read_file to..." in Gemini context. This happens naturally — Gemini understands the intent.

### 2. Grep/Glob Patterns

Claude Code has dedicated `Grep` and `Glob` tools. In Gemini, use `run_shell_command`:

```bash
# Instead of Grep tool:
rg "pattern" --type ts

# Instead of Glob tool:
find . -name "*.ts" -path "*/components/*"
```

### 3. Subagent Spawning

Claude Code uses the `Agent` tool to spawn subagents. In Gemini:

```bash
# Spawn a Gemini subagent
gemini --prompt "$(cat <<'EOF'
# Task: [task from agent definition]
...
EOF
)"
```

### 4. MCP Server Compatibility

Kinderpowers MCP servers (kp-github, kp-sequential-thinking) work with any MCP-compatible client, including Gemini CLI. No adaptation needed — MCP is runtime-agnostic.

## Skill Compatibility

Skills are `SKILL.md` files with instructions. They're runtime-agnostic by design — the instructions describe *what to do*, not *which tools to call*. Gemini can follow skill instructions directly.

**High compatibility** (work as-is):
- metathinking, brainstorming, strategic-planning, requirements
- retrospective, adversarial-review, architecture
- All research and analysis skills

**Needs adaptation** (reference Claude-specific tools):
- verification-before-completion (references Read, Bash by name)
- test-driven-development (references Edit, Bash by name)
- executing-plans (references Edit, Write by name)

**Claude-only** (depend on Claude Code infrastructure):
- using-kinderpowers (Claude plugin system)
- find-skills (skills.sh marketplace, Claude plugin)

## Sequential Thinking MCP

The `kp-sequential-thinking` MCP server works identically with Gemini CLI — it's MCP-native. Gemini even has a pre-tuned profile (`gemini_flash`) in the server that optimizes explore counts and branching thresholds for Gemini's strengths:

- Wider exploration (5-7 alternatives vs Claude's 4-5)
- Lower depth (2 layers vs 3)
- Creative guidance emphasizing Gemini's broad association patterns

## Best Practices for Cross-Runtime Skills

When writing new skills intended for both Claude and Gemini:

1. **Describe intent, not tools**: "Search the codebase for X" not "Use Grep to find X"
2. **Use absolute paths**: Gemini is more sensitive to relative path issues
3. **Include verification commands**: Both runtimes benefit from explicit `bash` verification steps
4. **Avoid tool-specific parameters**: "Read lines 40-80 of auth.ts" works on both, while `Read(file_path="auth.ts", offset=40, limit=40)` is Claude-specific
