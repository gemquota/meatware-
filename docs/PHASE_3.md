# Phase 3: Deterministic Reference Runtime

**Status:** In progress  
**Selected implementation language:** Rust  
**Goal:** Build a deliberately simple, trustworthy executable baseline against the Phase 2 contracts.

## Language decision

Rust is selected as the primary reference runtime language. The C++/Rust candidate parity workload provides initial semantic and performance evidence, but timing from shared hosted runners is informational rather than a general performance claim. The logical substrate contract remains language-neutral. Rust-specific representation choices must not leak into the scientific model.

## Implemented runtime foundation

- [x] C++/Rust candidate parity workload: 200,000 entities, 1.2 million relations, eight update steps.
- [x] Rust crate with deterministic ordered storage and serde-based snapshots.
- [x] Opaque numeric identities and retired-ID non-reuse tracking.
- [x] Extensible typed property values and first-class state-bearing relations.
- [x] Recursive relation-to-relation endpoints.
- [x] Proposal-based atomic transition batches.
- [x] Stable proposal ordering, source-boundary read dependencies and explicit conflict policies (reject by default; lowest/highest proposal key wins when selected).
- [x] Explicit SplitMix64 random streams with stable event-local key derivation from boundary, proposal key and channel.
- [x] Canonical ordered-state encoding and diagnostic state hashes.
- [x] Versioned snapshot envelopes with legacy unwrapped-snapshot loading.
- [x] Append-only transition event log with source/target boundary and hash verification.
- [x] Deterministic replay by reapplying recorded proposal batches.
- [x] JSON event-log export/import.
- [x] Replay integrity checks detect tampered records and out-of-band state mutation.
- [x] Headless executable entry point.
- [x] Unit tests for runtime invariants, snapshot migration and replay.

## Remaining Phase 3 work

- [x] Extend read dependencies to relation endpoint sets.
- [ ] Extend read dependencies to versioned predicates and environmental state.
- [x] Add a versioned run-manifest schema recording the actual PRNG, stream-consumption scheme, seed, model parameters, conflict policy, initial state hash and terminal/replay hashes.
- [ ] Switch applicable dynamics to event-local streams and record their derivation scheme in the manifest.
- [ ] Add property-based and fuzz testing.
- [ ] Measure memory and runtime on mutation-heavy workloads.
- [ ] Validate cross-platform numeric reproducibility limits.
- [ ] Complete a headless experiment runner that emits the shared manifest format.
- [ ] Define a stable snapshot migration policy beyond the first version.

## Event log and snapshot semantics

A snapshot is a versioned envelope containing the complete configuration. The loader continues to accept the original unwrapped configuration format as a legacy version. Unknown versioned formats are rejected explicitly.

An event log stores the initial snapshot and an ordered sequence of transition records. Each record contains the source and target logical boundaries, source and target state hashes, and the proposal batch that produced the transition. Replay starts from the initial snapshot, checks the source boundary/hash, reapplies the proposal batch through the reference transition function, then checks the target boundary/hash.

Appending is transactional: a candidate state is evaluated first, and neither the live configuration nor the log changes if the transition fails. The log also detects if a caller has mutated the live configuration outside the recorded transition path.

The event log records computational transitions, not a complete philosophical or physical causal explanation. Proposals now carry source-state read predicates (existence and expected property values), checked before edits. Property-write conflicts can be rejected or resolved by lowest/highest stable proposal key; rejection remains the default. These decisions are replayed because the proposal batch is recorded. The run-manifest schema records the selected conflict policy and actual random generator/stream scheme. Each event-log transition also records its conflict policy, so replay does not silently fall back to a different resolver. The browser MVP currently uses the default reject policy and a single sequential random stream; event-local stream derivation is available in the runtime but is not yet used by that model. Environmental reads and complete causal references remain future work.

## Deliberate limitations

The first runtime uses ordered maps for deterministic iteration and canonical state. This is a correctness baseline, not a performance claim. The default resolver rejects conflicting property writes. Explicit alternatives select the lowest or highest stable proposal key; these are deterministic baseline policies, not universal scientific defaults. Read dependencies currently cover entity/relation existence and expected property values, not arbitrary predicates or version counters. Entity retirement is rejected if it leaves an active relation with a dangling endpoint.

State hashes use FNV-1a 64-bit for diagnostics and regression checks, not cryptographic integrity. Versioned JSON snapshots do not yet promise cross-version or cross-platform bitwise equivalence. Deterministic replay is guaranteed only to the extent that the same runtime semantics, numeric behaviour and recorded proposal inputs are preserved. The runtime does not yet implement the complete proposal dependency and causal-event model from the Phase 2 contract.

## Verification

The versioned manifest schema and browser runtime export are documented in [RUN_MANIFEST.md](RUN_MANIFEST.md). The latest repository CI is the authoritative verification record. It runs documentation validation, Python tests, native Rust tests and the headless runtime. The browser Pages workflow separately compiles the Rust WebAssembly target and deploys the site. No local Rust compilation is claimed where the tool environment does not provide Cargo.

No biological primitives, privileged reproduction operation or renderer dependency are present in the reference substrate.
