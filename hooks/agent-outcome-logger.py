#!/usr/bin/env python3
"""PostToolUse hook: logs structured outcomes when Agent tool completes.

Appends JSONL to ~/.kinderpowers/agent_outcomes.jsonl for experiment tracking.
Variant tagging via `variant:` marker in prompt enables A/B comparison.
"""

import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

MAX_FILE_BYTES = 10 * 1024 * 1024  # 10 MB


def _rotate_if_needed(out_file: Path) -> None:
    """Rotate log file if it exceeds MAX_FILE_BYTES. Never raises."""
    try:
        if out_file.exists() and out_file.stat().st_size > MAX_FILE_BYTES:
            rotated = out_file.with_suffix(".jsonl.1")
            out_file.replace(rotated)
    except Exception as exc:
        print(f"agent-outcome-logger: rotation failed: {exc}", file=sys.stderr)


def main():
    try:
        hook_input = json.load(sys.stdin)
    except (json.JSONDecodeError, EOFError):
        return

    tool_name = hook_input.get("tool_name", "")
    if tool_name != "Agent":
        return

    tool_input = hook_input.get("tool_input", {})
    tool_result = hook_input.get("tool_response", hook_input.get("tool_result", ""))

    # Extract variant from prompt if tagged
    prompt = tool_input.get("prompt", "")
    variant = "default"
    for line in prompt.splitlines():
        stripped = line.strip().lower()
        if stripped.startswith("variant:"):
            variant = stripped.split(":", 1)[1].strip()
            break

    # Truncate result for storage
    if isinstance(tool_result, dict):
        result_text = json.dumps(tool_result)
    else:
        result_text = str(tool_result) if tool_result else ""
    output_chars = len(result_text)
    output_preview = result_text[:500]

    record = {
        "ts": datetime.now(timezone.utc).isoformat(),
        "agent": tool_input.get("subagent_type", tool_input.get("name", "unknown")),
        "model": tool_input.get("model"),
        "description": tool_input.get("description", ""),
        "session_id": os.environ.get("CLAUDE_SESSION_ID", hook_input.get("session_id", "")),
        "bead_id": os.environ.get("CLAUDE_BEAD_ID", ""),
        "project": os.environ.get("CLAUDE_PROJECT_DIR", os.getcwd()),
        "output_chars": output_chars,
        "output_preview": output_preview,
        "variant": variant,
    }

    out_dir = Path.home() / ".kinderpowers"
    out_dir.mkdir(parents=True, exist_ok=True)
    out_file = out_dir / "agent_outcomes.jsonl"

    _rotate_if_needed(out_file)

    with open(out_file, "a") as f:
        f.write(json.dumps(record, separators=(",", ":")) + "\n")


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:
        print(f"agent-outcome-logger: unexpected error: {exc}", file=sys.stderr)
        sys.exit(0)
