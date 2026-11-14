# TASK-ID — Short task title

**Kind:** review / feedback_suggestions / creative_outline / production_benchmark  
**Status:** open / claimed / submitted / in_review / revision_requested / accepted / rejected / blocked / superseded  
**Availability:** open / blocked_rights_gate / blocked_dependency / maintainer_only  
**Rights scope:** feedback_only / gated_creative_work  
**Tier:** 1 / 2 / Both  
**Effort:** minimum–maximum minutes  
**Owner:** Unassigned  
**Spec path:** path/to/task-specification  
**Target output:** path/to/output or issue feedback  
**Depends on task IDs:** []  
**Source paths:** authoritative inputs used by the task  
**Issue URL:** https://github.com/gemquota/meatware-/issues/NNN  
**Human review required:** true

## Goal
One sentence describing the output.

## Context
Only the source material needed to complete this task. Link authoritative canon and relevant continuity entries.

## Instructions
A self-contained task prompt. Define expected format and scope. Tell contributors what not to change. Respect the task's rights scope and availability.

## Acceptance criteria
- [ ] Required output exists and has the specified structure.
- [ ] Established canon is preserved.
- [ ] New proposals and assumptions are labelled.
- [ ] Continuity conflicts are flagged.
- [ ] Output is original and provenance is clear.
- [ ] A reviewer can use the artifact in the next pipeline stage.
- [ ] Human review is completed before acceptance.

## Submission
Use the linked GitHub issue or open a PR. Include Task-ID, chosen public handle, summary, assumptions, canon conflicts, artifact reference and meaningful tool disclosure. Do not include private contact details.

For structured records, follow `submission.schema.json` and validate with:

```bash
python scripts/agent_tasks.py validate-submission path/to/submission.json
```

## Review notes
Record decisions and revision requests in the issue/PR. Only an authorized reviewer may accept work or change canon.

## Source links
- Authoritative source:
- Related task:
- Continuity entries:
