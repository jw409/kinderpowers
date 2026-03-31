"""Tests for outcome_parser using synthetic JSONL data."""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

import json
import tempfile
from pathlib import Path

from outcome_parser import (
    AgentOutcome,
    Outcome,
    classify_outcome,
    effectiveness_by_agent,
    parse_outcomes,
    summarize_outcomes,
)

# --- synthetic records ---

RECORD_SUCCESS = {
    "ts": "2026-03-28T10:00:00Z",
    "agent": "researcher",
    "model": "sonnet",
    "description": "Research API surface",
    "session_id": "sess-001",
    "bead_id": "meshly-abc",
    "project": "/home/jw/dev/meshly",
    "output_chars": 450,
    "output_preview": "Research completed successfully. Found 12 endpoints.",
    "variant": "default",
}

RECORD_FAILURE_EMPTY = {
    "ts": "2026-03-28T10:05:00Z",
    "agent": "test-runner",
    "model": "haiku",
    "description": "Run unit tests",
    "session_id": "sess-002",
    "bead_id": "meshly-def",
    "project": "/home/jw/dev/meshly",
    "output_chars": 0,
    "output_preview": "",
    "variant": "default",
}

RECORD_FAILURE_ERROR = {
    "ts": "2026-03-28T10:10:00Z",
    "agent": "test-runner",
    "model": "sonnet",
    "description": "Run integration tests",
    "session_id": "sess-003",
    "bead_id": "meshly-ghi",
    "project": "/home/jw/dev/meshly",
    "output_chars": 30,
    "output_preview": "Error: could not connect to DB",
    "variant": "default",
}

RECORD_PARTIAL = {
    "ts": "2026-03-28T10:15:00Z",
    "agent": "implementer",
    "model": "opus",
    "description": "Implement feature X",
    "session_id": "sess-004",
    "bead_id": "meshly-jkl",
    "project": "/home/jw/dev/meshly",
    "output_chars": 800,
    "output_preview": "Implemented 3 of 5 functions. Error: failed to resolve import for util module.",
    "variant": "default",
}

RECORD_SUCCESS_NO_KEYWORD = {
    "ts": "2026-03-28T10:20:00Z",
    "agent": "researcher",
    "model": "sonnet",
    "description": "Scan codebase",
    "session_id": "sess-005",
    "bead_id": "meshly-mno",
    "project": "/home/jw/dev/meshly",
    "output_chars": 200,
    "output_preview": "Found 15 files matching the pattern. Analyzed imports and dependencies.",
    "variant": "default",
}

RECORD_PARTIAL_MIXED = {
    "ts": "2026-03-28T10:25:00Z",
    "agent": "implementer",
    "model": "opus",
    "description": "Refactor auth",
    "session_id": "sess-006",
    "bead_id": "meshly-pqr",
    "project": "/home/jw/dev/meshly",
    "output_chars": 600,
    "output_preview": "Refactoring completed successfully but one test failed due to import error.",
    "variant": "default",
}

RECORD_UNKNOWN_SHORT = {
    "ts": "2026-03-28T10:30:00Z",
    "agent": "planner",
    "model": None,
    "description": "Plan sprint",
    "session_id": "sess-007",
    "bead_id": "meshly-stu",
    "project": "/home/jw/dev/meshly",
    "output_chars": 75,
    "output_preview": "Outline drafted.",
    "variant": "default",
}


ALL_RECORDS = [
    RECORD_SUCCESS,
    RECORD_FAILURE_EMPTY,
    RECORD_FAILURE_ERROR,
    RECORD_PARTIAL,
    RECORD_SUCCESS_NO_KEYWORD,
    RECORD_PARTIAL_MIXED,
    RECORD_UNKNOWN_SHORT,
]


# --- classify_outcome tests ---


def test_classify_success() -> None:
    result = classify_outcome(RECORD_SUCCESS)
    assert result.outcome == Outcome.SUCCESS
    assert result.confidence >= 0.8
    assert result.agent == "researcher"
    assert result.model == "sonnet"


def test_classify_failure_empty_output() -> None:
    result = classify_outcome(RECORD_FAILURE_EMPTY)
    assert result.outcome == Outcome.FAILURE
    assert result.confidence >= 0.9
    assert result.output_chars == 0


def test_classify_failure_error_text() -> None:
    result = classify_outcome(RECORD_FAILURE_ERROR)
    assert result.outcome == Outcome.FAILURE
    assert result.confidence >= 0.8


def test_classify_partial_mixed_signals() -> None:
    result = classify_outcome(RECORD_PARTIAL)
    assert result.outcome == Outcome.PARTIAL
    assert result.confidence >= 0.5


