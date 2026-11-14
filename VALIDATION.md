# Repository Validation

The repository has a small, dependency-free validation command:

```bash
python scripts/validate_project.py
```

It checks:

- JSON syntax and minimum structural contracts for the franchise canon, agent manifest, and credits ledger.
- The six-film sequence and required protagonist records.
- Every file path declared by the agent manifest and credits ledger.
- Unique task IDs and correctly scoped task issue URLs.
- Relative Markdown links across the repository, including links that escape the repository root.

GitHub Actions runs the same command on every push and pull request. A passing check means the repository structure is internally consistent; it does **not** mean a story decision is approved, a contribution is accepted, rights are cleared, or production is ready.

The validator intentionally uses only the Python standard library. Extend its checks when the repository adopts new machine-readable contracts, and keep editorial decisions with human reviewers.
