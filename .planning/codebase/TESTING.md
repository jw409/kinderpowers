# Testing Patterns

**Analysis Date:** 2026-03-19

## Overview

Three distinct test suites, each with its own framework:

| Language | Framework | Location | Run Command |
|----------|-----------|----------|-------------|
| Rust | `cargo test` (built-in) | `mcp-servers/sequential-thinking/src/**/tests` + `tests/integration.rs` | `cargo test` |
| Node.js | Node built-in `node:test` | `gsd/tests/*.test.cjs` | `npm test` (in `gsd/`) |
| Python | Manual runner (no framework) | `test_scanner.py` | `python test_scanner.py` |

---

## Rust: Unit Tests

### Framework

```
cargo test                          # Run all tests (unit + integration)
cargo test -- --nocapture           # Show stdout/stderr during tests
cargo test test_name                # Run specific test by name
```

No external test framework. Uses standard `#[test]` and `#[tokio::test]` attributes.

### Test File Organization

Unit tests live in `#[cfg(test)] mod tests` blocks **inside the same file as the code under test**:

- `mcp-servers/sequential-thinking/src/thinking.rs` — ~720 lines of tests for `ThinkingEngine`, `validate()`, `process()`, `format_thought()`, `wrap_text()`, hints, merges
- `mcp-servers/sequential-thinking/src/server.rs` — ~260 lines of tests for `SeqThinkServer`, `SequentialThinkingParams`, `detect_client_type()`
- `mcp-servers/sequential-thinking/src/profiles.rs` — ~150 lines of tests for `TuningProfile` loading, regex matching, fallback
- `mcp-servers/sequential-thinking/src/logging.rs` — ~210 lines of tests for `PersistentLogger`

Integration tests live in a separate file: `mcp-servers/sequential-thinking/tests/integration.rs`

### Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{fallback_profile, default_profiles};

    // Factory function — defined once, reused by all tests in the module
    fn make_engine() -> ThinkingEngine {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let profile = fallback_profile();
        ThinkingEngine::new(profile, "test-model".into(), "test-client".into())
    }

    fn make_thought(num: u32, total: u32) -> ThoughtData {
        ThoughtData {
            thought: format!("Thought number {}", num),
            thought_number: num,
            total_thoughts: total,
            next_thought_needed: true,
            // All optional fields set to None explicitly
            is_revision: None,
            ...
        }
    }

    #[test]
    fn validate_empty_thought_rejected() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.thought = String::new();
        let result = engine.validate(t);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("non-empty"));
    }
}
```

### Test Naming Convention

Rust: `{subject}_{condition}_{expected_outcome}` pattern:
- `validate_empty_thought_rejected`
- `validate_clamps_confidence`
- `process_first_thought_has_guidance`
- `process_compliance_resets_on_branch`
- `engine_branches_after_branching`
- `tool_method_poisoned_lock_returns_error`

### Factory Patterns

Each module defines local factory functions that encapsulate test setup:

```rust
// Reusable engine factory — disables file logging for test isolation
fn make_engine() -> ThinkingEngine {
    std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
    ThinkingEngine::new(fallback_profile(), "test-model".into(), "test-client".into())
}

// Variant that enables logging to test stderr warning paths
fn make_engine_with_logging() -> ThinkingEngine {
    std::env::remove_var("DISABLE_THOUGHT_LOGGING");
    ThinkingEngine::new(fallback_profile(), "test-model-logging".into(), "test-client".into())
}

// Logger factory using test-friendly path injection constructor
fn make_logger_in_tmp(tmp: &tempfile::TempDir) -> PersistentLogger {
    let log_file = tmp.path().join("test.jsonl");
    PersistentLogger::new_with_path(Some(log_file), "test-model", "test-client", "Default")
}
```

### Test-Friendly Constructor Pattern

When a production struct has non-deterministic behavior (file paths, env vars), a `#[cfg(test)]` constructor is added that accepts explicit dependencies:

