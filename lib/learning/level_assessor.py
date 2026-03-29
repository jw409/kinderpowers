"""Assess agent progression level (L1-L4) from observed outcomes.

Reads agent_outcomes.jsonl (last 30 days) and classifies the current
capability level based on evidence of skill usage, GSD agents, team
coordination, and self-improvement patterns.

Progression model:
    L1: Coding Assistant  -- uses skills, follows guidance
    L2: Agentic Worker    -- spawns agents, runs GSD phases
    L3: Team Orchestrator -- multi-agent coordination
    L4: Dark Factory      -- self-improving, reads/writes playbook
"""

from __future__ import annotations

import json
import re
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone, timedelta
from pathlib import Path

from .outcome_parser import AgentOutcome, Outcome, parse_outcomes

LEVEL_LABELS: dict[int, str] = {
    1: "Coding Assistant",
    2: "Agentic Worker",
    3: "Team Orchestrator",
    4: "Dark Factory",
}

DEFAULT_LEVEL_PATH = Path.home() / ".kinderpowers" / "level.json"
DEFAULT_PLAYBOOK_PATH = Path.home() / ".kinderpowers" / "playbook.json"

_GSD_PATTERN = re.compile(r"gsd-", re.IGNORECASE)
_L2_KEYWORDS = re.compile(r"\b(plan|execute|phase|verify)\b", re.IGNORECASE)
_L3_KEYWORDS = re.compile(r"\b(team|coordinate|parallel)\b", re.IGNORECASE)
_L4_KEYWORDS = re.compile(r"\b(playbook|learning)\b", re.IGNORECASE)
_SKILL_PATTERN = re.compile(r"skill", re.IGNORECASE)


@dataclass
class LevelAssessment:
    level: int  # 1-4
    label: str  # "Coding Assistant", "Agentic Worker", etc.
    evidence: dict[str, int]  # counts per category
    suggestions: list[str]  # how to reach next level
    assessed_at: str  # ISO8601
    session_count: int  # sessions analyzed
    outcome_count: int  # total outcomes analyzed


def _filter_recent(outcomes: list[AgentOutcome], days: int = 30) -> list[AgentOutcome]:
    """Keep only outcomes from the last N days."""
    cutoff = datetime.now(timezone.utc) - timedelta(days=days)
    recent: list[AgentOutcome] = []
    for o in outcomes:
        if not o.ts:
            continue
        try:
            ts = datetime.fromisoformat(o.ts)
        except (ValueError, TypeError):
            continue
        if ts >= cutoff:
            recent.append(o)
    return recent


def _count_l1_evidence(outcomes: list[AgentOutcome]) -> int:
    """Count L1 evidence: skill invocations and basic tool usage."""
    count = 0
    for o in outcomes:
        if _SKILL_PATTERN.search(o.agent) or _SKILL_PATTERN.search(o.description):
            count += 1
    # Any outcome at all is basic tool usage evidence
    return count + len(outcomes)


def _count_l2_evidence(outcomes: list[AgentOutcome]) -> int:
    """Count L2 evidence: GSD agent spawns and planning keywords."""
    gsd_agents: set[str] = set()
    keyword_hits = 0
    for o in outcomes:
        if _GSD_PATTERN.search(o.agent):
            gsd_agents.add(o.agent)
        if _L2_KEYWORDS.search(o.description):
            keyword_hits += 1
    return len(gsd_agents) + keyword_hits


def _count_l3_evidence(outcomes: list[AgentOutcome]) -> int:
    """Count L3 evidence: team coordination and concurrent agents."""
    keyword_hits = 0
    for o in outcomes:
        if _L3_KEYWORDS.search(o.description):
            keyword_hits += 1

    # Count sessions with 3+ agent spawns (concurrent work)
    by_session: dict[str, int] = defaultdict(int)
    for o in outcomes:
        if o.session_id:
            by_session[o.session_id] += 1

    multi_agent_sessions = sum(
        1 for count in by_session.values() if count >= 3
    )

    return keyword_hits + multi_agent_sessions


def _check_l4_evidence(outcomes: list[AgentOutcome]) -> bool:
    """Check L4 evidence: playbook activity and self-improvement patterns."""
    # Check if playbook file exists and was modified in last 7 days
    playbook_path = DEFAULT_PLAYBOOK_PATH
    playbook_active = False
    if playbook_path.exists():
        try:
            mtime = datetime.fromtimestamp(
                playbook_path.stat().st_mtime, tz=timezone.utc
            )
            if mtime >= datetime.now(timezone.utc) - timedelta(days=7):
                playbook_active = True
        except OSError:
            pass

    if not playbook_active:
        return False

    # Check for self-referential learning patterns in outcome previews
    for o in outcomes:
        preview = getattr(o, "output_preview", "") or ""
        if not preview:
            # AgentOutcome from outcome_parser doesn't have output_preview,
            # fall back to checking description
            preview = o.description
        if _L4_KEYWORDS.search(preview):
            return True

    # Check for same agent type improving over time (success rate trending up)
    by_agent_chronological: dict[str, list[bool]] = defaultdict(list)
    for o in outcomes:
        by_agent_chronological[o.agent].append(o.outcome == Outcome.SUCCESS)

    for _agent, results in by_agent_chronological.items():
        if len(results) < 6:
            continue
        mid = len(results) // 2
        first_half = sum(results[:mid]) / mid if mid else 0
        second_half = sum(results[mid:]) / (len(results) - mid) if (len(results) - mid) else 0
        if second_half > first_half + 0.1:
            return True

    return False


