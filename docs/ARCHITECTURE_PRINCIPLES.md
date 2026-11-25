# Architecture Principles

These principles constrain every implementation phase. The normative substrate and transition definitions live in [SUBSTRATE_MODEL.md](SUBSTRATE_MODEL.md) and [TRANSITION_CONTRACT.md](TRANSITION_CONTRACT.md).

## Separation of concerns

- **Substrate:** entities, properties, state and relationships.
- **Dynamics:** interactions, transformations, scheduling and environmental processes.
- **Environment:** external conditions, fields and resource processes.
- **Analysis:** measurements, hypotheses, causal analysis and comparisons.
- **Visualisation:** observation, inspection, dashboards and export.
- **Experiment orchestration:** runs, seeds, branches and provenance.

Analysis and visualisation must not mutate simulation state. Substrate and dynamics must not depend on biological classifications or rendering.

## Logical substrate

- Model a dynamic graph of state-bearing entities and state-bearing relations.
- Relations may reference entities or other relations.
- Do not impose a universal root, owner, parent/child tree or organisational hierarchy.
- Give identities opaque, stable, non-reused semantics within a run.
- Keep property values typed, extensible and canonically serializable.
- Treat composite configurations as observed arrangements, not privileged domain objects.

## Transition semantics

- Evaluate immutable proposals against a completed source configuration.
- Resolve conflicts through explicit, versioned policies.
- Commit accepted batches atomically.
- Never silently use last-write-wins.
- Give newly created items a declared eligibility boundary.
- Use explicit keyed random streams.
- Make failed transitions and causal dependencies observable.

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

Visual resemblance is useful for exploratory screening, not proof of scientific equivalence. Compare state distributions, interaction statistics, persistence, propagation, causal traces and relevant trajectory properties. State clearly when exact equivalence is impossible or not claimed.

## Architecture decision records

A significant decision records context, options, evidence, consequences and revisit conditions. Decisions without measurements must be labelled provisional.