```rust
// Production constructor — discovers paths from env vars
pub fn new(model_id: &str, client_type: &str, profile_name: &str) -> Self { ... }

// Test constructor — caller controls the log file path
#[cfg(test)]
pub(crate) fn new_with_path(log_file: Option<PathBuf>, ...) -> Self {
    Self { session_id: "test-session".into(), log_file, ... }
}
```

This avoids env var setup/teardown in most tests.

### Env Var Tests

Env var–dependent tests are serialized into a single `#[test]` function to avoid parallel race conditions. The comment pattern is explicit:

```rust
// NOTE: env var tests must be serialized to avoid parallel env var conflicts.
#[test]
fn detect_client_type_all_branches() {
    // Clear all detection env vars first
    std::env::remove_var("CLAUDE_CODE_VERSION");
    // ... test all branches ...
    std::env::remove_var("CLAUDE_CODE_VERSION");
}
```

Similarly, multiple file scenarios that use the same env var are grouped into one test function (`load_profiles_file_scenarios`).

### Mocking

No mock framework. The pattern is:
1. **Environment substitution:** Set env vars before constructing the subject, remove after
2. **Temp files/dirs:** `tempfile::tempdir()` + `tempfile::NamedTempFile` for file-based tests
3. **Constructor injection:** `new_with_path()` style test constructors accept dependencies directly

```rust
#[test]
fn persist_writes_valid_jsonl() {
    let tmp = tempfile::tempdir().unwrap();
    let logger = make_logger_in_tmp(&tmp);
    logger.persist(&make_test_thought(1));
    // Assert on the actual file
    let content = fs::read_to_string(logger.log_file_path().unwrap()).unwrap();
    let record: serde_json::Value = serde_json::from_str(&content.trim()).unwrap();
    assert_eq!(record["thought"], "Test thought 1");
}
```

### What to `unwrap()` in Tests

`unwrap()` is used freely in test setup and assertions — panics on failure are acceptable. The intent is to fail fast with a clear panic message rather than propagating `Result`.

---

## Rust: Integration Tests

### File Location

`mcp-servers/sequential-thinking/tests/integration.rs`

### Pattern

Spawns the compiled binary via `Command` with `Stdio::piped()`, speaks JSON-RPC over stdin/stdout:

```rust
struct McpClient {
    child: tokio::process::Child,
    stdin: tokio::process::ChildStdin,
    reader: BufReader<tokio::process::ChildStdout>,
    next_id: u32,
}

impl McpClient {
    async fn new() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_kp-sequential-thinking"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .env("DISABLE_THOUGHT_LOGGING", "true")
            .spawn().expect("Failed to start server");
        // ... initialize MCP protocol ...
    }
}
```

Each test function gets its own `McpClient` instance (fresh server process per test).

```rust
#[tokio::test]
async fn test_branching() {
    let mut client = McpClient::new().await;
    let _ = client.tool_call("sequentialthinking", json!({ ... })).await;
    let resp = client.tool_call("sequentialthinking", json!({ "branchFromThought": 1, ... })).await;
    let parsed = McpClient::get_parsed(&resp);
    assert!(parsed["branches"].as_array().unwrap().iter().any(|b| b.as_str() == Some("approach-a")));
}
```

Integration test names use `test_` prefix (snake_case).

### Integration Test Helper Methods

```rust
fn get_text(resp: &Value) -> &str { ... }    // Extract text from MCP content[0]
fn get_parsed(resp: &Value) -> Value { ... } // Parse text as JSON
fn is_error(resp: &Value) -> bool { ... }    // Check for JSON-RPC protocol error
fn is_tool_error(resp: &Value) -> bool { ... } // Check for MCP tool-level error
```

### Coverage Target

No explicit coverage target configured. The `--strip` release profile and `opt-level = 2` in `Cargo.toml` indicate production build focus; test coverage is comprehensive by inspection (validation, branching, merging, compliance, hints, search, resource endpoints).

---

## Node.js Tests

### Framework

Node.js built-in test runner (`node:test`). No Jest, Mocha, or other test framework.