def _suggestions_for_level(level: int, evidence: dict[str, int]) -> list[str]:
    """Generate suggestions for reaching the next level."""
    if level >= 4:
        return ["You are at the highest level. Keep refining the playbook."]

    suggestions: list[str] = []
    if level == 1:
        suggestions.append("Spawn GSD agents (gsd-research, gsd-execute) to reach L2.")
        suggestions.append("Use plan/execute/verify phases in agent descriptions.")
        needed = max(0, 5 - evidence.get("l2_evidence", 0))
        if needed > 0:
            suggestions.append(f"Need {needed} more GSD agent invocations for L2.")
    elif level == 2:
        suggestions.append("Coordinate multiple agents in parallel to reach L3.")
        suggestions.append("Use team orchestration with 3+ agents per session.")
        needed = max(0, 3 - evidence.get("l3_evidence", 0))
        if needed > 0:
            suggestions.append(f"Need {needed} more multi-agent coordination events for L3.")
    elif level == 3:
        suggestions.append("Create and maintain a playbook at ~/.kinderpowers/playbook.json.")
        suggestions.append("Include 'playbook' or 'learning' references in agent outcomes.")
        suggestions.append("Demonstrate improving success rates over time for L4.")

    return suggestions


def assess_level(outcomes: list[AgentOutcome] | None = None) -> LevelAssessment:
    """Assess the current agent progression level from outcome data.

    Args:
        outcomes: Pre-parsed outcomes. If None, reads from default JSONL path
                  and filters to last 30 days.

    Returns:
        LevelAssessment with level, evidence counts, and suggestions.
    """
    if outcomes is None:
        outcomes = parse_outcomes()

    recent = _filter_recent(outcomes, days=30)

    sessions = {o.session_id for o in recent if o.session_id}

    l1_count = _count_l1_evidence(recent)
    l2_count = _count_l2_evidence(recent)
    l3_count = _count_l3_evidence(recent)
    l4_present = _check_l4_evidence(recent)

    evidence = {
        "l1_evidence": l1_count,
        "l2_evidence": l2_count,
        "l3_evidence": l3_count,
        "l4_evidence": 1 if l4_present else 0,
    }

    # Scoring
    level = 1  # default
    if l2_count >= 5:
        level = 2
    if level >= 2 and l3_count >= 3:
        level = 3
    if level >= 3 and l4_present:
        level = 4

    suggestions = _suggestions_for_level(level, evidence)

    return LevelAssessment(
        level=level,
        label=LEVEL_LABELS[level],
        evidence=evidence,
        suggestions=suggestions,
        assessed_at=datetime.now(timezone.utc).isoformat(),
        session_count=len(sessions),
        outcome_count=len(recent),
    )


def write_level(assessment: LevelAssessment, path: Path | None = None) -> None:
    """Write a LevelAssessment to disk as JSON.

    Args:
        assessment: The assessment to persist.
        path: Target file. Defaults to ~/.kinderpowers/level.json.
    """
    target = path or DEFAULT_LEVEL_PATH
    target.parent.mkdir(parents=True, exist_ok=True)

    data = {
        "level": assessment.level,
        "label": assessment.label,
        "evidence": assessment.evidence,
        "suggestions": assessment.suggestions,
        "assessed_at": assessment.assessed_at,
        "session_count": assessment.session_count,
        "outcome_count": assessment.outcome_count,
    }
    target.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def read_level(path: Path | None = None) -> LevelAssessment | None:
    """Read a previously written LevelAssessment from disk.

    Args:
        path: Source file. Defaults to ~/.kinderpowers/level.json.

    Returns:
        LevelAssessment if file exists and is valid, else None.
    """
    target = path or DEFAULT_LEVEL_PATH
    if not target.exists():
        return None

    try:
        data = json.loads(target.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return None

    try:
        return LevelAssessment(
            level=data["level"],
            label=data["label"],
            evidence=data["evidence"],
            suggestions=data["suggestions"],
            assessed_at=data["assessed_at"],
            session_count=data["session_count"],
            outcome_count=data["outcome_count"],
        )
    except (KeyError, TypeError):
        return None
