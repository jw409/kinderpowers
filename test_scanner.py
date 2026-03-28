#!/usr/bin/env python3
"""Tests for the compulsion-language scanner."""

import tempfile
from pathlib import Path

from scanner import scan_file, scan_directory


def _scan_content(content: str, suffix: str = ".md"):
    """Write content to a temp file, scan it, and clean up automatically."""
    with tempfile.TemporaryDirectory() as d:
        path = Path(d) / f"test{suffix}"
        path.write_text(content)
        return list(scan_file(path))


def test_detects_iron_law():
    findings = _scan_content("## The Iron Law\nDo the thing.\n")
    assert any(f.severity == "high" and "Iron Law" in f.pattern for f in findings)


def test_detects_not_negotiable():
    findings = _scan_content("This is not negotiable.\n")
    assert any(f.severity == "high" for f in findings)


def test_detects_must_without_escape():
    findings = _scan_content("You MUST do this.\n")
    assert any(f.severity == "medium" for f in findings)


def test_allows_must_with_escape():
    findings = _scan_content("You MUST do this unless there's a good reason.\n")
    must_findings = [f for f in findings if "MUST" in f.pattern]
    assert len(must_findings) == 0


def test_detects_never_without_escape():
    findings = _scan_content("NEVER do this.\n")
    assert any(f.severity == "medium" for f in findings)


def test_allows_never_with_escape():
    findings = _scan_content("NEVER do this unless you understand the consequences.\n")
    never_findings = [f for f in findings if "NEVER" in f.pattern]
    assert len(never_findings) == 0


def test_clean_file():
    findings = _scan_content("This is agency-preserving guidance.\nStrongly recommended.\n")
    assert len(findings) == 0


def test_detects_delete_start_over():
    findings = _scan_content("Delete it. Start over.\n")
    assert any(f.severity == "medium" for f in findings)


def test_directory_scan():
    with tempfile.TemporaryDirectory() as d:
        (Path(d) / "clean.md").write_text("Good guidance.\n")
        (Path(d) / "bad.md").write_text("This is not negotiable.\n")
        findings = list(scan_directory(Path(d)))
        assert len(findings) >= 1
        assert any(f.severity == "high" for f in findings)


def test_directory_scan_nested():
    with tempfile.TemporaryDirectory() as d:
        subdir = Path(d) / "level1" / "level2"
        subdir.mkdir(parents=True)
        (subdir / "deep.md").write_text("## The Iron Law\nObey.\n")
        (Path(d) / "top.md").write_text("Good guidance.\n")
        findings = list(scan_directory(Path(d)))
        assert any(f.severity == "high" and "Iron Law" in f.pattern for f in findings)
        deep_findings = [f for f in findings if "deep.md" in str(f.file)]
        assert len(deep_findings) >= 1, "scan_directory must find files in nested subdirectories"


if __name__ == "__main__":
    tests = [v for k, v in globals().items() if k.startswith("test_")]
    for test in tests:
        try:
            test()
            print(f"  PASS: {test.__name__}")
        except AssertionError as e:
            print(f"  FAIL: {test.__name__}: {e}")
    print(f"\n{len(tests)} tests complete.")
