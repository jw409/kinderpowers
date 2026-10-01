# Coding Conventions

**Analysis Date:** 2026-03-19

## Languages

Three languages are in active use, each with distinct conventions:
- **Rust** — `mcp-servers/sequential-thinking/src/`
- **Node.js (CommonJS)** — `gsd/get-shit-done/bin/lib/`, `gsd/tests/`, `gsd/hooks/`, `lib/`
- **Python** — `scanner.py`, `test_scanner.py`, `hooks/`, `tests/claude-code/`

---

## Rust Conventions

### Naming Patterns

**Files:** snake_case module names matching their `mod` declarations (`main.rs`, `thinking.rs`, `profiles.rs`, `logging.rs`, `server.rs`)

**Structs/Types:** PascalCase — `ThinkingEngine`, `ThoughtData`, `TuningProfile`, `SeqThinkServer`, `PersistentLogger`, `ComplianceStats`, `MergeSummary`, `Hint`

**Functions/Methods:** snake_case — `process()`, `validate()`, `format_thought()`, `load_profiles()`, `get_profile_for_model()`, `fallback_profile()`, `wrap_text()`

**Constants/Statics:** Not present; magic numbers are documented inline with comments

**Fields:** snake_case — `thought_number`, `total_thoughts`, `branch_from_thought`, `model_id`

### Serde Conventions

All types that cross the JSON boundary use `#[serde(rename_all = "camelCase")]`. Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]`. Input structs use `#[serde(default)]` on optional fields rather than `Option<Option<T>>`.

```rust
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SequentialThinkingParams {
    pub thought: String,
    #[serde(default)]
    pub branch_id: Option<String>,
}
```

### Error Handling

**Production code:** Uses `anyhow::Result<()>` at `main()` level. Internal logic returns `Result<T, String>` for domain errors (validation). MCP errors use `rmcp::ErrorData` (aliased as `McpError`) with `McpError::internal_error()` and `McpError::invalid_params()`.

**Critical pattern:** Tool handler errors are returned as `Ok(CallToolResult::error(...))` not `Err(McpError)` — MCP protocol errors are reserved for protocol-level failures (e.g., poisoned mutex):

```rust
match engine.process(data) {
    Ok(response) => Ok(CallToolResult::success(vec![Content::text(text)])),
    Err(msg) => {
        let err_json = serde_json::json!({"error": msg, "status": "failed"});
        Ok(CallToolResult::error(vec![Content::text(text)]))
    }
}
```

**`unwrap()` policy:** Used freely in tests (test setup failures are acceptable panics). In production code, `unwrap_or_default()` or explicit error propagation is used instead.

### Struct Construction

Long structs with many optional fields use explicit field construction (not `..Default::default()` shortcuts) in tests. Factory functions like `make_params()` and `make_engine()` are defined in test modules for reuse:

```rust
fn make_engine() -> ThinkingEngine {
    std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
    let profile = fallback_profile();
    ThinkingEngine::new(profile, "test-model".into(), "test-client".into())
}
```

### Visibility

Public API: `pub fn`, `pub struct`. Internal API available to tests: `pub(crate) fn`. Test-only constructors gated with `#[cfg(test)]`:

```rust
#[cfg(test)]
pub(crate) fn new_with_path(log_file: Option<PathBuf>, ...) -> Self { ... }
```

### Logging

Uses `tracing` crate with structured fields:
```rust
tracing::info!(model = %model_id, profile = %profile.display_name, "server ready");
tracing::warn!(error = %e, path = %path.display(), "failed to read profiles");
```

All tracing output goes to `stderr`. `stdout` is reserved for MCP JSON-RPC.

### `#[allow(dead_code)]` Convention

Used with explanatory comments when APIs are intentionally public for future use:
```rust
#[allow(dead_code)] // Stored for future per-model analytics
model_id: String,

#[allow(dead_code)] // Available for diagnostics
pub(crate) fn log_file_path(&self) -> Option<&Path> { ... }
```

### Section Headers in Source

Large source files use section separator comments to group related items:
```rust
// ============================================================================
// MCP Parameter struct — maps to the tool's JSON Schema
// ============================================================================
```

---

## Node.js (CommonJS) Conventions

### Module Format

All Node.js files use CommonJS (`.cjs` extension). No ES modules. Exports via `module.exports = { ... }` at file bottom. Requires via `const { fn } = require(...)`.

### Naming Patterns

**Files:** kebab-case with `.cjs` extension — `core.cjs`, `model-profiles.cjs`, `profile-output.cjs`, `gsd-check-update.js`

**Functions:** camelCase — `loadConfig()`, `resolveModelInternal()`, `createTempProject()`, `runGsdTools()`, `getMilestoneInfo()`

**Variables:** camelCase — `tmpDir`, `phaseDir`, `originalCwd`

