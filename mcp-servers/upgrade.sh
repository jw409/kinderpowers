#!/bin/bash
set -euo pipefail

# kp-mcp-servers upgrader
# Pulls latest, rebuilds, and re-registers

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
KP_ROOT="$(dirname "$SCRIPT_DIR")"
RUST_MIN_STACK=16777216
export RUST_MIN_STACK

echo "=== kp-mcp-servers upgrade ==="

# Pull latest
echo "[1/3] Pulling latest kinderpowers..."
cd "$KP_ROOT"
BEFORE=$(git rev-parse HEAD)
git pull --rebase origin main 2>&1 | tail -3
AFTER=$(git rev-parse HEAD)

if [ "$BEFORE" = "$AFTER" ]; then
  echo "  Already up to date."
  echo ""
  read -p "Rebuild anyway? [y/N] " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Nothing to do."
    exit 0
  fi
else
  echo "  Updated: $(git log --oneline "$BEFORE..$AFTER" | wc -l) new commits"
  git log --oneline "$BEFORE..$AFTER" | head -10
fi

# Rebuild
echo ""
echo "[2/3] Rebuilding..."
cd "$SCRIPT_DIR/github"
cargo build --release 2>&1 | grep -E "Compiling kp-github|Finished|error" || true
echo "  kp-github-mcp: $(du -h target/release/kp-github-mcp | cut -f1)"

cd "$SCRIPT_DIR/sequential-thinking"
cargo build --release 2>&1 | grep -E "Compiling kp-sequential|Finished|error" || true
echo "  kp-sequential-thinking: $(du -h target/release/kp-sequential-thinking | cut -f1)"

# Re-register (binaries are in the same path, so this just ensures config is correct)
echo ""
echo "[3/3] Verifying registration..."
if command -v claude &>/dev/null; then
  claude mcp list 2>/dev/null | grep -E "kp-github|kp-seqthink" || echo "  WARNING: servers not registered. Run install.sh first."
fi

echo ""
echo "=== Upgrade complete ==="
echo "Restart Claude Code or run /mcp to reconnect."
