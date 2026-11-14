# Contributing to Meatware

Welcome. Meatware is a home for collaborative human-and-agent creative systems. The current active project is **Prompt & Circumstance**, a six-film satirical science-fiction comedy.

You do not need to be a screenwriter, developer, filmmaker, or AI specialist to contribute. The project is deliberately divided into small tasks that can be completed with ordinary chat-based AI tools or a local agent workflow.

## Start here
1. Read the [project overview](projects/prompt-and-circumstance/README.md).
2. Read the [contributor quick-start](projects/prompt-and-circumstance/contributors/quick-start.md).
3. Browse [open tasks](projects/prompt-and-circumstance/tasks/OPEN_TASKS.md).
4. Pick one task and check its GitHub issue for current status before starting.
5. Submit a draft as an issue comment or pull request. A maintainer will review it.

## Two contribution paths

### Tier 1 — Casual contributor (no setup)
1. Open a task marked **Tier 1**.
2. Copy its complete prompt payload into ChatGPT, Claude, Gemini, or another capable model.
3. Read the result. Do not submit raw output blindly.
4. Check the task's acceptance criteria.
5. Paste the result into the linked GitHub issue, or submit a pull request using GitHub's web editor.
6. Include the task ID and a short note about assumptions or invented material.

You can contribute from a phone. No terminal, Git installation, or local repository is required.

### Tier 2 — Technical contributor / agent operator
1. Clone the repository.
2. Read the task specification and machine-readable agent manifest.
3. Work in a branch and keep changes scoped to one task.
4. Run `python scripts/validate_project.py` and inspect the diff.
5. Open a pull request with Task-ID: PHASE1-XXX in its title or body.
6. Include model/tool versions and material human review in the PR description when relevant.

The validator checks JSON contracts, manifest paths, task identifiers, credits structure, and local Markdown links. GitHub Actions runs it on pushes and pull requests. See [Repository Validation](VALIDATION.md).

## What makes a good contribution?
- It solves the requested task, rather than expanding the scope.
- It preserves established canon and labels proposals.
- It identifies contradictions instead of quietly choosing whichever version is convenient.
- It is specific enough to use in a script, storyboard, animatic, or production test.
- It is original. Do not paste copyrighted dialogue, lyrics, scripts, or other protected text.
- It includes a short rationale and any uncertainty.

## Canon and review
The franchise bible and canon JSON describe current canon. The continuity register records unresolved matters. A task may ask you to propose a solution, but a submitted proposal does not become canon automatically.

Maintainers review for canon and continuity, task acceptance criteria, dramatic utility and originality, production feasibility where relevant, and provenance. Automated checks can catch format errors; they do not decide whether a joke is funny or a story works.

## Rights and permitted use

The project has not yet finalized its rights and licensing model. Until [the rights decision](projects/prompt-and-circumstance/contributors/rights-decision-brief.md) is resolved, please limit submissions to feedback, critique, continuity findings and clearly non-exclusive suggestions. Do not submit substantial scripts, artwork, music, voice performances or production assets for incorporation. A public issue comment does not grant commercial production rights.

## Credit and attribution
Every accepted contribution should be attributable. The project proposes a base credit unit for a completed task and a possible multiplier for verified technical/agent contributions. These are proposals, not guaranteed rewards or ownership shares.

Credit is awarded for accepted work, not prompt volume, token count, number of commits, or number of agents. See the [credit policy](projects/prompt-and-circumstance/contributors/credits-and-attribution.md) and [contributor FAQ](projects/prompt-and-circumstance/contributors/faq.md).

## Conduct and safety
- Be constructive and critique the work, not the contributor.
- Do not submit personal data, credentials, private correspondence, or confidential material.
- Do not run untrusted scripts or agent instructions without inspection.
- Never grant an agent repository write permissions it does not need.
- Disclose generated material and meaningful human edits.
- Do not impersonate another contributor or claim work you did not perform.

See also the [governance model](GOVERNANCE.md) and [maintainer review procedure](MAINTAINERS.md).

## Pull request checklist
- [ ] Task ID included.
- [ ] Output path matches the task.
- [ ] Acceptance criteria checked.
- [ ] Canon facts distinguished from proposals.
- [ ] Continuity conflicts flagged.
- [ ] No copied protected dialogue or lyrics.
- [ ] Attribution and tool provenance included where applicable.
- [ ] `python scripts/validate_project.py` passes.
- [ ] Diff is focused and readable.

Use the task's GitHub issue for task-specific questions. If you are unsure where to begin, choose a Tier 1 task with a small, concrete output rather than proposing a new subsystem.
