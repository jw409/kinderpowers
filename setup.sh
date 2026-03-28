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
# GSD workflows reference ~/.claude/get-shit-done at runtime
echo "[1/3] GSD runtime"
mkdir -p "${CLAUDE_DIR}"
if [ -d "${PLUGIN_ROOT}/gsd" ]; then
  link_dir "${PLUGIN_ROOT}/gsd" "${CLAUDE_DIR}/get-shit-done"
else
  echo "  WARN: ${PLUGIN_ROOT}/gsd not found — skipping GSD symlink"
fi

# NOTE: GSD commands and agents are NOT symlinked here.
# The plugin system registers them under the kinderpowers: namespace
# automatically (kinderpowers:gsd:* skills, kinderpowers:gsd-* agents).
# Symlinking into ~/.claude/commands/ and ~/.claude/agents/ would create
# duplicates (gsd:* AND kinderpowers:gsd:*) that confuse users and models.

# --- 2. Hookify rules (optional) ---
echo "[2/3] Hookify rules"
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

# --- 3. Agent outcome logger hook ---
echo "[3/3] Agent outcome logger"
KP_DIR="${HOME}/.kinderpowers"
KP_HOOKS="${KP_DIR}/hooks"
mkdir -p "$KP_HOOKS"

# Copy hook script
HOOK_SRC="${PLUGIN_ROOT}/hooks/agent-outcome-logger.py"
HOOK_DST="${KP_HOOKS}/agent-outcome-logger.py"
if [ -f "$HOOK_SRC" ]; then
  cp "$HOOK_SRC" "$HOOK_DST"
  chmod +x "$HOOK_DST"
  echo "  OK: agent-outcome-logger.py -> ${HOOK_DST}"
else
  echo "  SKIP: hook source not found at ${HOOK_SRC}"
fi

# Register in settings.json (idempotent)
SETTINGS_FILE="${CLAUDE_DIR}/settings.json"
HOOK_ENTRY="{\"matcher\":\"Agent\",\"command\":\"python3 ${HOOK_DST}\"}"

if grep -q "agent-outcome-logger" "$SETTINGS_FILE" 2>/dev/null; then
  echo "  OK: hook already registered in settings.json"
elif command -v jq >/dev/null 2>&1; then
  # jq available — auto-register
  if [ ! -f "$SETTINGS_FILE" ]; then
    # Create settings.json with just the hooks section
    jq -n --argjson entry "$HOOK_ENTRY" \
      '{"hooks":{"PostToolUse":[$entry]}}' > "$SETTINGS_FILE"
    echo "  OK: created ${SETTINGS_FILE} with PostToolUse hook"
  elif ! jq -e '.hooks' "$SETTINGS_FILE" >/dev/null 2>&1; then
    # File exists but no hooks key — add it
    jq --argjson entry "$HOOK_ENTRY" \
      '.hooks = {"PostToolUse":[$entry]}' "$SETTINGS_FILE" > "${SETTINGS_FILE}.tmp" \
      && mv "${SETTINGS_FILE}.tmp" "$SETTINGS_FILE"
    echo "  OK: added hooks.PostToolUse to settings.json"
  elif ! jq -e '.hooks.PostToolUse' "$SETTINGS_FILE" >/dev/null 2>&1; then
    # hooks exists but no PostToolUse array — add it
    jq --argjson entry "$HOOK_ENTRY" \
      '.hooks.PostToolUse = [$entry]' "$SETTINGS_FILE" > "${SETTINGS_FILE}.tmp" \
      && mv "${SETTINGS_FILE}.tmp" "$SETTINGS_FILE"
    echo "  OK: added PostToolUse array to settings.json"
  else
    # PostToolUse array exists — append our entry
    jq --argjson entry "$HOOK_ENTRY" \
      '.hooks.PostToolUse += [$entry]' "$SETTINGS_FILE" > "${SETTINGS_FILE}.tmp" \
      && mv "${SETTINGS_FILE}.tmp" "$SETTINGS_FILE"
    echo "  OK: appended agent-outcome-logger to PostToolUse hooks"
  fi
else
  # No jq — fall back to manual instructions
  echo "  NOTE: jq not found — cannot auto-register hook."
  echo "  Add this to your ${SETTINGS_FILE} hooks.PostToolUse array:"
  echo '    {'
  echo '      "matcher": "Agent",'
  echo "      \"command\": \"python3 ${HOOK_DST}\""
  echo '    }'
fi

echo ""
echo "=== setup complete ==="
