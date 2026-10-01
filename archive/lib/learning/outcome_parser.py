"""Outcome parser for agent invocations.

Reads agent_outcomes.jsonl (produced by hooks/agent-outcome-logger.py)
and classifies each invocation as SUCCESS, FAILURE, or PARTIAL.

Ported from talent-os scavenger/outcome_parser.py patterns.
"""

from __future__ import annotations

import json
import re
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Any


class Outcome(Enum):
    SUCCESS = "success"
    FAILURE = "failure"
    PARTIAL = "partial"
    UNKNOWN = "unknown"


@dataclass
class AgentOutcome:
    ts: str
    agent: str
    model: str | None
    description: str
    outcome: Outcome
    confidence: float  # 0.0-1.0
    session_id: str
    output_chars: int


# --- signal patterns ---

_FAILURE_PATTERNS: list[re.Pattern[str]] = [
    re.compile(r"\berror\b", re.IGNORECASE),
    re.compile(r"\bfailed\b", re.IGNORECASE),
    re.compile(r"Exception:", re.IGNORECASE),
    re.compile(r"Traceback", re.IGNORECASE),
    re.compile(r"\bcould not\b", re.IGNORECASE),
]

_SUCCESS_PATTERNS: list[re.Pattern[str]] = [
    re.compile(r"\bcompleted\b", re.IGNORECASE),
    re.compile(r"\bdone\b", re.IGNORECASE),
    re.compile(r"\bpass\b", re.IGNORECASE),
    re.compile(r"\bsuccess\b", re.IGNORECASE),
]

DEFAULT_OUTCOMES_PATH = Path.home() / ".kinderpowers" / "agent_outcomes.jsonl"


def _has_failure_signals(preview: str) -> bool:
    return any(p.search(preview) for p in _FAILURE_PATTERNS)


def _has_success_signals(preview: str) -> bool:
    return any(p.search(preview) for p in _SUCCESS_PATTERNS)


def classify_outcome(record: dict[str, Any]) -> AgentOutcome:
    """Classify a single JSONL record into an AgentOutcome."""
    output_chars: int = record.get("output_chars", 0)
    preview: str = record.get("output_preview", "") or ""

    has_fail = _has_failure_signals(preview)
    has_success = _has_success_signals(preview)

    # Decision tree
    if output_chars == 0:
        outcome = Outcome.FAILURE
        confidence = 0.95
    elif has_fail and not has_success:
        if output_chars < 50:
            outcome = Outcome.FAILURE
            confidence = 0.9
        else:
            # Substantial output but contains error language
            outcome = Outcome.PARTIAL
            confidence = 0.7
    elif has_fail and has_success:
        # Mixed signals
        outcome = Outcome.PARTIAL
        confidence = 0.6
    elif output_chars > 100 and has_success:
        outcome = Outcome.SUCCESS
        confidence = 0.9
    elif output_chars > 100 and not has_fail:
        outcome = Outcome.SUCCESS
        confidence = 0.75
    elif output_chars < 50 and not has_success:
        outcome = Outcome.FAILURE
        confidence = 0.65
    else:
        # 50-100 chars, no strong signals either way
        outcome = Outcome.UNKNOWN
        confidence = 0.4

    return AgentOutcome(
        ts=record.get("ts", ""),
        agent=record.get("agent", "unknown"),
        model=record.get("model"),
        description=record.get("description", ""),
        outcome=outcome,
        confidence=confidence,
        session_id=record.get("session_id", ""),
        output_chars=output_chars,
    )


def parse_outcomes(path: Path | None = None) -> list[AgentOutcome]:
    """Read and classify all records from an agent_outcomes.jsonl file.

    Args:
        path: Path to the JSONL file. Defaults to ~/.kinderpowers/agent_outcomes.jsonl.

    Returns:
        List of classified AgentOutcome records. Malformed lines are skipped.
    """
    target = path or DEFAULT_OUTCOMES_PATH
    if not target.exists():
        return []

    outcomes: list[AgentOutcome] = []
    with open(target, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                record = json.loads(line)
            except json.JSONDecodeError:
                continue
            outcomes.append(classify_outcome(record))
    return outcomes


def summarize_outcomes(outcomes: list[AgentOutcome]) -> dict[str, Any]:
    """Aggregate outcome stats by agent type, model, and project.

    Returns a dict with keys:
        total: int
        by_outcome: {outcome_name: count}
        success_rate: float
        by_agent: {agent: {outcome_name: count, success_rate: float}}
        by_model: {model: {outcome_name: count, success_rate: float}}
    """
    if not outcomes:
        return {
            "total": 0,
            "by_outcome": {},
            "success_rate": 0.0,
            "by_agent": {},
            "by_model": {},
        }

    by_outcome: dict[str, int] = defaultdict(int)
    by_agent: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))
    by_model: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))

    for o in outcomes:
        by_outcome[o.outcome.value] += 1
        by_agent[o.agent][o.outcome.value] += 1
        model_key = o.model or "unknown"
        by_model[model_key][o.outcome.value] += 1

    total = len(outcomes)
    success_count = by_outcome.get("success", 0)

    def _rate(bucket: dict[str, int]) -> dict[str, Any]:
        n = sum(bucket.values())
        s = bucket.get("success", 0)
        return {**dict(bucket), "success_rate": round(s / n, 4) if n else 0.0}

    return {
        "total": total,
        "by_outcome": dict(by_outcome),
        "success_rate": round(success_count / total, 4) if total else 0.0,
        "by_agent": {k: _rate(v) for k, v in by_agent.items()},
        "by_model": {k: _rate(v) for k, v in by_model.items()},
    }


def effectiveness_by_agent(outcomes: list[AgentOutcome]) -> dict[str, float]:
    """Return success rate per agent type.

    Returns:
        Dict mapping agent name to success rate (0.0-1.0).
    """
    counts: dict[str, list[int]] = defaultdict(lambda: [0, 0])  # [success, total]
    for o in outcomes:
        counts[o.agent][1] += 1
        if o.outcome == Outcome.SUCCESS:
            counts[o.agent][0] += 1
    return {
        agent: round(pair[0] / pair[1], 4) if pair[1] else 0.0
        for agent, pair in counts.items()
    }
