# MEATWARE

**An abstract computational laboratory for investigating the emergence of evolution and cumulative adaptation.**

Meatware investigates whether heritable variation, differential propagation, selection and cumulative adaptation can arise from minimal interacting systems without encoding biological categories into the underlying model.

The substrate has no predefined organisms, cells, genes, species, metabolisms or reproductive operations. It models entities, state, relationships, transformations and configurable environmental processes. Biological terminology may classify observations, never define simulation primitives.

## Project documents

- [Experimental specification](docs/SPECIFICATION.md)
- [Phased development plan](docs/DEVELOPMENT_PLAN.md)
- [Architecture principles](docs/ARCHITECTURE_PRINCIPLES.md)
- [Substrate model contract](docs/SUBSTRATE_MODEL.md)
- [Transition and conflict-resolution contract](docs/TRANSITION_CONTRACT.md)
- [Pending architecture decisions](docs/DECISIONS_PENDING.md)
- [Phase 1 record](docs/PHASE_1.md)
- [Phase 2 record](docs/PHASE_2.md)

## Current status

**Phase 2 complete:** logical substrate semantics and transition contracts are defined and versioned. Phase 3 will implement a small deterministic reference runtime against these contracts. Language and physical storage layout remain open until representative workloads can be compared.

## Central research question

> Under what conditions can interacting, persistent and variable configurations give rise to heritable change, differential propagation, selection and cumulative adaptation, without those phenomena being explicitly programmed into the substrate?

## Non-negotiable principles

1. No biological primitives in the substrate.
2. No hardcoded evolutionary outcomes or privileged reproduction operation.
3. No universal fitness score.
4. No assumption that complexity must increase.
5. Analysis is observational and cannot modify simulation state.
6. Baseline experiments are immutable and reproducible.
7. Optimisations are compared against reference behaviour.
8. Negative results and anomalies are preserved.

Emergence is not guaranteed by architecture or visual demonstration. The project succeeds by testing hypotheses, preserving evidence, reproducing results and distinguishing negative findings from inadequate models or instrumentation.