**Internal helpers:** suffix `Internal` marks functions that are exported for testing but not public API — `resolveModelInternal()`, `findPhaseInternal()`, `getRoadmapPhaseInternal()`, `generateSlugInternal()`

### Error Handling

Functions return `null` for not-found cases (not thrown exceptions). Config loading catches parse errors and returns defaults silently. Shell commands are wrapped in try/catch returning a structured `{ success, output, error }` object:

```javascript
function runGsdTools(args, cwd) {
  try {
    const result = execSync(`node "${TOOLS_PATH}" ${args}`, { cwd, encoding: 'utf-8' });
    return { success: true, output: result.trim() };
  } catch (err) {
    return { success: false, output: err.stdout?.trim() || '', error: err.stderr?.trim() || err.message };
  }
}
```

### Configuration Access

All config access goes through `loadConfig(projectRoot)` which returns a flat merged object. Config file is at `.planning/config.json`. The function handles missing files, invalid JSON, and nested-vs-flat key formats by falling back to defaults.

### JSDoc

Not used systematically. Block comments with `/** */` appear on module-level functions in test files. Inline `//` comments explain non-obvious behavior.

### File/Path Handling

`path.join()` everywhere, no string concatenation for paths. `fs.mkdtempSync()` for temp dirs in tests. `fs.rmSync(dir, { recursive: true, force: true })` for cleanup.

---

## Python Conventions

### Module Style

Standard library only (no third-party imports in `scanner.py`). Uses `dataclasses.dataclass` for structured data. Uses `typing` for type hints on function signatures. Generator functions (`Iterator[Finding]`) for streaming results.

### Naming Patterns

**Files:** snake_case — `scanner.py`, `test_scanner.py`

**Functions:** snake_case — `scan_file()`, `scan_directory()`, `format_finding()`, `_write_temp()` (underscore prefix for test helpers)

**Classes:** PascalCase — `Finding`

**Constants:** UPPER_SNAKE — `COMPULSION_PATTERNS`

### Type Annotations

Used on function signatures but not inline variables:
```python
def scan_file(path: Path) -> Iterator[Finding]:
def scan_directory(path: Path, extensions: tuple = (".md",)) -> Iterator[Finding]:
def format_finding(f: Finding, verbose: bool = False) -> str:
```

### Error Handling

Exceptions caught broadly in I/O operations, printed to `stderr`, and execution continues:
```python
try:
    content = path.read_text()
except Exception as e:
    print(f"Warning: Could not read {path}: {e}", file=sys.stderr)
    return
```

`sys.exit(main())` pattern for CLI entry point with integer return codes.

### Docstrings

One-line docstrings on module, classes, and public functions:
```python
def scan_file(path: Path) -> Iterator[Finding]:
    """Scan a single file for compulsion-language patterns."""
```

---

## Import Organization

**Rust:** `use` statements grouped — stdlib (`std::*`), then external crates (`serde`, `rmcp`, etc.), then internal (`crate::*`). No blank lines between `use` statements within a group.

**Node.js:** No enforced ordering. Typical pattern: stdlib (`require('fs')`, `require('path')`), then local (`require('./helpers.cjs')`, `require('../get-shit-done/bin/lib/core.cjs')`).

**Python:** stdlib imports in alphabetical order (`argparse`, `dataclasses`, `pathlib`, `re`, `sys`, `typing`). No blank lines between stdlib imports.

---

## Comments

**When to comment:**
- Section separators for large files (Rust `// ===` style)
- Explain `#[allow(dead_code)]` with future-use rationale
- Regression test bugs: `// Bug: loadConfig previously omitted model_overrides` or inline `// REG-01`, `// REG-02`
- Non-obvious defaults or protocol choices (e.g., `// MCP uses stdout for JSON-RPC`)
- Env var race conditions in tests: `// NOTE: env var tests must be serialized to avoid races`

**Pattern for documenting intentional API design:**
```rust
/// A non-prescriptive hint the server surfaces. The caller decides what to do.
pub struct Hint { ... }
```

---

## Function Design

**Rust:** Functions do one thing. Validation separated from processing (`validate()` then `process()`). Pure functions (no side effects) for formatting and text operations (`format_thought()`, `wrap_text()`).

**Node.js:** Helper factories encapsulate setup complexity (`createTempProject()`, `createTempGitProject()`). Config writers as local `function writeConfig(obj)` inside `describe` blocks.

**Python:** Generator functions for streaming large results. Data classes for structured outputs rather than dicts.

---

## Module Design

**Rust:** One `mod` per file, declared in `main.rs`. No barrel `mod.rs` files. Public surface kept minimal — prefer `pub(crate)` over `pub` for internal helpers.

**Node.js:** `module.exports` at bottom of each file, explicit object listing all exports. `Internal` suffix marks test-visible but not user-facing functions.

---

*Convention analysis: 2026-03-19*
