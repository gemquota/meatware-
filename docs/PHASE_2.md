# Phase 2: Substrate Semantics and Model Contracts

**Status:** Complete  
**Goal:** Define the smallest coherent abstract substrate and state-transition contract before implementing a runtime.

## Deliverables

- [x] Substrate model contract.
- [x] Transition and conflict-resolution contract.
- [x] Explicit entity, relation, identity and lifetime semantics.
- [x] Extensible typed property-value domain.
- [x] Recursive relation-to-relation organisation.
- [x] Generic transformations with atomic transition batches.
- [x] Explicit conflict classes and selectable resolution policies.
- [x] Random-stream and determinism requirements.
- [x] Canonical state and read-only observation requirements.
- [x] Model constraints are configurable rather than universal.
- [x] Conformance scenarios for future implementations.
- [x] Architecture decision records and unresolved-decision register.
- [x] Documentation validation and tests updated.

## Decisions established

1. Entity and relation identity is opaque, stable within a run and never reused.
2. Relations are first-class state-bearing items and may reference other relations.
3. There is no universal parent/child tree, root, owner or organisational hierarchy.
4. Properties use a portable typed value domain and are extensible by versioned extensions.
5. Missing and null are distinct; non-finite floating-point values are excluded from the portable core.
6. State changes are proposals evaluated against a completed source configuration.
7. A transition batch is atomic. Conflicts are detected and resolved by declared model policies.
8. Last-write-wins is never an implicit fallback.
9. New items become eligible at the next logical boundary under reference semantics.
10. Randomness uses explicit streams and stable keys.
11. Determinism is guaranteed only within a declared execution/numeric contract.
12. Conservation and other physical-like constraints are selectable model rules.
13. Observation is read-only; analytical classifications remain outside substrate state.
14. The logical model is graph-oriented. Physical storage layout and implementation language remain open.

## Deferred decisions

- Implementation language and runtime.
- Dense/sparse/graph storage layout and memory allocator strategy.
- Concrete numeric precision per process.
- Exact parallel execution model.
- Continuous-time and hybrid scheduler implementation.
- Spatial indexing and graph query algorithms.
- Serialization format and schema migration mechanism.
- Performance thresholds and benchmark hardware.
- How much causal history is retained in memory versus external storage.

These are intentionally deferred because the contract does not yet have representative runtime workloads. Choosing them now would be architecture by aesthetic preference.

## Exit criteria

The contract is sufficiently precise to implement a reference runtime without inventing semantics for identity, state ownership, relation lifecycle, transition atomicity, conflicts, randomness or observation. All normative rules have corresponding conformance scenarios. No language or runtime has been prematurely selected.

## Completion record

Pending repository CI verification.
