#!/usr/bin/env python3
"""Validate the repository's machine-readable project structure and local Markdown links."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
LINK_RE = re.compile(r"!?(?:\[[^\]]*\])\((<[^>]+>|[^)]+)\)")
REQUIRED_CHARACTERS = {"Tyler", "Marcus", "Dave", "Chad"}


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def read_json(path: Path, errors: list[str]):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(errors, f"Missing required file: {path.relative_to(ROOT)}")
    except json.JSONDecodeError as exc:
        fail(errors, f"Invalid JSON in {path.relative_to(ROOT)}: {exc}")
    return None


def check_repo_path(value: str, label: str, errors: list[str]) -> None:
    candidate = ROOT / value
    if not candidate.is_file():
        fail(errors, f"{label} points to missing file: {value}")


def validate_json_contracts(errors: list[str]) -> None:
    canon_path = ROOT / "projects/prompt-and-circumstance/canon/canon.json"
    manifest_path = ROOT / ".well-known/agent-manifest.json"
    credits_path = ROOT / "projects/prompt-and-circumstance/contributors/CREDITS.json"

    canon = read_json(canon_path, errors)
    if isinstance(canon, dict):
        franchise = canon.get("franchise")
        if not isinstance(franchise, dict) or not franchise.get("title"):
            fail(errors, "canon.json: franchise.title is required")
        characters = canon.get("characters")
        if not isinstance(characters, dict):
            fail(errors, "canon.json: characters must be an object")
        else:
            missing = sorted(REQUIRED_CHARACTERS - characters.keys())
            if missing:
                fail(errors, "canon.json: missing protagonists: " + ", ".join(missing))
        films = canon.get("six_film_arc")
        if not isinstance(films, list) or len(films) != 6:
            fail(errors, "canon.json: six_film_arc must contain exactly six films")
        else:
            numbers = [film.get("film_number") for film in films if isinstance(film, dict)]
            if numbers != [1, 2, 3, 4, 5, 6]:
                fail(errors, "canon.json: film_number values must be ordered 1 through 6")

    manifest = read_json(manifest_path, errors)
    if isinstance(manifest, dict):
        if manifest.get("repository") != "gemquota/meatware-":
            fail(errors, "agent manifest repository must be gemquota/meatware-")
        for field in ("task_index",):
            value = manifest.get(field)
            if not isinstance(value, str):
                fail(errors, f"agent manifest: {field} must be a path string")
            else:
                check_repo_path(value, f"agent manifest {field}", errors)
        for field in ("canon_sources", "governance_sources"):
            values = manifest.get(field)
            if not isinstance(values, list) or not values:
                fail(errors, f"agent manifest: {field} must be a non-empty array")
                continue
            for value in values:
                if not isinstance(value, str):
                    fail(errors, f"agent manifest: invalid path in {field}: {value!r}")
                else:
                    check_repo_path(value, f"agent manifest {field}", errors)
        tasks = manifest.get("tasks")
        if not isinstance(tasks, list) or not tasks:
            fail(errors, "agent manifest: tasks must be a non-empty array")
        else:
            ids = [task.get("id") for task in tasks if isinstance(task, dict)]
            if len(ids) != len(tasks):
                fail(errors, "agent manifest: every task must be an object")
            if len(ids) != len(set(ids)):
                fail(errors, "agent manifest: task IDs must be unique")
            for task in tasks:
                if not isinstance(task, dict):
                    continue
                task_id = task.get("id", "<missing-id>")
                path = task.get("path")
                if not isinstance(path, str):
                    fail(errors, f"agent manifest task {task_id}: path is required")
                else:
                    check_repo_path(path, f"agent manifest task {task_id}", errors)
                issue_url = task.get("issue_url")
                if not isinstance(issue_url, str) or not issue_url.startswith(
                    "https://github.com/gemquota/meatware-/issues/"
                ):
                    fail(errors, f"agent manifest task {task_id}: invalid issue_url")

    credits = read_json(credits_path, errors)
    if isinstance(credits, dict):
        if not isinstance(credits.get("entries"), list):
            fail(errors, "CREDITS.json: entries must be an array")
        if not isinstance(credits.get("adjustments"), list):
            fail(errors, "CREDITS.json: adjustments must be an array")
        policy = credits.get("policy")
        if isinstance(policy, str):
            check_repo_path(policy, "CREDITS.json policy", errors)
        else:
            fail(errors, "CREDITS.json: policy path is required")


def validate_markdown_links(errors: list[str]) -> int:
    checked = 0
    for markdown in sorted(ROOT.rglob("*.md")):
        if ".git" in markdown.parts:
            continue
        text = markdown.read_text(encoding="utf-8")
        for match in LINK_RE.finditer(text):
            raw = match.group(1).strip()
            if raw.startswith("<") and raw.endswith(">"):
                raw = raw[1:-1]
            # Ignore optional Markdown link titles after the destination.
            if raw.startswith(("http://", "https://", "mailto:", "tel:", "#")):
                continue
            destination = raw.split()[0] if raw else ""
            if not destination:
                continue
            parsed = urlsplit(destination)
            if parsed.scheme or parsed.netloc:
                continue
            target = unquote(parsed.path)
            if not target:
                continue
            resolved = (ROOT / target.lstrip("/")) if target.startswith("/") else (markdown.parent / target)
            resolved = resolved.resolve()
            if ROOT not in resolved.parents and resolved != ROOT:
                fail(errors, f"{markdown.relative_to(ROOT)}: link escapes repository: {destination}")
                continue
            checked += 1
            if not resolved.exists():
                fail(errors, f"{markdown.relative_to(ROOT)}: broken local link: {destination}")
    return checked


def main() -> int:
    errors: list[str] = []
    validate_json_contracts(errors)
    links_checked = validate_markdown_links(errors)
    if errors:
        print(f"Validation failed with {len(errors)} error(s):")
        for error in errors:
            print(f" - {error}")
        return 1
    print("Repository validation passed.")
    print(f" - Canon, agent manifest, and credits JSON: valid")
    print(f" - Local Markdown links checked: {links_checked}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
