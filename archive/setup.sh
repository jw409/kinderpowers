#!/usr/bin/env bash
# kinderpowers setup.sh — post-install wiring, as selectable modules.
# Idempotent: safe to re-run at any time.
#
#   ./setup.sh                        install the default modules
#   ./setup.sh --list                 show every module and whether it's default
#   ./setup.sh --with task-observer   add an opt-in module
#   ./setup.sh --without gsd          skip a default module
#   ./setup.sh --only kinderpowers    install exactly these (repeatable)
#   ./setup.sh --force                replace existing real files with symlinks
#
# Modules are separated so a machine can take the parts it wants. The two
# third-party libraries are opt-in and are NOT installed by default — they are
# other people's work with their own licenses, listed in the README appendix.
# Neither is vendored here: each module invokes its upstream installer so the
# code arrives from, and stays attributable to, its own author.

set -euo pipefail

PLUGIN_ROOT="$(cd "$(dirname "$0")" && pwd)"
CLAUDE_DIR="${HOME}/.claude"
KP_DIR="${HOME}/.kinderpowers"
FORCE=false

# module id | default? | description
MODULES=(
  "kinderpowers|yes|Hookify enforcement rules + agent outcome logger hook"
  "gsd|yes|GSD lifecycle runtime (~/.claude/get-shit-done)"
  "mattpocock-skills|no|Third-party: mattpocock/skills (MIT) via its own installer"
  "task-observer|no|Third-party: rebelytics/one-skill-to-rule-them-all (CC BY 4.0)"
)

module_ids() { for m in "${MODULES[@]}"; do echo "${m%%|*}"; done; }
module_field() {  # $1=id $2=2|3
  for m in "${MODULES[@]}"; do
    [ "${m%%|*}" = "$1" ] && echo "$m" | cut -d'|' -f"$2" && return
  done
}

usage_modules() {
  echo "Modules:"
  for m in "${MODULES[@]}"; do
    IFS='|' read -r id def desc <<< "$m"
    printf "  %-18s %-10s %s\n" "$id" "$([ "$def" = yes ] && echo "[default]" || echo "[opt-in]")" "$desc"
  done
}

# --- Argument parsing -----------------------------------------------------
SELECTED=()
WITH=()
WITHOUT=()
ONLY=()

while [ $# -gt 0 ]; do
  case "$1" in
    --force) FORCE=true ;;
    --list) usage_modules; exit 0 ;;
    --with) shift; WITH+=("${1:?--with needs a module id}") ;;
    --without) shift; WITHOUT+=("${1:?--without needs a module id}") ;;
    --only) shift; ONLY+=("${1:?--only needs a module id}") ;;
    -h|--help) sed -n '2,17p' "$0"; echo; usage_modules; exit 0 ;;
    *) echo "Unknown argument: $1" >&2; echo >&2; usage_modules >&2; exit 2 ;;
  esac
  shift
done

# Validate every id the caller named, so a typo fails loudly instead of
# silently installing nothing.
for id in "${WITH[@]:-}" "${WITHOUT[@]:-}" "${ONLY[@]:-}"; do
  [ -z "$id" ] && continue
  if ! module_ids | grep -qx "$id"; then
    echo "Unknown module: $id" >&2; echo >&2; usage_modules >&2; exit 2
  fi
done

