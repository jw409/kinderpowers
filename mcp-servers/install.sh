#!/bin/bash
set -euo pipefail

# kp-mcp-servers installer
# Builds and registers both MCP servers with Claude Code

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUST_MIN_STACK=16777216
export RUST_MIN_STACK
FORCE_BUILD=0

# Parse flags
for arg in "$@"; do
  case "$arg" in
    --build) FORCE_BUILD=1 ;;
    *) echo "Unknown flag: $arg"; exit 1 ;;
  esac
done

echo "=== kp-mcp-servers installer ==="
echo ""

if ! command -v claude &>/dev/null; then
  echo "WARNING: claude CLI not found — will build but skip registration"
  SKIP_REGISTER=1
else
  SKIP_REGISTER=0
fi

# Platform detection for pre-built binaries
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$OS" in
  linux)  PLATFORM_DIR="linux-${ARCH}" ;;
  darwin) PLATFORM_DIR="macos-${ARCH}" ;;
  *)      PLATFORM_DIR="" ;;
esac

PREBUILT_DIR="$SCRIPT_DIR/bin/${PLATFORM_DIR}"

# Try pre-built binaries first
if [ "$FORCE_BUILD" = "0" ] && [ -n "$PLATFORM_DIR" ] && [ -d "$PREBUILT_DIR" ]; then
  PREBUILT_GITHUB="$PREBUILT_DIR/kp-github-mcp"
  PREBUILT_STEPWISE="$PREBUILT_DIR/kp-stepwise"

  if [ -x "$PREBUILT_GITHUB" ] && [ -x "$PREBUILT_STEPWISE" ]; then
    echo "Found pre-built binaries for ${PLATFORM_DIR}"
    GITHUB_BIN="$PREBUILT_GITHUB"
    STEPWISE_BIN="$PREBUILT_STEPWISE"
    echo "  kp-github-mcp:          $(du -h "$GITHUB_BIN" | cut -f1)"
    echo "  kp-stepwise: $(du -h "$STEPWISE_BIN" | cut -f1)"
    echo "  (use --build to force cargo build)"
    echo ""
  else
    echo "Pre-built binaries incomplete for ${PLATFORM_DIR}, falling back to cargo build"
    FORCE_BUILD=1
  fi
else
  FORCE_BUILD=1
fi

if [ "$FORCE_BUILD" = "1" ]; then
  # Check prerequisites
  if ! command -v cargo &>/dev/null; then
    echo "ERROR: cargo not found. Install Rust: https://rustup.rs"
    exit 1
  fi

  # Build kp-github-mcp (subshell preserves cwd)
  echo "[1/4] Building kp-github-mcp..."
  ( cd "$SCRIPT_DIR/github" && cargo build --release 2>&1 | grep -E "Compiling|Finished|error|warning" | tail -20 ) || true
  GITHUB_BIN="$SCRIPT_DIR/github/target/release/kp-github-mcp"
  if [ ! -f "$GITHUB_BIN" ]; then
    echo "ERROR: kp-github-mcp build failed"
    exit 1
  fi
  echo "  Built: $GITHUB_BIN ($(du -h "$GITHUB_BIN" | cut -f1))"

  # Build kp-stepwise (subshell preserves cwd)
  echo "[2/4] Building kp-stepwise..."
  ( cd "$SCRIPT_DIR/stepwise" && cargo build --release 2>&1 | grep -E "Compiling|Finished|error|warning" | tail -20 ) || true
  STEPWISE_BIN="$SCRIPT_DIR/stepwise/target/release/kp-stepwise"
  if [ ! -f "$STEPWISE_BIN" ]; then
    echo "ERROR: kp-stepwise build failed"
    exit 1
  fi
  echo "  Built: $STEPWISE_BIN ($(du -h "$STEPWISE_BIN" | cut -f1))"
fi

if [ "$SKIP_REGISTER" = "1" ]; then
  echo ""
  echo "Binaries ready. Register manually with:"
  echo "  claude mcp add kp-github --transport stdio -- $GITHUB_BIN"
  echo "  claude mcp add kp-stepwise --transport stdio -- $STEPWISE_BIN"
  exit 0
fi

# Register with Claude Code
echo "[3/4] Registering kp-github-mcp..."
claude mcp remove kp-github 2>/dev/null || true
claude mcp add kp-github --transport stdio -- "$GITHUB_BIN" 2>&1

echo "[4/4] Registering kp-stepwise..."
claude mcp remove kp-stepwise 2>/dev/null || true
claude mcp add kp-stepwise --transport stdio -- "$STEPWISE_BIN" 2>&1

echo ""
echo "=== Done ==="
echo "kp-github-mcp:          63 tools, 4 resources"
echo "kp-stepwise: 1 tool (stepwise_plan)"
echo ""
echo "Restart Claude Code or run /mcp to connect."
echo ""
echo "Optional: disable the official GitHub plugin:"
echo "  In settings.json, set \"github@claude-plugins-official\": false"
