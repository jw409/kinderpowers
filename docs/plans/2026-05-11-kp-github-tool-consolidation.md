# kp-github MCP: tool consolidation by resource

**Status:** Spec — implementation pending
**Target version:** kp-github-mcp 0.2.0 (breaking, hard cutover)
**Date:** 2026-05-11

## Problem

`kp-github-mcp` currently registers 65 MCP tools, one per (resource, verb) pair: `github_labels_list`, `github_labels_get`, `github_labels_create`, etc. Each tool ships its own parameter schema with redundant `owner`/`repo`/`fields`/`format` descriptions, so the total registered tool surface weighs ~22k tokens in a consumer's context. On a 1M-context model this is fine; on smaller contexts it crowds out actual work. The schema redundancy also makes the server harder to read — five label tools are five copies of the same pattern.

Vanta's MCP server in the same ecosystem demonstrates a tighter shape: one tool per resource, with discriminant params (e.g., `controlId` present → get; absent → list). We can apply the same shape here.

## Goal

Reduce the registered tool surface from 65 tools to ~25 tools while preserving every existing capability. Target: ~8k tokens of registered tool schemas (≈64% reduction).

## Approach

For each resource group, collapse N verb-specific tools into one tool with an `action` enum parameter and per-action optional fields. The existing per-action functions in `mcp-servers/github/src/tools/<resource>.rs` already factor the work cleanly — only `server.rs` (the `#[rmcp::tool]` registrations) and the param structs need to change. Client logic stays.

### Resource → tool mapping

| New tool                    | Replaces (count) | Actions                                                                                                                   |
|-----------------------------|------------------|---------------------------------------------------------------------------------------------------------------------------|
| `github_labels`             | 5                | `list`, `get`, `create`, `update`, `delete`                                                                               |
| `github_releases`           | 4                | `list`, `latest`, `get_by_tag`, `create`                                                                                  |
| `github_branches`           | 2                | `list`, `create`                                                                                                          |
| `github_tags`               | 2                | `list`, `get`                                                                                                             |
| `github_teams`              | 2                | `list`, `members`                                                                                                         |
| `github_commits`            | 2                | `list`, `get`                                                                                                              |
| `github_actions`            | 5                | `list_workflows`, `list_runs`, `get_run`, `rerun`, `run_logs`                                                             |
| `github_files`              | 4                | `get`, `create_or_update`, `delete`, `push`                                                                               |
| `github_repos`              | 5                | `get`, `create`, `fork`, `compare`, `search`                                                                              |
| `github_releases`           | (above)          |                                                                                                                            |
| `github_issues`             | 10               | `list`, `get`, `create`, `update`, `comment`, `comments`, `labels`, `search`, `sub_issues`, `list_types`                  |
| `github_prs`                | 11               | `list`, `get`, `create`, `update`, `merge`, `search`, `diff`, `files`, `status`, `checks`, `update_branch`                |
| `github_prs_reviews`        | 6                | `list`, `create`, `submit`, `delete`, `add_comment`, `comments`                                                           |
| `github_prs_comments`       | 2                | `list`, `reply`                                                                                                            |
| `github_users`              | 2                | `me`, `search`                                                                                                             |
| `github_code_search`        | 1                | (kept as-is; no peer verbs)                                                                                                |

Total: 16 tools (down from 65). Estimated registered-schema budget: ~7–9k tokens.

### Param struct pattern

Use a flat struct with an `action` enum discriminant and per-action optional fields. Flat structs generate the cleanest JSON Schema in `rmcp` — tagged enums produce nested schemas that bloat the very thing we're trimming. Example for labels:

```rust
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LabelAction {
    List,
    Get,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LabelsParams {
    pub action: LabelAction,
    pub owner: String,
    pub repo: String,
    /// Label name. Required for `get`, `update`, `delete`.
    #[serde(default)]
    pub name: Option<String>,
    /// New name (rename). Used by `update`.
    #[serde(default)]
    pub new_name: Option<String>,
    /// Hex color without `#`. Used by `create`, `update`.
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// Pagination limit. Used by `list`.
    #[serde(default)]
    pub limit: Option<u32>,
    // shared output controls
    #[serde(default)]
    pub fields: Option<Vec<String>>,
    #[serde(default)]
    pub format: Option<String>,
}
```

Server dispatch:

```rust
#[rmcp::tool(name = "github_labels", description = "GitHub labels: list, get, create, update, delete.")]
async fn github_labels(&self, Parameters(p): Parameters<LabelsParams>) -> Result<CallToolResult, McpError> {
    match p.action {
        LabelAction::List   => /* call tools::labels::list */,
        LabelAction::Get    => /* require p.name */,
        LabelAction::Create => /* require p.name, p.color optional */,
        LabelAction::Update => /* require p.name */,
        LabelAction::Delete => /* require p.name */,
    }
}
```

Missing required fields per action return `McpError::invalid_params` with a clear message identifying which field the chosen action needed. Do not silently fall back — fail loud.

## Versioning + compatibility

- **Hard cutover.** No alias layer. Bump `Cargo.toml` to `0.2.0`. Old tool names disappear.
- Consumers (kinderpowers' own agents, skills, hooks; downstream meshly-developer; any other kp consumers) update call sites in the same release window.
- Release notes spell out the full rename table so consumers can grep their codebases.

## Out of scope

- Read/write server split (separate MCP servers for read-only vs mutating tools). Considered, not pursued in this pass — revisit after measuring v0.2.0 footprint in consumer contexts.
- Deferred-tool / `ToolSearch` integration. The MCP harness already supports deferred tools, but enabling it for kp-github is independent of this refactor and can land separately.
- Behavioral changes to underlying GitHub API calls. The functions in `tools/<resource>.rs` are not modified.

## Test plan

- Existing integration tests in `mcp-servers/github/tests/` must be updated to call new tool names with `action` parameters.
- Add per-action coverage for each new consolidated tool: at minimum, one happy-path test per action and one missing-required-field test per action.
- Hand-test through Claude Code (or any MCP client) against a real repo to confirm schemas render usefully in tool pickers.
- Run `cargo test -p kp-github-mcp` and `cargo clippy -p kp-github-mcp -- -D warnings`.

## Release

After tests pass, rebuild prebuilt binaries (see existing `build: update pre-built MCP server binaries [skip ci]` commits as precedent) and bump the consumer references that pin a binary hash.

## Implementation order (suggested)

Start with the smallest groups to establish the pattern, then scale up:

1. `github_branches` (2 actions) — pilot
2. `github_tags`, `github_teams`, `github_commits` (2 actions each)
3. `github_labels` (5)
4. `github_releases`, `github_files` (4 each)
5. `github_actions`, `github_repos`, `github_users` (5/5/2)
6. `github_issues` (10)
7. `github_prs` + `github_prs_reviews` + `github_prs_comments` (19 across 3 tools)

Land each as its own commit on this branch. Don't ship a monolithic diff — one resource per commit lets the review check the pattern incrementally.