if [ ${#ONLY[@]} -gt 0 ]; then
  SELECTED=("${ONLY[@]}")
else
  while read -r id; do
    [ "$(module_field "$id" 2)" = yes ] || continue
    skip=false
    for w in "${WITHOUT[@]:-}"; do [ "$w" = "$id" ] && skip=true; done
    [ "$skip" = true ] || SELECTED+=("$id")
  done < <(module_ids)
  for w in "${WITH[@]:-}"; do
    [ -z "$w" ] && continue
    already=false
    for s in "${SELECTED[@]:-}"; do [ "$s" = "$w" ] && already=true; done
    [ "$already" = true ] || SELECTED+=("$w")
  done
fi

echo "=== kinderpowers setup ==="
echo "Plugin root: ${PLUGIN_ROOT}"
[ "$FORCE" = true ] && echo "Mode: --force (replacing existing files)"
echo "Modules: ${SELECTED[*]:-none}"
echo ""

# --- Helpers --------------------------------------------------------------
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

# --- Module: gsd ----------------------------------------------------------
# GSD workflows reference ~/.claude/get-shit-done at runtime.
module_gsd() {
  echo "[gsd] GSD runtime"
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
}

# --- Module: kinderpowers -------------------------------------------------
module_kinderpowers() {
  echo "[kinderpowers] Hookify rules"
  local HOOKIFY_RULES_DIR=""

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
      link_file "$rule" "${HOOKIFY_RULES_DIR}/$(basename "$rule")"
    done
    echo "  Hookify rules linked to: ${HOOKIFY_RULES_DIR}"
  else
    echo "  Hookify not detected — skipping rule installation"
    echo "  (Install hookify, then re-run this script)"
  fi

  echo "[kinderpowers] Agent outcome logger"
  local KP_HOOKS="${KP_DIR}/hooks"
  mkdir -p "$KP_HOOKS"

  local HOOK_SRC="${PLUGIN_ROOT}/hooks/agent-outcome-logger.py"
  local HOOK_DST="${KP_HOOKS}/agent-outcome-logger.py"
  if [ -f "$HOOK_SRC" ]; then
    cp "$HOOK_SRC" "$HOOK_DST"
    chmod +x "$HOOK_DST"
    echo "  OK: agent-outcome-logger.py -> ${HOOK_DST}"
  else
    echo "  SKIP: hook source not found at ${HOOK_SRC}"
  fi

  # Register in settings.json (idempotent)
  local SETTINGS_FILE="${CLAUDE_DIR}/settings.json"
  local HOOK_ENTRY="{\"matcher\":\"Agent\",\"command\":\"python3 ${HOOK_DST}\"}"

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
}

# --- Module: mattpocock-skills (third-party, MIT) -------------------------
# Installed through its own marketplace entry rather than vendored, so it
# updates on its author's cadence and stays under his name. See README
# appendix "Adjacent skill libraries".
module_mattpocock_skills() {
  echo "[mattpocock-skills] Third-party skills by Matt Pocock (MIT)"
  echo "  Source: https://github.com/mattpocock/skills"
  if command -v claude >/dev/null 2>&1; then
    echo "  Running: claude plugin install mattpocock-skills"
    if claude plugin install mattpocock-skills; then
      echo "  OK: installed. Run /setup-matt-pocock-skills once per repo."
    else
      echo "  FAILED. Install by hand with one of:" >&2
      echo "    claude plugin install mattpocock-skills" >&2
      echo "    npx skills@latest add mattpocock/skills" >&2
      return 1
    fi
  else
    echo "  claude CLI not found. Install by hand with one of:"
    echo "    claude plugin install mattpocock-skills"
    echo "    npx skills@latest add mattpocock/skills"
  fi
  echo "  NOTE: pick ONE install path — the plugin and the skills.sh copy"
  echo "  duplicate every skill if both are present."
}

# --- Module: task-observer (third-party, CC BY 4.0) -----------------------
# Cloned rather than vendored. CC BY 4.0 requires attribution and a record of
# modifications; a pristine clone keeps LICENSE.txt and git history intact and
# leaves us nothing to re-license.
module_task_observer() {
  echo "[task-observer] Third-party meta-skill by Eoghan Henn / rebelytics (CC BY 4.0)"
  echo "  Source: https://github.com/rebelytics/one-skill-to-rule-them-all"
  if ! command -v git >/dev/null 2>&1; then
    echo "  git not found — cannot install." >&2
    return 1
  fi
  local VENDOR="${KP_DIR}/vendor/task-observer"
  mkdir -p "$(dirname "$VENDOR")"
  if [ -d "${VENDOR}/.git" ]; then
    echo "  Updating existing clone at ${VENDOR}"
    git -C "$VENDOR" pull --ff-only --quiet || echo "  WARN: pull failed; keeping existing checkout"
  else
    echo "  Cloning to ${VENDOR}"
    git clone --depth 1 --quiet \
      https://github.com/rebelytics/one-skill-to-rule-them-all.git "$VENDOR"
  fi
  if [ ! -f "${VENDOR}/SKILL.md" ]; then
    echo "  ERROR: ${VENDOR}/SKILL.md missing — upstream layout changed." >&2
    return 1
  fi
  mkdir -p "${CLAUDE_DIR}/skills"
  link_dir "$VENDOR" "${CLAUDE_DIR}/skills/task-observer"
  echo "  Attribution: Eoghan Henn (rebelytics), CC BY 4.0 — see ${VENDOR}/LICENSE.txt"
}

# --- Run ------------------------------------------------------------------
# Every module writes somewhere under ~/.claude. This used to be created as a
# side effect of the GSD step always running first; with modules selectable
# independently, that ordering is no longer guaranteed.
mkdir -p "${CLAUDE_DIR}"

status=0
for id in "${SELECTED[@]:-}"; do
  [ -z "$id" ] && continue
  case "$id" in
    kinderpowers)       module_kinderpowers ;;
    gsd)                module_gsd ;;
    mattpocock-skills)  module_mattpocock_skills || status=1 ;;
    task-observer)      module_task_observer || status=1 ;;
  esac
  echo ""
done

if [ "$status" -eq 0 ]; then
  echo "=== setup complete ==="
else
  echo "=== setup finished with errors (see above) ===" >&2
fi
exit "$status"
