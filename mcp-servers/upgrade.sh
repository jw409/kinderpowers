#!/bin/bash
set -euo pipefail

# Update pinned server code/binaries without replacing MCP account/profile config.
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
KP_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_FROM_SOURCE=0

for arg in "$@"; do
  case "$arg" in
    --build) BUILD_FROM_SOURCE=1 ;;
    --help) echo "Usage: $0 [--build]"; exit 0 ;;
    *) echo "Unknown flag: $arg" >&2; exit 1 ;;
  esac
done

# Prior source installations point directly at target/release. Keep those
# binaries current as well as the packaged wrappers, without reading or
# replacing the MCP host's environment configuration.
if [ -e "$SCRIPT_DIR/github/target/release/kp-github-mcp" ] ||
   [ -e "$SCRIPT_DIR/stepwise/target/release/kp-stepwise" ]; then
  BUILD_FROM_SOURCE=1
fi

echo "Updating kinderpowers with a fast-forward-only pull..."
git -C "$KP_ROOT" pull --ff-only origin main
if [ "$BUILD_FROM_SOURCE" = "1" ]; then
  "$SCRIPT_DIR/install.sh" --no-register --build
else
  "$SCRIPT_DIR/install.sh" --no-register
fi
echo "Pinned binaries prepared; existing MCP registrations were not changed."
echo "Restart your MCP host to load them."