```bash
# From gsd/ directory:
npm test                    # Runs all *.test.cjs files via scripts/run-tests.cjs
npm run test:coverage       # c8 coverage — requires 70% line coverage on lib/*.cjs
```

### Test File Organization

All test files live in `gsd/tests/` with `.test.cjs` extension:

```
gsd/tests/
├── helpers.cjs               # Shared utilities (NOT a test file)
├── core.test.cjs             # core.cjs functions
├── phase.test.cjs            # Phase listing, sorting
├── config.test.cjs           # Config loading
├── state.test.cjs            # State management
├── frontmatter.test.cjs      # Frontmatter parsing
├── milestone.test.cjs        # Milestone detection
├── roadmap.test.cjs          # Roadmap parsing
├── template.test.cjs         # Template filling
├── dispatcher.test.cjs       # Dispatch logic
├── verify.test.cjs           # Verification commands
├── commands.test.cjs         # CLI command dispatch
├── model-profiles.test.cjs   # Model profile resolution
├── profile-output.test.cjs   # Profile output formatting
├── profile-pipeline.test.cjs # Pipeline orchestration
├── ...                       # (28 test files total)
```

### Test Structure

```javascript
const { test, describe, beforeEach, afterEach } = require('node:test');
const assert = require('node:assert');
const { createTempProject, cleanup } = require('./helpers.cjs');
const { loadConfig } = require('../get-shit-done/bin/lib/core.cjs');

describe('loadConfig', () => {
  let tmpDir;
  let originalCwd;

  beforeEach(() => {
    tmpDir = createTempProject();
    originalCwd = process.cwd();
  });

  afterEach(() => {
    process.chdir(originalCwd);
    cleanup(tmpDir);
  });

  // Local helper — defined inside describe block, not exported
  function writeConfig(obj) {
    fs.writeFileSync(
      path.join(tmpDir, '.planning', 'config.json'),
      JSON.stringify(obj, null, 2)
    );
  }

  test('returns defaults when config.json is missing', () => {
    const config = loadConfig(tmpDir);
    assert.strictEqual(config.model_profile, 'balanced');
  });
});
```

### Assertion Style

Uses `node:assert` with strict equality:
- `assert.strictEqual(a, b)` — primary assertion
- `assert.deepStrictEqual(a, b)` — for objects and arrays
- `assert.ok(condition, message)` — for truthy checks
- `assert.throws(() => { ... }, /pattern/)` — for error assertions

No `expect()` or matcher chains.

### Test Helpers (`gsd/tests/helpers.cjs`)

```javascript
// Create a temp project with .planning/phases/ scaffold
function createTempProject() {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'gsd-test-'));
  fs.mkdirSync(path.join(tmpDir, '.planning', 'phases'), { recursive: true });
  return tmpDir;
}

// Create temp project with a real git init + initial commit
function createTempGitProject() { ... }

// Clean up temp dir
function cleanup(tmpDir) {
  fs.rmSync(tmpDir, { recursive: true, force: true });
}

// Run gsd-tools CLI and capture output
function runGsdTools(args, cwd) {
  // Accepts string (shell-interpreted) or array (shell-bypassed)
  try {
    const result = execSync(`node "${TOOLS_PATH}" ${args}`, { cwd, encoding: 'utf-8' });
    return { success: true, output: result.trim() };
  } catch (err) {
    return { success: false, output: ..., error: ... };
  }
}
```

### CLI Testing Pattern

Tests that exercise the full CLI path use `runGsdTools()` and parse JSON output:

```javascript
test('lists phase directories sorted numerically', () => {
  fs.mkdirSync(path.join(tmpDir, '.planning', 'phases', '10-final'), { recursive: true });
  const result = runGsdTools('phases list', tmpDir);
  assert.ok(result.success, `Command failed: ${result.error}`);
  const output = JSON.parse(result.output);
  assert.deepStrictEqual(output.directories, ['01-foundation', '02-api', '10-final']);
});
```

### Regression Test Documentation

Known bug regressions are documented in the test name and comment:

