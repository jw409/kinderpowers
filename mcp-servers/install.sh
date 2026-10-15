#!/bin/bash
set -euo pipefail

# Prepare the pinned MCP servers and register missing Claude Code entries.
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
KP_ROOT="$(dirname "$SCRIPT_DIR")"
export RUST_MIN_STACK="${RUST_MIN_STACK:-16777216}"
FORCE_BUILD=0
SKIP_REGISTER=0

for arg in "$@"; do
  case "$arg" in
    --build) FORCE_BUILD=1 ;;
    --no-register) SKIP_REGISTER=1 ;;
    --help) echo "Usage: $0 [--build] [--no-register]"; exit 0 ;;
    *) echo "Unknown flag: $arg" >&2; exit 1 ;;
  esac
done

echo "Preparing the MCP server commits pinned by this checkout..."
git -C "$KP_ROOT" submodule update --init -- mcp-servers/github mcp-servers/stepwise

if ! command -v claude >/dev/null 2>&1; then
  echo "Claude CLI is unavailable; registration commands will be printed."
  SKIP_REGISTER=1
fi

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) PLATFORM_DIR="linux-x86_64" ;;
  Darwin-arm64) PLATFORM_DIR="macos-arm64" ;;
  *) PLATFORM_DIR="" ;;
esac

prepare_binary() {
  local repo="$1" binary="$2" wrapper="$3"
  local source_dir="$SCRIPT_DIR/$repo"
  local prebuilt="$source_dir/bin/$PLATFORM_DIR/$binary"
  local built="$source_dir/target/release/$binary"

  if [ "$FORCE_BUILD" = "0" ] && [ -n "$PLATFORM_DIR" ] && [ -x "$prebuilt" ]; then
    if [ ! -x "$SCRIPT_DIR/bin/$wrapper" ]; then
      echo "ERROR: missing executable wrapper: $SCRIPT_DIR/bin/$wrapper" >&2
      return 1
    fi
    echo "Using $repo's pinned $PLATFORM_DIR binary." >&2
    printf '%s\n' "$SCRIPT_DIR/bin/$wrapper"
    return
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    echo "ERROR: $binary needs a source build, but cargo is unavailable. Install Rust or use a supported pre-built platform." >&2
    return 1
  fi
  echo "Building $binary from its pinned source..." >&2
  if ! (cd "$source_dir" && cargo build --locked --release >&2); then
    echo "ERROR: $binary build failed; an existing binary will not be treated as a successful build." >&2
    return 1
  fi
  if [ ! -x "$built" ]; then
    echo "ERROR: cargo succeeded without producing an executable at $built" >&2
    return 1
  fi
  printf '%s\n' "$built"
}

GITHUB_BIN="$(prepare_binary github kp-github-mcp kp-github-mcp)"
STEPWISE_BIN="$(prepare_binary stepwise kp-stepwise kp-stepwise)"

registration_command() {
  local name="$1" binary="$2" mode="$3"
  local command_args=(claude mcp add "$name" --transport stdio)
  if [ "$name" = "kp-github" ]; then
    if [ -n "${KP_GITHUB_ACCOUNT:-}" ]; then
      command_args+=(--env "KP_GITHUB_ACCOUNT=$KP_GITHUB_ACCOUNT")
    fi
    if [ -n "${KP_GITHUB_EVENTS_CONFIG:-}" ]; then
      command_args+=(--env "KP_GITHUB_EVENTS_CONFIG=$KP_GITHUB_EVENTS_CONFIG")
    fi
  fi
  command_args+=(-- "$binary")
  if [ "$mode" = "print" ]; then
    printf '  '
    printf '%q ' "${command_args[@]}"
    printf '\n'
  else
    "${command_args[@]}"
  fi
}

if [ "$SKIP_REGISTER" = "1" ]; then
  echo "Binaries ready. To register them manually:"
  registration_command kp-github "$GITHUB_BIN" print
  registration_command kp-stepwise "$STEPWISE_BIN" print
  exit 0
fi

register_if_missing() {
  local name="$1" binary="$2"
  if claude mcp get "$name" >/dev/null 2>&1; then
    echo "Existing $name registration preserved, including its account and profile environment."
    echo "Prepared command: $binary"
  else
    registration_command "$name" "$binary" run
  fi
}

register_if_missing kp-github "$GITHUB_BIN"
register_if_missing kp-stepwise "$STEPWISE_BIN"
echo "Restart your MCP host to load the prepared binaries."
