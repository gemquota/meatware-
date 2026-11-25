#!/usr/bin/env python3
"""Validate core Meatware documentation and model-contract coverage."""
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
    "docs/PHASE_2.md",
    "docs/SUBSTRATE_MODEL.md",
    "docs/TRANSITION_CONTRACT.md",
    "docs/DECISIONS_PENDING.md",
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

    model = ROOT / "docs/SUBSTRATE_MODEL.md"
    if model.is_file():
        content = model.read_text(encoding="utf-8").lower()
        for contract in (
            "unique, opaque identifier",
            "relation-to-relation",
            "missing property and property set to null are distinct",
            "atomic at the configuration level",
            "read-only",
            "no privileged",
        ):
            if contract not in content:
                errors.append(f"Substrate contract missing: {contract}")

    transitions = ROOT / "docs/TRANSITION_CONTRACT.md"
    if transitions.is_file():
        content = transitions.read_text(encoding="utf-8").lower()
        for contract in (
            "write/write",
            "write/remove",
            "retire/write",
            "atomic",
            "last-write-wins",
            "deterministic",
        ):
            if contract not in content:
                errors.append(f"Transition contract missing: {contract}")

    fixture = ROOT / "tests/fixtures/transition_cases.json"
    if fixture.is_file():
        import json
        try:
            cases = json.loads(fixture.read_text(encoding="utf-8"))
            if len(cases.get("cases", [])) < 10:
                errors.append("Transition conformance fixture has fewer than 10 cases")
        except (json.JSONDecodeError, AttributeError):
            errors.append("Transition conformance fixture is not valid JSON")
    else:
        errors.append("Missing transition conformance fixture")

    return errors


if __name__ == "__main__":
    problems = validate()
    if problems:
        for problem in problems:
            print(f"ERROR: {problem}", file=sys.stderr)
        raise SystemExit(1)
    print(f"Documentation validation passed ({len(REQUIRED)} required documents and model contracts).")
