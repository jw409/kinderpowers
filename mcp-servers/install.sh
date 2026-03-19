#!/bin/bash
set -euo pipefail

# kp-mcp-servers installer
# Builds and registers both MCP servers with Claude Code

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUST_MIN_STACK=16777216
export RUST_MIN_STACK

echo "=== kp-mcp-servers installer ==="
echo ""

# Check prerequisites
if ! command -v cargo &>/dev/null; then
  echo "ERROR: cargo not found. Install Rust: https://rustup.rs"
  exit 1
fi

if ! command -v claude &>/dev/null; then
  echo "WARNING: claude CLI not found — will build but skip registration"
  SKIP_REGISTER=1
else
  SKIP_REGISTER=0
fi

# Build kp-github-mcp
echo "[1/4] Building kp-github-mcp..."
cd "$SCRIPT_DIR/github"
cargo build --release 2>&1 | grep -E "Compiling kp-github|Finished|error" || true
GITHUB_BIN="$SCRIPT_DIR/github/target/release/kp-github-mcp"
if [ ! -f "$GITHUB_BIN" ]; then
  echo "ERROR: kp-github-mcp build failed"
  exit 1
fi
echo "  Built: $GITHUB_BIN ($(du -h "$GITHUB_BIN" | cut -f1))"

# Build kp-sequential-thinking
echo "[2/4] Building kp-sequential-thinking..."
cd "$SCRIPT_DIR/sequential-thinking"
cargo build --release 2>&1 | grep -E "Compiling kp-sequential|Finished|error" || true
SEQTHINK_BIN="$SCRIPT_DIR/sequential-thinking/target/release/kp-sequential-thinking"
if [ ! -f "$SEQTHINK_BIN" ]; then
  echo "ERROR: kp-sequential-thinking build failed"
  exit 1
fi
echo "  Built: $SEQTHINK_BIN ($(du -h "$SEQTHINK_BIN" | cut -f1))"

if [ "$SKIP_REGISTER" = "1" ]; then
  echo ""
  echo "Binaries built. Register manually with:"
  echo "  claude mcp add kp-github --transport stdio -- $GITHUB_BIN"
  echo "  claude mcp add kp-sequential-thinking --transport stdio -- $SEQTHINK_BIN"
  exit 0
fi

# Register with Claude Code
echo "[3/4] Registering kp-github-mcp..."
claude mcp remove kp-github 2>/dev/null || true
claude mcp add kp-github --transport stdio -- "$GITHUB_BIN" 2>&1

echo "[4/4] Registering kp-sequential-thinking..."
claude mcp remove kp-sequential-thinking 2>/dev/null || true
claude mcp add kp-sequential-thinking --transport stdio -- "$SEQTHINK_BIN" 2>&1

echo ""
echo "=== Done ==="
echo "kp-github-mcp:          63 tools, 4 resources"
echo "kp-sequential-thinking: 1 tool (sequentialthinking)"
echo ""
echo "Restart Claude Code or run /mcp to connect."
echo ""
echo "Optional: disable the official GitHub plugin:"
echo "  In settings.json, set \"github@claude-plugins-official\": false"
