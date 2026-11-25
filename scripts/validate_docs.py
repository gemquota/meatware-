#!/usr/bin/env python3
"""Validate core Meatware documentation without third-party dependencies."""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = (
    "README.md",
    "docs/SPECIFICATION.md",
    "docs/DEVELOPMENT_PLAN.md",
    "docs/ARCHITECTURE_PRINCIPLES.md",
    "docs/PHASE_1.md",
)
LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")


def validate() -> list[str]:
    errors: list[str] = []
    for relative in REQUIRED:
        path = ROOT / relative
        if not path.is_file():
            errors.append(f"Missing required document: {relative}")
        elif not path.read_text(encoding="utf-8").strip():
            errors.append(f"Empty required document: {relative}")

    readme = ROOT / "README.md"
    if readme.is_file():
        for target in LINK_RE.findall(readme.read_text(encoding="utf-8")):
            if target.startswith(("https://", "http://", "#", "mailto:")):
                continue
            local = (readme.parent / target.split("#", 1)[0]).resolve()
            if not local.exists():
                errors.append(f"Broken README link: {target}")

    spec = ROOT / "docs/SPECIFICATION.md"
    if spec.is_file():
        content = spec.read_text(encoding="utf-8").lower()
        for invariant in (
            "no universal fitness score",
            "analysis and visualisation observe without changing simulation state",
            "no privileged",
        ):
            if invariant not in content:
                errors.append(f"Specification invariant missing: {invariant}")
    return errors


if __name__ == "__main__":
    problems = validate()
    if problems:
        for problem in problems:
            print(f"ERROR: {problem}", file=sys.stderr)
        raise SystemExit(1)
    print(f"Documentation validation passed ({len(REQUIRED)} required documents).")