def test_classify_success_no_keyword() -> None:
    """Output > 100 chars and no failure signals should still be SUCCESS."""
    result = classify_outcome(RECORD_SUCCESS_NO_KEYWORD)
    assert result.outcome == Outcome.SUCCESS
    assert result.confidence >= 0.7


def test_classify_partial_both_signals() -> None:
    """Both success and failure keywords present -> PARTIAL."""
    result = classify_outcome(RECORD_PARTIAL_MIXED)
    assert result.outcome == Outcome.PARTIAL


def test_classify_unknown_ambiguous() -> None:
    """Short output, no strong signals either way."""
    result = classify_outcome(RECORD_UNKNOWN_SHORT)
    assert result.outcome == Outcome.UNKNOWN
    assert result.confidence < 0.6


# --- parse_outcomes tests ---


def _write_jsonl(records: list[dict], path: Path) -> None:
    with open(path, "w", encoding="utf-8") as f:
        for r in records:
            f.write(json.dumps(r) + "\n")


def test_parse_outcomes_from_file() -> None:
    with tempfile.TemporaryDirectory() as td:
        p = Path(td) / "outcomes.jsonl"
        _write_jsonl(ALL_RECORDS, p)
        outcomes = parse_outcomes(p)
        assert len(outcomes) == len(ALL_RECORDS)
        assert all(isinstance(o, AgentOutcome) for o in outcomes)


def test_parse_outcomes_missing_file() -> None:
    outcomes = parse_outcomes(Path("/tmp/nonexistent_outcomes_abc123.jsonl"))
    assert outcomes == []


def test_parse_outcomes_skips_malformed() -> None:
    with tempfile.TemporaryDirectory() as td:
        p = Path(td) / "outcomes.jsonl"
        with open(p, "w") as f:
            f.write(json.dumps(RECORD_SUCCESS) + "\n")
            f.write("NOT VALID JSON\n")
            f.write(json.dumps(RECORD_FAILURE_EMPTY) + "\n")
            f.write("\n")  # blank line
        outcomes = parse_outcomes(p)
        assert len(outcomes) == 2


# --- summarize_outcomes tests ---


def test_summarize_outcomes_empty() -> None:
    summary = summarize_outcomes([])
    assert summary["total"] == 0
    assert summary["success_rate"] == 0.0


def test_summarize_outcomes_stats() -> None:
    with tempfile.TemporaryDirectory() as td:
        p = Path(td) / "outcomes.jsonl"
        _write_jsonl(ALL_RECORDS, p)
        outcomes = parse_outcomes(p)
        summary = summarize_outcomes(outcomes)

        assert summary["total"] == 7
        assert "success" in summary["by_outcome"]
        assert "failure" in summary["by_outcome"]
        assert 0.0 <= summary["success_rate"] <= 1.0

        # by_agent should have researcher, test-runner, implementer, planner
        assert "researcher" in summary["by_agent"]
        assert "test-runner" in summary["by_agent"]

        # by_model should have sonnet, haiku, opus, unknown
        assert "sonnet" in summary["by_model"]
        assert "opus" in summary["by_model"]


def test_summarize_by_agent_rates() -> None:
    with tempfile.TemporaryDirectory() as td:
        p = Path(td) / "outcomes.jsonl"
        _write_jsonl(ALL_RECORDS, p)
        outcomes = parse_outcomes(p)
        summary = summarize_outcomes(outcomes)

        # researcher: 2 success out of 2
        researcher = summary["by_agent"]["researcher"]
        assert researcher["success_rate"] == 1.0

        # test-runner: 0 success out of 2
        runner = summary["by_agent"]["test-runner"]
        assert runner["success_rate"] == 0.0


# --- effectiveness_by_agent tests ---


def test_effectiveness_by_agent() -> None:
    with tempfile.TemporaryDirectory() as td:
        p = Path(td) / "outcomes.jsonl"
        _write_jsonl(ALL_RECORDS, p)
        outcomes = parse_outcomes(p)
        eff = effectiveness_by_agent(outcomes)

        assert eff["researcher"] == 1.0
        assert eff["test-runner"] == 0.0
        assert 0.0 <= eff["implementer"] <= 1.0
        assert "planner" in eff


def test_effectiveness_empty() -> None:
    eff = effectiveness_by_agent([])
    assert eff == {}


if __name__ == "__main__":
    import sys

    # Simple test runner
    test_funcs = [v for k, v in globals().items() if k.startswith("test_") and callable(v)]
    failed = 0
    for fn in test_funcs:
        try:
            fn()
            print(f"  PASS  {fn.__name__}")
        except Exception as e:
            print(f"  FAIL  {fn.__name__}: {e}")
            failed += 1
    print(f"\n{len(test_funcs)} tests, {failed} failed")
    sys.exit(1 if failed else 0)