```javascript
// Bug: loadConfig previously omitted model_overrides from return value
test('returns model_overrides when present (REG-01)', () => { ... });

// Bug: getRoadmapPhaseInternal was missing from module.exports
test('is exported from core.cjs (REG-02)', () => { ... });
```

### Coverage Configuration

```bash
npm run test:coverage
# c8 options:
#   --check-coverage
#   --lines 70
#   --include 'get-shit-done/bin/lib/*.cjs'
#   --exclude 'tests/**'
#   --all (includes un-tested files in report)
```

Target: 70% line coverage on `get-shit-done/bin/lib/*.cjs`.

---

## Python Tests

### Framework

No external framework. Self-contained runner at bottom of `test_scanner.py`:

```python
if __name__ == "__main__":
    tests = [v for k, v in globals().items() if k.startswith("test_")]
    for test in tests:
        try:
            test()
            print(f"  PASS: {test.__name__}")
        except AssertionError as e:
            print(f"  FAIL: {test.__name__}: {e}")
    print(f"\n{len(tests)} tests complete.")
```

Run with:
```bash
python test_scanner.py
```

### Test Structure

Flat functions, no classes. Each test function starts with `test_`:

```python
def test_detects_iron_law():
    path = _write_temp("## The Iron Law\nDo the thing.\n")
    findings = list(scan_file(path))
    assert any(f.severity == "high" and "Iron Law" in f.pattern for f in findings)

def test_allows_must_with_escape():
    path = _write_temp("You MUST do this unless there's a good reason.\n")
    findings = list(scan_file(path))
    must_findings = [f for f in findings if "MUST" in f.pattern]
    assert len(must_findings) == 0
```

### Test Helper

```python
def _write_temp(content: str, suffix: str = ".md") -> Path:
    """Write content to a temp file and return its path."""
    f = tempfile.NamedTemporaryFile(mode="w", suffix=suffix, delete=False)
    f.write(content)
    f.close()
    return Path(f.name)
```

Underscore prefix indicates it is a helper, not a test.

### What is Tested

- Positive detection: each COMPULSION_PATTERNS entry has a corresponding test
- Negative detection (escape clauses): MUST/NEVER with "unless" modifiers
- Directory scanning: multiple files, severity filtering
- Clean files: no false positives

### Note on Coverage

No coverage tool configured. Test file is co-located with source in the project root (`scanner.py` + `test_scanner.py`).

---

## Shell-Based Behavior Tests

`tests/` also contains shell-script behavior tests for AI agent skill triggering:

```
tests/
├── claude-code/              # Claude Code integration tests (bash scripts)
├── opencode/                 # OpenCode integration tests (bash scripts)
├── explicit-skill-requests/  # Skill detection with sample prompts (.txt)
├── skill-triggering/         # Trigger condition tests
└── subagent-driven-dev/      # Multi-agent development scenario tests
```

These are not automated unit tests — they require a running AI agent and are intended for manual or CI execution via `run-all.sh` / `run-tests.sh`.

---

## Cross-Cutting Test Patterns

### Isolation

All tests that touch the filesystem create isolated temp directories:
- **Rust:** `tempfile::tempdir()` — auto-cleaned on drop
- **Node.js:** `fs.mkdtempSync(path.join(os.tmpdir(), 'gsd-test-'))` — cleaned in `afterEach`
- **Python:** `tempfile.NamedTemporaryFile(delete=False)` — manual (no cleanup in test_scanner.py)

### State Mutation

Rust tests that test stateful engines run sequentially within a `#[test]` function (no async). When testing cumulative state (e.g., 5 consecutive thoughts), a `for` loop processes thoughts and assertions check state at each step.

### Error Path Coverage

Tests explicitly cover error cases alongside happy paths:
- Empty/zero inputs rejected
- Invalid JSON gracefully returns defaults
- Missing files return `null` or defaults
- Mutex poisoning returns `Err(McpError)`
- Write failures are fire-and-forget (no panic)

---

*Testing analysis: 2026-03-19*
