#!/usr/bin/env python3
"""Tests for the compulsion-language scanner."""

import tempfile
from pathlib import Path

from scanner import Finding, scan_file, scan_directory


def _write_temp(content: str, suffix: str = ".md") -> Path:
    """Write content to a temp file and return its path."""
    f = tempfile.NamedTemporaryFile(mode="w", suffix=suffix, delete=False)
    f.write(content)
    f.close()
    return Path(f.name)


def test_detects_iron_law():
    path = _write_temp("## The Iron Law\nDo the thing.\n")
    findings = list(scan_file(path))
    assert any(f.severity == "high" and "Iron Law" in f.pattern for f in findings)


def test_detects_not_negotiable():
    path = _write_temp("This is not negotiable.\n")
    findings = list(scan_file(path))
    assert any(f.severity == "high" for f in findings)


def test_detects_must_without_escape():
    path = _write_temp("You MUST do this.\n")
    findings = list(scan_file(path))
    assert any(f.severity == "medium" for f in findings)


def test_allows_must_with_escape():
    path = _write_temp("You MUST do this unless there's a good reason.\n")
    findings = list(scan_file(path))
    must_findings = [f for f in findings if "MUST" in f.pattern]
    assert len(must_findings) == 0


def test_detects_never_without_escape():
    path = _write_temp("NEVER do this.\n")
    findings = list(scan_file(path))
    assert any(f.severity == "medium" for f in findings)


def test_allows_never_with_escape():
    path = _write_temp("NEVER do this unless you understand the consequences.\n")
    findings = list(scan_file(path))
    never_findings = [f for f in findings if "NEVER" in f.pattern]
    assert len(never_findings) == 0


def test_clean_file():
    path = _write_temp("This is agency-preserving guidance.\nStrongly recommended.\n")
    findings = list(scan_file(path))
    assert len(findings) == 0


def test_detects_delete_start_over():
    path = _write_temp("Delete it. Start over.\n")
    findings = list(scan_file(path))
    assert any(f.severity == "medium" for f in findings)


def test_directory_scan():
    with tempfile.TemporaryDirectory() as d:
        (Path(d) / "clean.md").write_text("Good guidance.\n")
        (Path(d) / "bad.md").write_text("This is not negotiable.\n")
        findings = list(scan_directory(Path(d)))
        assert len(findings) >= 1
        assert any(f.severity == "high" for f in findings)


if __name__ == "__main__":
    tests = [v for k, v in globals().items() if k.startswith("test_")]
    for test in tests:
        try:
            test()
            print(f"  PASS: {test.__name__}")
        except AssertionError as e:
            print(f"  FAIL: {test.__name__}: {e}")
    print(f"\n{len(tests)} tests complete.")
