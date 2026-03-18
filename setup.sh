#!/usr/bin/env bash
# kinderpowers setup.sh — post-install symlink wiring
# Idempotent: safe to re-run at any time.

set -euo pipefail

PLUGIN_ROOT="$(cd "$(dirname "$0")" && pwd)"
CLAUDE_DIR="${HOME}/.claude"

echo "=== kinderpowers setup ==="
echo "Plugin root: ${PLUGIN_ROOT}"
echo ""

# --- Helper ---
link_dir() {
  local target="$1" link="$2"
  if [ -L "$link" ]; then
    rm "$link"
  elif [ -d "$link" ]; then
    echo "  WARN: $link exists as real directory — skipping (back up and remove to allow symlink)"
    return
  fi
  ln -s "$target" "$link"
  echo "  OK: $link -> $target"
}

link_file() {
  local target="$1" link="$2"
  if [ -L "$link" ]; then
    rm "$link"
  elif [ -f "$link" ]; then
    echo "  WARN: $link exists as real file — skipping (back up and remove to allow symlink)"
    return
  fi
  ln -s "$target" "$link"
  echo "  OK: $link -> $target"
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
for dir in "${CLAUDE_DIR}"/plugins/cache/*/hookify/*/rules 2>/dev/null; do
  if [ -d "$dir" ]; then
    HOOKIFY_RULES_DIR="$dir"
    break
  fi
done

if [ -z "$HOOKIFY_RULES_DIR" ]; then
  # Try alternate location
  if [ -d "${CLAUDE_DIR}/hookify/rules" ]; then
    HOOKIFY_RULES_DIR="${CLAUDE_DIR}/hookify/rules"
  fi
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
  echo "  (Install hookify plugin, then re-run this script)"
fi

echo ""
echo "=== setup complete ==="
