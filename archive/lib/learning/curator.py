"""Delta-based playbook management for kinderpowers.

Maintains a playbook at ~/.kinderpowers/playbook.md with learned patterns
about agent and model effectiveness. Uses delta operations (ADD, UPDATE,
MERGE, PRUNE) to evolve the playbook over time based on agent outcome data.

Ported from talent-os teacher/curator.py patterns.
"""

from __future__ import annotations

import json
import re
from collections import defaultdict
from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum
from pathlib import Path
from typing import Any

from .outcome_parser import AgentOutcome, Outcome


# ---------------------------------------------------------------------------
# Data structures
# ---------------------------------------------------------------------------


class DeltaOp(Enum):
    ADD = "add"
    UPDATE = "update"
    MERGE = "merge"
    PRUNE = "prune"


class Section(Enum):
    EFFECTIVE = "effective_patterns"
    ANTI = "anti_patterns"
    MODEL = "model_selection"


@dataclass
class Bullet:
    id: str  # EP-001, AP-001, MS-001
    content: str
    success_count: int = 0
    failure_count: int = 0
    confidence: float = 0.5
    evidence_refs: list[str] = field(default_factory=list)

    @property
    def net_score(self) -> int:
        return self.success_count - self.failure_count


@dataclass
class Delta:
    op: DeltaOp
    section: Section
    bullet: Bullet
    reason: str
    timestamp: str  # ISO8601


# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

DEFAULT_PLAYBOOK_PATH = Path.home() / ".kinderpowers" / "playbook.md"
DEFAULT_AUDIT_PATH = Path.home() / ".kinderpowers" / "playbook_audit.jsonl"

# ---------------------------------------------------------------------------
# Playbook parsing
# ---------------------------------------------------------------------------

# Section headers in the markdown, mapped to Section enum values.
_SECTION_HEADERS: dict[str, Section] = {
    "## Effective Patterns": Section.EFFECTIVE,
    "## Anti-Patterns": Section.ANTI,
    "## Model Selection": Section.MODEL,
}

# Regex for a bullet line: - **[EP-001]** content (optional parenthetical stats)
_BULLET_RE = re.compile(
    r"^- \*\*\[(?P<id>[A-Z]{2}-\d{3})\]\*\*\s+(?P<content>.+)$"
)

# Regex to extract inline stats like "success rate: 85%, 20 invocations"
_STATS_RE = re.compile(
    r"\("
    r"(?:success rate:\s*(?P<success_rate>\d+)%"
    r"(?:,\s*(?P<invocations>\d+)\s*invocations?)?)?"
    r"(?:failure rate:\s*(?P<failure_rate>\d+)%"
    r"(?:,\s*(?P<fail_invocations>\d+)\s*invocations?)?)?"
    r"(?:cost efficiency:\s*(?P<cost>\d+)x)?"
    r".*?\)$"
)

# Simpler stat patterns for individual capture
_SUCCESS_RATE_RE = re.compile(r"success rate:\s*(\d+)%")
_FAILURE_RATE_RE = re.compile(r"failure rate:\s*(\d+)%")
_INVOCATIONS_RE = re.compile(r"(\d+)\s*invocations?")
_SUCCESS_PAREN_RE = re.compile(r"(\d+)%\s*success")


def _parse_bullet(line: str) -> Bullet | None:
    """Parse a single markdown bullet line into a Bullet, or None."""
    m = _BULLET_RE.match(line.strip())
    if not m:
        return None

    bullet_id = m.group("id")
    content = m.group("content")

    # Try to extract counts from inline stats
    success_count = 0
    failure_count = 0
    confidence = 0.5

    sr = _SUCCESS_RATE_RE.search(content)
    fr = _FAILURE_RATE_RE.search(content)
    inv = _INVOCATIONS_RE.search(content)
    sp = _SUCCESS_PAREN_RE.search(content)

    invocations = int(inv.group(1)) if inv else 0

    if sr:
        rate = int(sr.group(1)) / 100.0
        if invocations:
            success_count = round(rate * invocations)
            failure_count = invocations - success_count
        confidence = rate
    elif fr:
        rate = int(fr.group(1)) / 100.0
        if invocations:
            failure_count = round(rate * invocations)
            success_count = invocations - failure_count
        confidence = 1.0 - rate
    elif sp:
        rate = int(sp.group(1)) / 100.0
        confidence = rate

    return Bullet(
        id=bullet_id,
        content=content,
        success_count=success_count,
        failure_count=failure_count,
        confidence=confidence,
    )


