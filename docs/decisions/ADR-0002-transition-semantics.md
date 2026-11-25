# ADR-0002: Atomic Proposal-Based Transitions

**Status:** Accepted  
**Phase:** 2

## Context

Interactions may be concurrent, multi-party, stochastic and state-dependent. Direct in-place mutation makes outcomes depend on iteration order, complicates replay and hides conflicts.

## Decision

Rules generate immutable transformation proposals against a completed source configuration. A resolver checks dependencies, identifies conflicts, applies explicitly configured policies and validates the candidate result. The complete accepted batch commits atomically.

New items are not eligible until the next logical boundary under reference semantics. Failed batches leave source state unchanged. Last-write-wins is not an implicit policy.

## Consequences

- Rule evaluation is isolated from live mutation.
- Conflicts become observable experimental outcomes.
- Deterministic execution has a well-defined reference point.
- Parallel evaluation is possible, subject to stable proposal keys and deterministic resolution.
- Some models may require alternative scheduling, but must declare their semantics.

## Alternatives considered

1. **Immediate in-place mutation:** rejected due to order dependence and poor replay semantics.
2. **Unrestricted last-write-wins:** rejected because it hides conflict and creates accidental selection pressure from iteration order.
3. **Proposal/evaluate/resolve/commit:** accepted as the reference transition contract.

## Revisit when

A concrete model demonstrates that atomic logical boundaries cannot represent a required process. Any alternative must preserve explicit causal ordering, conflict records and reproducibility guarantees.
