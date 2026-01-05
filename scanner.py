#!/usr/bin/env python3
"""
Compulsion-language scanner for skill files.

Detects patterns that remove agent agency. Flags for human review,
does not auto-reject.

Usage:
    python scanner.py path/to/skill.md
    python scanner.py --dir path/to/skills/
    python scanner.py --check  # Exit 1 if compulsion patterns found (for CI)
"""

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterator


@dataclass
class Finding:
    """A detected hook-language pattern."""
    file: Path
    line_num: int
    line: str
    pattern: str
    severity: str  # "high", "medium", "low"
    suggestion: str


# Patterns that indicate compulsion language
COMPULSION_PATTERNS = [
    # Tier 1: Absolute non-negotiable (high severity)
    {
        "pattern": r"\bnot negotiable\b",
        "severity": "high",
        "suggestion": "Replace with 'strongly recommended' + cost documentation",
    },
    {
        "pattern": r"\bnot optional\b",
        "severity": "high",
        "suggestion": "Replace with 'recommended' + explanation of consequences",
    },
    {
        "pattern": r"\bNO CHOICE\b",
        "severity": "high",
        "suggestion": "Replace with decision framework: options + tradeoffs",
    },
    {
        "pattern": r"\bNO EXCEPTIONS?\b",
        "severity": "high",
        "suggestion": "Document the exceptions that exist and their costs",
    },
    {
        "pattern": r"YOU DO NOT HAVE A CHOICE",
        "severity": "high",
        "suggestion": "Replace with informed consent: 'Here are your options:'",
    },

    # Tier 2: Iron Law framing (high severity)
    {
        "pattern": r"^#+\s*.*Iron Law",
        "severity": "high",
        "suggestion": "Replace 'Iron Law' with 'Iron Principle' + failure mode documentation",
    },
    {
        "pattern": r"Violating the letter.*violating the spirit",
        "severity": "high",
        "suggestion": "Replace with 'The spirit matters more than the letter. Here's why:'",
    },

    # Tier 3: MUST/NEVER without escape (medium severity)
    # These need context - only flag if no escape clause nearby
    {
        "pattern": r"\bMUST\b(?!.*\b(unless|except|if|when|consider)\b)",
        "severity": "medium",
        "suggestion": "Add escape clause or replace with 'should strongly consider'",
    },
    {
        "pattern": r"\bNEVER\b(?!.*\b(unless|except|if|when|rarely)\b)",
        "severity": "medium",
        "suggestion": "Add exception cases or replace with 'avoid' + consequences",
    },
    {
        "pattern": r"\bABSOLUTELY MUST\b",
        "severity": "high",
        "suggestion": "Replace with 'critical for [reason]' + documented costs of skipping",
    },

    # Tier 4: Rationalization blocking (medium severity)
    {
        "pattern": r"cannot rationalize",
        "severity": "medium",
        "suggestion": "Replace with 'watch for these patterns' + why they fail",
    },
    {
        "pattern": r"Delete.*Start over",
        "severity": "medium",
        "suggestion": "Replace with options: 'Delete and restart' OR 'proceed with documented risk'",
    },

    # Tier 5: Required sub-skills (low severity - structure, not compulsion)
    {
        "pattern": r"REQUIRED SUB-SKILL",
        "severity": "low",
        "suggestion": "Replace with 'Recommended next skill' + why the sequence matters",
    },
    {
        "pattern": r"\bForbidden\b",
        "severity": "medium",
        "suggestion": "Replace with 'Anti-pattern' + documented consequences",
    },

    # Bonus: Sycophancy prevention (keep these - they're about honesty, not compulsion)
    # Not flagging: "NEVER say 'You're absolutely right'" - that's about integrity
]


def scan_file(path: Path) -> Iterator[Finding]:
    """Scan a single file for compulsion-language patterns."""
    try:
        content = path.read_text()
    except Exception as e:
        print(f"Warning: Could not read {path}: {e}", file=sys.stderr)
        return

    lines = content.split("\n")

    for i, line in enumerate(lines, 1):
        for hook in COMPULSION_PATTERNS:
            if re.search(hook["pattern"], line, re.IGNORECASE):
                yield Finding(
                    file=path,
                    line_num=i,
                    line=line.strip()[:80],
                    pattern=hook["pattern"],
                    severity=hook["severity"],
                    suggestion=hook["suggestion"],
                )


def scan_directory(path: Path, extensions: tuple = (".md",)) -> Iterator[Finding]:
    """Recursively scan a directory for compulsion language."""
    for file in path.rglob("*"):
        if file.suffix in extensions:
            yield from scan_file(file)


def format_finding(f: Finding, verbose: bool = False) -> str:
    """Format a finding for display."""
    severity_colors = {
        "high": "\033[31m",    # red
        "medium": "\033[33m",  # yellow
        "low": "\033[36m",     # cyan
    }
    reset = "\033[0m"
    color = severity_colors.get(f.severity, "")

    result = f"{color}[{f.severity.upper()}]{reset} {f.file}:{f.line_num}"
    result += f"\n  {f.line}"
    if verbose:
        result += f"\n  Pattern: {f.pattern}"
        result += f"\n  Suggestion: {f.suggestion}"
    return result


def main():
    parser = argparse.ArgumentParser(
        description="Scan skill files for compulsion language"
    )
    parser.add_argument(
        "path",
        type=Path,
        nargs="?",
        default=Path("."),
        help="File or directory to scan",
    )
    parser.add_argument(
        "--dir", "-d",
        type=Path,
        help="Directory to scan recursively",
    )
    parser.add_argument(
        "--check", "-c",
        action="store_true",
        help="Exit with code 1 if high-severity patterns found (for CI)",
    )
    parser.add_argument(
        "--verbose", "-v",
        action="store_true",
        help="Show pattern and suggestion for each finding",
    )
    parser.add_argument(
        "--severity", "-s",
        choices=["high", "medium", "low"],
        default="low",
        help="Minimum severity to report (default: low = all)",
    )
    args = parser.parse_args()

    severity_order = {"high": 3, "medium": 2, "low": 1}
    min_severity = severity_order[args.severity]

    scan_path = args.dir or args.path

    if scan_path.is_file():
        findings = list(scan_file(scan_path))
    else:
        findings = list(scan_directory(scan_path))

    # Filter by severity
    findings = [f for f in findings if severity_order[f.severity] >= min_severity]

    if not findings:
        print("No compulsion language detected.")
        return 0

    # Group by severity
    by_severity = {"high": [], "medium": [], "low": []}
    for f in findings:
        by_severity[f.severity].append(f)

    print(f"\nFound {len(findings)} compulsion pattern(s):\n")

    for severity in ["high", "medium", "low"]:
        if by_severity[severity]:
            print(f"=== {severity.upper()} ({len(by_severity[severity])}) ===\n")
            for f in by_severity[severity]:
                print(format_finding(f, args.verbose))
                print()

    if args.check:
        # For CI: exit 1 if high severity found
        if by_severity["high"]:
            return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
