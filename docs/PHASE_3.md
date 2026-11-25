# Phase 3: Deterministic Reference Runtime

**Status:** In progress  
**Selected implementation language:** Rust  
**Goal:** Build a deliberately simple, trustworthy executable baseline against the Phase 2 contracts.

## Language decision

Rust is selected as the primary reference runtime language. The completed candidate parity smoke test provides initial evidence, and Rust fits the project's engineering constraints: explicit ownership, memory safety without a garbage collector, strong data modelling and reproducible builds. The first timing sample favoured Rust (38.328 ms versus 41.311 ms), but a single GitHub-hosted runner observation is supporting evidence only, not a general performance claim. C++ remains a comparison baseline.

The logical substrate contract remains language-neutral. Rust-specific representation choices must not leak into the scientific model.

## Implemented runtime foundation

- [x] C++/Rust candidate parity workload: 200,000 entities, 1.2 million relations, eight update steps.
- [x] Rust crate with standard-library-first runtime and serde-based snapshots.
- [x] Opaque numeric identities and retired-ID non-reuse tracking.
- [x] Extensible typed property values and first-class state-bearing relations.
- [x] Recursive relation-to-relation endpoints.
- [x] Proposal-based atomic transition batches.
- [x] Stable proposal ordering and rejection of conflicting writes.
- [x] Explicit SplitMix64 random streams.
- [x] Canonical ordered-state encoding, snapshot round-trip and diagnostic state hash.
- [x] Headless executable entry point.
- [x] Unit tests for core invariants.

## Still open

- [ ] Add read-version dependencies and configurable conflict resolvers.
- [ ] Define stable event-local random-stream key derivation in the run manifest.
- [ ] Add schema versioning and snapshot migration.
- [ ] Add deterministic replay from an append-only event log.
- [ ] Add property-based and fuzz testing.
- [ ] Measure memory and runtime on mutation-heavy workloads.
- [ ] Validate cross-platform numeric reproducibility limits.
- [ ] Complete a headless experiment runner and manifest format.

## Deliberate limitations

The first runtime uses ordered maps for deterministic iteration and canonical state. This is a correctness baseline, not a performance claim. The initial resolver rejects conflicting writes rather than silently choosing a winner. Entity retirement is rejected if it leaves an active relation with a dangling endpoint. New items become visible only after commit.

State hashes use FNV-1a 64-bit for diagnostics and regression checks, not cryptographic integrity. Snapshots use ordered JSON; cross-version and cross-platform bitwise equivalence remain unclaimed pending explicit format and numeric-policy tests. The runtime does not yet implement the complete proposal dependency and causal-event model from the Phase 2 contract.

## Verification

- Candidate parity CI: [run 45528364851](https://github.com/gemquota/meatware-/actions/runs/45528364851).
- Rust unit tests, formatting and headless execution are now part of repository CI.
- No biological primitives, privileged reproduction operation or renderer dependency are present.
