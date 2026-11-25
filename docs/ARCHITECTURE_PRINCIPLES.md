# Architecture Principles

These principles constrain every implementation phase.

## Separation of concerns

- **Substrate:** entities, properties, state and relationships.
- **Dynamics:** interactions, transformations, scheduling and environmental processes.
- **Environment:** external conditions, fields and resource processes.
- **Analysis:** measurements, hypotheses, causal analysis and comparisons.
- **Visualisation:** observation, inspection, dashboards and export.
- **Experiment orchestration:** runs, seeds, branches and provenance.

Analysis and visualisation must not mutate simulation state. Substrate and dynamics must not depend on biological classifications or rendering.

## Scientific invariants

1. Biological concepts are not substrate primitives.
2. No privileged reproduction or evolution operation.
3. No universal fitness scalar.
4. No assumption that complexity increases.
5. No fixed hierarchy of organisational levels.
6. No hidden observer intervention.
7. No silent baseline changes.
8. No optimisation without reference comparisons.

## Implementation constraints

- Keep a simple reference execution path.
- Make state transitions explicit and testable.
- Version model rules and experiment configurations.
- Use deterministic random streams when reproducibility is requested.
- Keep execution order and conflict resolution observable.
- Record approximation, precision and scheduling choices.
- Use canonical serialization where practical.
- Prefer data-oriented and sparse representations only where benchmarks support them.
- Do not couple simulation semantics to rendering cadence.
- Keep dependencies replaceable behind narrow interfaces.

## Evidence standard

Visual resemblance is not equivalence. Compare state distributions, interaction statistics, persistence, propagation, causal traces and relevant trajectory properties. State clearly when exact equivalence is impossible or not claimed.

## Architecture decision records

A significant decision records context, options, evidence, consequences and revisit conditions. Decisions without measurements must be labelled provisional.
