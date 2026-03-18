#!/usr/bin/env bash
# kinderpowers setup.sh — post-install symlink wiring
# Idempotent: safe to re-run at any time.
# Use --force to replace existing real files/directories with symlinks.

set -euo pipefail

PLUGIN_ROOT="$(cd "$(dirname "$0")" && pwd)"
CLAUDE_DIR="${HOME}/.claude"
FORCE=false

for arg in "$@"; do
  case "$arg" in
    --force) FORCE=true ;;
  esac
done

echo "=== kinderpowers setup ==="
echo "Plugin root: ${PLUGIN_ROOT}"
[ "$FORCE" = true ] && echo "Mode: --force (replacing existing files)"
echo ""

# --- Helpers ---
link_dir() {
  local target="$1" link="$2"
  if [ -L "$link" ]; then
    rm "$link"
  elif [ -d "$link" ]; then
    if [ "$FORCE" = true ]; then
      echo "  Backing up: $link -> ${link}.bak"
      mv "$link" "${link}.bak"
    else
      echo "  SKIP: $link exists (use --force to replace)"
      return
    fi
  fi
  ln -s "$target" "$link"
  echo "  OK: $link -> $target"
}

link_file() {
  local target="$1" link="$2"
  if [ -L "$link" ]; then
    rm "$link"
  elif [ -f "$link" ]; then
    if [ "$FORCE" = true ]; then
      mv "$link" "${link}.bak"
    else
      echo "  SKIP: $link exists (use --force to replace)"
      return
    fi
  fi
  ln -s "$target" "$link"
  echo "  OK: $(basename "$link")"
}

# --- 1. GSD runtime ---
echo "[1/4] GSD runtime"
mkdir -p "${CLAUDE_DIR}"
link_dir "${PLUGIN_ROOT}/gsd" "${CLAUDE_DIR}/get-shit-done"

# --- 2. GSD commands ---
echo "[2/4] GSD commands"
mkdir -p "${CLAUDE_DIR}/commands"
link_dir "${PLUGIN_ROOT}/commands/gsd" "${CLAUDE_DIR}/commands/gsd"

# --- 3. GSD agents ---
echo "[3/4] GSD agents"
mkdir -p "${CLAUDE_DIR}/agents"
for agent in "${PLUGIN_ROOT}"/agents/gsd-*.md; do
  [ -f "$agent" ] || continue
  name="$(basename "$agent")"
  link_file "$agent" "${CLAUDE_DIR}/agents/${name}"
done

# --- 4. Hookify rules (optional) ---
echo "[4/4] Hookify rules"
HOOKIFY_RULES_DIR=""

# Search for hookify rules directory
if [ -d "${CLAUDE_DIR}/hookify/rules" ]; then
  HOOKIFY_RULES_DIR="${CLAUDE_DIR}/hookify/rules"
else
  # Check plugin cache locations
  for dir in "${CLAUDE_DIR}"/plugins/cache/*/hookify/*/rules; do
    if [ -d "$dir" ]; then
      HOOKIFY_RULES_DIR="$dir"
      break
    fi
  done
fi

if [ -n "$HOOKIFY_RULES_DIR" ]; then
  for rule in "${PLUGIN_ROOT}"/hookify-rules/*.local.md; do
    [ -f "$rule" ] || continue
    name="$(basename "$rule")"
    link_file "$rule" "${HOOKIFY_RULES_DIR}/${name}"
  done
  echo "  Hookify rules linked to: ${HOOKIFY_RULES_DIR}"
else
  echo "  Hookify not detected — skipping rule installation"
  echo "  (Install hookify, then re-run this script)"
fi

echo ""
echo "=== setup complete ==="
