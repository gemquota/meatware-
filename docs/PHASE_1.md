# Phase 1: Repository and Scientific Foundations

**Status:** Complete  
**Goal:** Establish the new project identity and make its scientific contract executable as repository checks.

## Scope

- Preserve the previous repository state on an archive branch.
- Replace active project files with Meatware documentation and minimal validation infrastructure.
- Store the authoritative specification and full development roadmap.
- Establish scientific and architectural invariants.
- Add automated documentation checks and CI.
- Do not implement simulation semantics or select a runtime without comparative evidence.

## Tasks

- [x] Inspect repository identity, default branch and tracked tree.
- [x] Record pre-transition commit.
- [x] Create archive branch at pre-transition commit.
- [x] Replace active tree with new project files.
- [x] Add documentation validator and tests.
- [x] Add CI workflow.
- [x] Verify new tree, links, tests and archive branch.
- [x] Record resulting commit and phase status.

## Acceptance criteria

1. Archive branch points to the exact pre-transition commit.
2. Active branch contains no old project material.
3. README links to the specification, plan and architecture principles.
4. Documentation validator passes.
5. Unit tests pass.
6. CI runs the same checks.
7. Specification and roadmap agree on scientific invariants.
8. Runtime and language remain uncommitted pending Phase 2 evidence.

## Out of scope

- Choosing a simulation language or renderer.
- Building an entity engine.
- Implementing evolutionary or biological mechanisms.
- Deleting GitHub issues, releases, tags or historical commits.
- Claiming emergence or adaptation has been demonstrated.

## Completion record

- Pre-transition commit: `3dda417c127f78433bdbbe5e19ae92c8c7fc4d36`.
- Archive branch: `archive/pre-meatware-evolution-reset`.
- Foundation commit: `bb978879629dfa39ce7167b245fd11cf2326deca`.
- CI run: [Validate project foundations](https://github.com/gemquota/meatware-/actions/runs/45525177904).
- Result: documentation validation and unit tests passed.
- Active tree: 9 files across README, specification, roadmap, architecture/phase documentation, validator, tests, CI and ignore rules.
- No simulation engine or runtime language has been selected.
- Git history and remote issue/release records were not deleted.