def load_playbook(path: Path | None = None) -> dict[Section, list[Bullet]]:
    """Parse playbook.md into structured data.

    Args:
        path: Path to playbook markdown. Defaults to ~/.kinderpowers/playbook.md.

    Returns:
        Dict mapping Section to list of Bullet entries. Missing sections
        are returned as empty lists.
    """
    target = path or DEFAULT_PLAYBOOK_PATH
    playbook: dict[Section, list[Bullet]] = {s: [] for s in Section}

    if not target.exists():
        return playbook

    current_section: Section | None = None
    with open(target, "r", encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()

            # Check for section header
            for header, section in _SECTION_HEADERS.items():
                if stripped.startswith(header):
                    current_section = section
                    break

            if current_section is None:
                continue

            # Try to parse as a bullet
            bullet = _parse_bullet(stripped)
            if bullet:
                playbook[current_section].append(bullet)

    return playbook


# ---------------------------------------------------------------------------
# Delta application
# ---------------------------------------------------------------------------

def _next_id(section: Section, bullets: list[Bullet]) -> str:
    """Generate the next sequential ID for a section."""
    prefix_map = {
        Section.EFFECTIVE: "EP",
        Section.ANTI: "AP",
        Section.MODEL: "MS",
    }
    prefix = prefix_map[section]
    existing_nums = []
    for b in bullets:
        if b.id.startswith(prefix + "-"):
            try:
                existing_nums.append(int(b.id.split("-")[1]))
            except (ValueError, IndexError):
                pass
    next_num = max(existing_nums, default=0) + 1
    return f"{prefix}-{next_num:03d}"


def apply_delta(playbook: dict[Section, list[Bullet]], delta: Delta) -> dict[Section, list[Bullet]]:
    """Apply a single delta operation to the playbook.

    Args:
        playbook: Current playbook state (mutated in place AND returned).
        delta: The delta to apply.

    Returns:
        The updated playbook dict.
    """
    section = delta.section
    bullets = playbook[section]

    if delta.op == DeltaOp.ADD:
        # Check for duplicate ID
        existing_ids = {b.id for b in bullets}
        if delta.bullet.id in existing_ids:
            # Treat as update instead
            for i, b in enumerate(bullets):
                if b.id == delta.bullet.id:
                    bullets[i] = delta.bullet
                    break
        else:
            bullets.append(delta.bullet)

    elif delta.op == DeltaOp.UPDATE:
        for i, b in enumerate(bullets):
            if b.id == delta.bullet.id:
                bullets[i] = delta.bullet
                break

    elif delta.op == DeltaOp.MERGE:
        # Merge counts and evidence into an existing bullet, keep higher confidence.
        for i, b in enumerate(bullets):
            if b.id == delta.bullet.id:
                b.success_count += delta.bullet.success_count
                b.failure_count += delta.bullet.failure_count
                b.confidence = max(b.confidence, delta.bullet.confidence)
                b.evidence_refs.extend(delta.bullet.evidence_refs)
                # Update content if the delta provides richer text
                if len(delta.bullet.content) > len(b.content):
                    b.content = delta.bullet.content
                break
        else:
            # No existing bullet to merge into — treat as ADD
            bullets.append(delta.bullet)

    elif delta.op == DeltaOp.PRUNE:
        playbook[section] = [b for b in bullets if b.id != delta.bullet.id]

    return playbook


# ---------------------------------------------------------------------------
# Delta generation from outcome data
# ---------------------------------------------------------------------------

# Thresholds
_EFFECTIVE_THRESHOLD = 0.75  # >75% success → effective pattern
_ANTI_THRESHOLD = 0.40       # <40% success → anti-pattern
_MIN_SAMPLES = 3             # Minimum invocations before generating a delta
_PRUNE_DECLINE_THRESHOLD = 0.20  # 20% drop in effectiveness → prune candidate


def generate_deltas(outcomes: list[AgentOutcome]) -> list[Delta]:
    """Analyze outcomes and produce deltas for the playbook.

    Rules:
    - Agent types with >75% success rate (min 3 samples) → ADD to effective_patterns
    - Agent types with <40% success rate (min 3 samples) → ADD to anti_patterns
    - Model-specific success rates (min 3 samples) → ADD/UPDATE model_selection
    - Existing bullets with declining effectiveness → PRUNE

    Args:
        outcomes: List of classified agent outcomes.

    Returns:
        List of Delta operations to apply.
    """
    if not outcomes:
        return []

    now = datetime.now(timezone.utc).isoformat()
    deltas: list[Delta] = []

    # --- Aggregate by agent ---
    agent_stats: dict[str, dict[str, int]] = defaultdict(lambda: {"success": 0, "failure": 0, "total": 0})
    for o in outcomes:
        bucket = agent_stats[o.agent]
        bucket["total"] += 1
        if o.outcome == Outcome.SUCCESS:
            bucket["success"] += 1
        elif o.outcome == Outcome.FAILURE:
            bucket["failure"] += 1

    for agent, stats in agent_stats.items():
        if stats["total"] < _MIN_SAMPLES:
            continue
        rate = stats["success"] / stats["total"]
        total = stats["total"]
        session_refs = list({o.session_id for o in outcomes if o.agent == agent and o.session_id})[:5]

        if rate >= _EFFECTIVE_THRESHOLD:
            bullet = Bullet(
                id="EP-000",  # placeholder — caller should assign via _next_id
                content=f"Use `{agent}` agent (success rate: {round(rate * 100)}%, {total} invocations)",
                success_count=stats["success"],
                failure_count=stats["failure"],
                confidence=rate,
                evidence_refs=session_refs,
            )
            deltas.append(Delta(
                op=DeltaOp.ADD,
                section=Section.EFFECTIVE,
                bullet=bullet,
                reason=f"{agent} has {round(rate * 100)}% success rate across {total} invocations",
                timestamp=now,
            ))
        elif rate <= _ANTI_THRESHOLD:
            failure_rate = 1.0 - rate
            bullet = Bullet(
                id="AP-000",  # placeholder
                content=f"Avoid `{agent}` agent for general use (failure rate: {round(failure_rate * 100)}%, {total} invocations)",
                success_count=stats["success"],
                failure_count=stats["failure"],
                confidence=failure_rate,
                evidence_refs=session_refs,
            )
            deltas.append(Delta(
                op=DeltaOp.ADD,
                section=Section.ANTI,
                bullet=bullet,
                reason=f"{agent} has {round(failure_rate * 100)}% failure rate across {total} invocations",
                timestamp=now,
            ))

    # --- Aggregate by model ---
    model_stats: dict[str, dict[str, int]] = defaultdict(lambda: {"success": 0, "failure": 0, "total": 0})
    for o in outcomes:
        model_key = o.model or "unknown"
        bucket = model_stats[model_key]
        bucket["total"] += 1
        if o.outcome == Outcome.SUCCESS:
            bucket["success"] += 1
        elif o.outcome == Outcome.FAILURE:
            bucket["failure"] += 1

    # Determine primary task type per model (by most common agent description)
    model_tasks: dict[str, str] = {}
    for o in outcomes:
        model_key = o.model or "unknown"
        if model_key not in model_tasks and o.description:
            model_tasks[model_key] = o.description

    for model, stats in model_stats.items():
        if stats["total"] < _MIN_SAMPLES or model == "unknown":
            continue
        rate = stats["success"] / stats["total"]
        total = stats["total"]
        task_hint = model_tasks.get(model, "general tasks")

        bullet = Bullet(
            id="MS-000",  # placeholder
            content=f"{model}: {task_hint} ({round(rate * 100)}% success)",
            success_count=stats["success"],
            failure_count=stats["failure"],
            confidence=rate,
        )
        deltas.append(Delta(
            op=DeltaOp.ADD,
            section=Section.MODEL,
            bullet=bullet,
            reason=f"{model} shows {round(rate * 100)}% success across {total} invocations",
            timestamp=now,
        ))

    # --- Assign sequential IDs ---
    # Group deltas by section so IDs don't collide.
    section_counters: dict[Section, int] = {s: 0 for s in Section}
    prefix_map = {
        Section.EFFECTIVE: "EP",
        Section.ANTI: "AP",
        Section.MODEL: "MS",
    }
    for d in deltas:
        section_counters[d.section] += 1
        prefix = prefix_map[d.section]
        d.bullet.id = f"{prefix}-{section_counters[d.section]:03d}"

    return deltas


# ---------------------------------------------------------------------------
# Playbook rendering
# ---------------------------------------------------------------------------

def _render_bullet(bullet: Bullet) -> str:
    """Render a Bullet back to a markdown line."""
    return f"- **[{bullet.id}]** {bullet.content}"


def write_playbook(playbook: dict[Section, list[Bullet]], path: Path | None = None) -> None:
    """Render playbook dict back to markdown file.

    Args:
        playbook: The playbook data structure.
        path: Output path. Defaults to ~/.kinderpowers/playbook.md.
    """
    target = path or DEFAULT_PLAYBOOK_PATH
    target.parent.mkdir(parents=True, exist_ok=True)

    now = datetime.now(timezone.utc).isoformat()
    lines: list[str] = [
        "# Kinderpowers Playbook",
        "<!-- Auto-generated by learning pipeline. Do not edit manually. -->",
        f"<!-- Last updated: {now} -->",
        "",
    ]

    # Effective Patterns
    lines.append("## Effective Patterns")
    lines.append("<!-- Patterns with net positive outcomes -->")
    lines.append("")
    for bullet in playbook.get(Section.EFFECTIVE, []):
        lines.append(_render_bullet(bullet))
    lines.append("")

    # Anti-Patterns
    lines.append("## Anti-Patterns")
    lines.append("<!-- Patterns with net negative outcomes -->")
    lines.append("")
    for bullet in playbook.get(Section.ANTI, []):
        lines.append(_render_bullet(bullet))
    lines.append("")

    # Model Selection
    lines.append("## Model Selection")
    lines.append("<!-- Which model works best for which task type -->")
    lines.append("")
    for bullet in playbook.get(Section.MODEL, []):
        lines.append(_render_bullet(bullet))
    lines.append("")

    target.write_text("\n".join(lines), encoding="utf-8")


# ---------------------------------------------------------------------------
# Audit log
# ---------------------------------------------------------------------------

def _delta_to_dict(delta: Delta) -> dict[str, Any]:
    """Serialize a Delta to a JSON-safe dict."""
    return {
        "op": delta.op.value,
        "section": delta.section.value,
        "bullet_id": delta.bullet.id,
        "bullet_content": delta.bullet.content,
        "success_count": delta.bullet.success_count,
        "failure_count": delta.bullet.failure_count,
        "confidence": delta.bullet.confidence,
        "evidence_refs": delta.bullet.evidence_refs,
        "reason": delta.reason,
        "timestamp": delta.timestamp,
    }


def append_audit(delta: Delta, path: Path | None = None) -> None:
    """Append a delta record to the playbook audit log.

    Args:
        delta: The delta operation to log.
        path: Audit log path. Defaults to ~/.kinderpowers/playbook_audit.jsonl.
    """
    target = path or DEFAULT_AUDIT_PATH
    target.parent.mkdir(parents=True, exist_ok=True)

    record = _delta_to_dict(delta)
    with open(target, "a", encoding="utf-8") as f:
        f.write(json.dumps(record, separators=(",", ":")) + "\n")
