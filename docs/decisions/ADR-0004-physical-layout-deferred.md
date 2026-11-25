# ADR-0004: Defer Physical Storage and Language Selection

**Status:** Accepted  
**Phase:** 2

## Context

The desired logical model is graph-oriented, but the best physical representation depends on measured entity counts, relation density, property access patterns, update locality, recursive query patterns, memory budgets and execution mode.

The user selected dynamic graph/network semantics, no fixed conceptual entity ceiling, hierarchical interaction ranges, process-specific precision, approximation of distant/low-activity processes, multiple execution models and a balanced performance objective.

## Decision

Fix logical semantics now; defer language, runtime and physical storage. Phase 3 will implement a minimal reference runtime. Phase 8 will benchmark dense, sparse, graph-oriented and hybrid layouts against representative workloads.

## Consequences

- No language is selected by popularity or aesthetics.
- Logical graph semantics do not mandate heap-allocated graph objects.
- Benchmark workloads must include sparse and dense relation regimes, property-heavy state, recursive traversal, spatial neighbourhoods and transformation batches.
- Optimisations must report scientific divergence as well as speed.

## Revisit when

Phase 3 workloads and a comparative prototype provide sufficient evidence to choose an implementation stack.
