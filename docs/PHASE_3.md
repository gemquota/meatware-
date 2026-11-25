# Phase 3: Deterministic Reference Runtime

**Status:** In progress  
**Goal:** Build a deliberately simple, trustworthy executable baseline against the Phase 2 contracts.

## Candidate comparison work

- [x] Select C++ and Rust as candidate languages, not final decisions.
- [x] Define a shared sparse-graph workload with 200,000 entities and 1.2 million relations.
- [x] Implement matching CSR construction and state-update loops.
- [x] Implement stable proposal ordering and conflict resolution.
- [x] Add checksum parity validation and a dedicated CI workflow.
- [ ] Run repeated benchmarks on controlled hardware.
- [ ] Select a primary implementation language based on evidence and project constraints.

## Runtime work

- [ ] Implement entity and relation storage against the Phase 2 contracts.
- [ ] Implement controlled state access and mutation.
- [ ] Implement explicit keyed random streams.
- [ ] Implement deterministic logical stepping and atomic transition batches.
- [ ] Implement canonical serialization and state hashing.
- [ ] Implement snapshot save/load and replay.
- [ ] Turn transition conformance fixtures into executable runtime tests.
- [ ] Add invariant checks and failure diagnostics.
- [ ] Publish baseline performance and memory measurements.

## Acceptance criteria

1. Candidate implementations execute the same workload and formulas.
2. Checksums match within a declared floating-point tolerance.
3. Language selection follows repeated measurements on documented hardware.
4. The reference runtime implements identity, property, relation and transition contracts.
5. Same manifest and seed reproduce canonical hashes under the declared reference mode.
6. Snapshot save/load preserves canonical state.
7. Invalid transitions fail without partially mutating source state.
8. Headless execution requires no renderer or UI.

## Current decision

The logical model is graph-oriented; physical storage and language remain open. The benchmark is deliberately narrow and cannot justify a decision by itself.
