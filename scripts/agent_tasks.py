#!/usr/bin/env python3
"""Local, offline task discovery and contribution validation for Meatware.

This tool is deliberately read-only: it never claims issues, changes canon,
uploads files, merges pull requests, or grants contributor rights.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
REGISTRY_PATH = ROOT / "projects/prompt-and-circumstance/tasks/registry.json"
STATUSES = {"open", "claimed", "submitted", "in_review", "revision_requested", "accepted", "rejected", "blocked", "superseded"}
AVAILABILITY = {"open", "blocked_rights_gate", "blocked_dependency", "maintainer_only"}
RIGHTS_SCOPES = {"feedback_only", "gated_creative_work"}
SUBMISSION_KEYS = {
    "schema_version", "submission_id", "task_id", "contributor", "submitted_at",
    "artifact", "summary", "assumptions", "canon_conflicts", "tool_disclosure",
    "rights_scope_acknowledged", "status",
}


def load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise ValueError(f"Missing file: {path.relative_to(ROOT)}")
    except json.JSONDecodeError as exc:
        raise ValueError(f"Invalid JSON in {path.relative_to(ROOT)}: {exc}")


def safe_repo_file(value: str, label: str, errors: list[str]) -> None:
    candidate = (ROOT / value).resolve()
    if ROOT not in candidate.parents or not candidate.is_file():
        errors.append(f"{label} is missing or escapes repository: {value}")


def load_registry():
    data = load_json(REGISTRY_PATH)
    if not isinstance(data, dict) or not isinstance(data.get("tasks"), list):
        raise ValueError("Task registry must be an object containing a tasks array")
    return data


def validate_registry(data: dict) -> list[str]:
    errors: list[str] = []
    if data.get("schema_version") != "1.0":
        errors.append("Unsupported task registry schema_version")
    if data.get("repository") != "gemquota/meatware-":
        errors.append("Task registry repository must be gemquota/meatware-")
    tasks = data.get("tasks", [])
    by_id = {}
    for task in tasks:
        if not isinstance(task, dict):
            errors.append("Every task must be an object")
            continue
        task_id = task.get("id")
        if not isinstance(task_id, str) or not re.fullmatch(r"[A-Z0-9]+-[0-9]{3}", task_id):
            errors.append(f"Invalid task ID: {task_id!r}")
            continue
        if task_id in by_id:
            errors.append(f"Duplicate task ID: {task_id}")
        by_id[task_id] = task
        if task.get("status") not in STATUSES:
            errors.append(f"{task_id}: invalid status {task.get('status')!r}")
        if task.get("availability") not in AVAILABILITY:
            errors.append(f"{task_id}: invalid availability {task.get('availability')!r}")
        if task.get("rights_scope") not in RIGHTS_SCOPES:
            errors.append(f"{task_id}: invalid rights_scope")
        if task.get("rights_scope") == "gated_creative_work" and task.get("availability") == "open":
            errors.append(f"{task_id}: gated creative work cannot be openly available while rights are unresolved")
        effort = task.get("effort_minutes", {})
        if not isinstance(effort, dict) or not isinstance(effort.get("min"), int) or not isinstance(effort.get("max"), int) or effort["min"] < 1 or effort["max"] < effort["min"]:
            errors.append(f"{task_id}: invalid effort_minutes range")
        issue = urlparse(task.get("issue_url", ""))
        if issue.scheme != "https" or issue.netloc != "github.com" or not issue.path.startswith("/gemquota/meatware-/issues/"):
            errors.append(f"{task_id}: issue_url must point to this repository")
        if task.get("human_review_required") is not True:
            errors.append(f"{task_id}: human_review_required must remain true")
        for field in ("spec_path", "output_path"):
            value = task.get(field)
            if value is not None:
                safe_repo_file(value, f"{task_id}.{field}", errors)
        for field in ("source_paths",):
            values = task.get(field)
            if not isinstance(values, list):
                errors.append(f"{task_id}.{field} must be an array")
            else:
                for value in values:
                    safe_repo_file(value, f"{task_id}.{field}", errors)
        if not isinstance(task.get("acceptance"), list) or not task["acceptance"]:
            errors.append(f"{task_id}: acceptance criteria must be a non-empty array")
    for task_id, task in by_id.items():
        for dependency in task.get("depends_on", []):
            if dependency not in by_id:
                errors.append(f"{task_id}: unknown task dependency {dependency}")
    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(task_id: str) -> None:
        if task_id in visiting:
            errors.append(f"Dependency cycle detected at {task_id}")
            return
        if task_id in visited or task_id not in by_id:
            return
        visiting.add(task_id)
        for dependency in by_id[task_id].get("depends_on", []):
            visit(dependency)
        visiting.remove(task_id)
        visited.add(task_id)

    for task_id in by_id:
        visit(task_id)
    return errors


def dependency_blockers(task: dict, by_id: dict) -> list[str]:
    blockers = []
    for dependency in task.get("depends_on", []):
        parent = by_id.get(dependency)
        if not parent or parent.get("status") != "accepted":
            blockers.append(dependency)
    return blockers


def task_summary(task: dict, by_id: dict) -> dict:
    blockers = dependency_blockers(task, by_id)
    available = task.get("availability") == "open" and task.get("status") == "open" and not blockers
    return {
        "id": task["id"],
        "title": task["title"],
        "status": task["status"],
        "availability": task["availability"],
        "ready": available,
        "blocked_by": blockers,
        "rights_scope": task["rights_scope"],
        "effort_minutes": task["effort_minutes"],
        "issue_url": task["issue_url"],
    }


def emit(value, as_json: bool) -> None:
    if as_json:
        print(json.dumps(value, indent=2, ensure_ascii=False))
    elif isinstance(value, list):
        for item in value:
            state = "READY" if item.get("ready") else item.get("availability", "UNAVAILABLE").upper()
            effort = item.get("effort_minutes", {})
            print(f'{item["id"]:<13} [{state:<20}] {effort.get("min","?")}-{effort.get("max","?")} min  {item["title"]}')
    else:
        print(json.dumps(value, indent=2, ensure_ascii=False))


def validate_submission(path: Path, registry: dict) -> list[str]:
    errors = []
    try:
        payload = load_json(path)
    except ValueError as exc:
        return [str(exc)]
    if not isinstance(payload, dict):
        return ["Submission must be a JSON object"]
    extra = set(payload) - SUBMISSION_KEYS
    missing = SUBMISSION_KEYS - set(payload)
    if extra:
        errors.append("Unknown fields: " + ", ".join(sorted(extra)))
    if missing:
        errors.append("Missing fields: " + ", ".join(sorted(missing)))
    if payload.get("schema_version") != "1.0":
        errors.append("schema_version must be 1.0")
    task_id = payload.get("task_id")
    task = next((item for item in registry["tasks"] if item.get("id") == task_id), None)
    if task is None:
        errors.append(f"Unknown task_id: {task_id!r}")
    contributor = payload.get("contributor")
    if not isinstance(contributor, str) or not contributor.strip() or len(contributor) > 80:
        errors.append("contributor must be a chosen public handle of 1-80 characters")
    if not isinstance(payload.get("submission_id"), str) or not re.fullmatch(r"[A-Za-z0-9._-]+", payload["submission_id"]):
        errors.append("submission_id must use letters, digits, dot, underscore or hyphen")
    try:
        datetime.fromisoformat(payload.get("submitted_at", "").replace("Z", "+00:00"))
    except (AttributeError, ValueError):
        errors.append("submitted_at must be an ISO-8601 date-time")
    artifact = payload.get("artifact")
    if not isinstance(artifact, dict) or artifact.get("type") not in {"issue_comment", "pull_request", "repository_path", "external_link"} or not isinstance(artifact.get("reference"), str) or not artifact["reference"].strip():
        errors.append("artifact must contain a supported type and non-empty reference")
    if not isinstance(payload.get("summary"), str) or not payload["summary"].strip() or len(payload["summary"]) > 2000:
        errors.append("summary must be 1-2000 characters")
    for field in ("assumptions", "canon_conflicts"):
        if field in payload and (not isinstance(payload[field], list) or not all(isinstance(x, str) for x in payload[field])):
            errors.append(f"{field} must be an array of strings")
    if "tool_disclosure" in payload and (not isinstance(payload["tool_disclosure"], str) or len(payload["tool_disclosure"]) > 500):
        errors.append("tool_disclosure must be a string of at most 500 characters")
    if payload.get("rights_scope_acknowledged") is not True:
        errors.append("rights_scope_acknowledged must be true")
    if payload.get("status", "submitted") not in {"submitted", "in_review", "revision_requested", "accepted", "rejected"}:
        errors.append("invalid submission status")
    if task and task.get("availability") != "open":
        errors.append(f"{task_id} is not open for submissions ({task.get('availability')}); do not submit work under the blocked scope")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Discover tasks and validate contribution records.")
    parser.add_argument("--json", action="store_true", help="emit machine-readable JSON")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("list", help="list all tasks")
    sub.add_parser("ready", help="list tasks currently available for feedback")
    show = sub.add_parser("show", help="show one task contract")
    show.add_argument("task_id")
    validate = sub.add_parser("validate-submission", help="validate a structured submission JSON file")
    validate.add_argument("path", type=Path)
    sub.add_parser("validate-registry", help="validate task IDs, paths, rights gates and dependency graph")
    args = parser.parse_args()

    try:
        registry = load_registry()
    except ValueError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2
    errors = validate_registry(registry)
    if errors:
        print("Task registry invalid:", file=sys.stderr)
        for error in errors:
            print(f" - {error}", file=sys.stderr)
        return 2
    by_id = {task["id"]: task for task in registry["tasks"]}

    if args.command == "validate-registry":
        result = {"valid": True, "task_count": len(by_id), "rights_gated": sum(t["availability"] == "blocked_rights_gate" for t in by_id.values())}
        emit(result, args.json)
        return 0
    if args.command == "list":
        values = [task_summary(t, by_id) for t in registry["tasks"]]
        emit(values, args.json)
        return 0
    if args.command == "ready":
        values = [task_summary(t, by_id) for t in registry["tasks"] if task_summary(t, by_id)["ready"]]
        emit(values, args.json)
        return 0
    if args.command == "show":
        task = by_id.get(args.task_id)
        if not task:
            print(f"Unknown task: {args.task_id}", file=sys.stderr)
            return 1
        blockers = dependency_blockers(task, by_id)
        emit({"task": task, "ready": task["availability"] == "open" and task["status"] == "open" and not blockers, "blocked_by": blockers}, args.json)
        return 0
    if args.command == "validate-submission":
        problems = validate_submission(args.path, registry)
        if problems:
            print("Submission invalid:")
            for problem in problems:
                print(f" - {problem}")
            return 1
        print("Submission structure valid. Human review is still required; this does not accept the work or grant rights.")
        return 0
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
