# Meatware: Phased Development Plan

**Status:** Active roadmap  
**Basis:** Experimental Specification v1.0  
**Delivery rule:** Each phase ends with evidence and an exit gate. Code existing is not proof that a phase is complete.

## Programme map

| Phase | Name | Outcome |
|---|---|---|
| 1 | Repository and scientific foundations | Clean project, specification, roadmap, architecture contracts and validation |
| 2 | Substrate semantics and model contracts | Precise entity, state, relationship, transformation and invariant definitions |
| 3 | Deterministic reference runtime | Small executable engine with reproducible state transitions |
| 4 | General interaction and transformation system | Composable rules, interaction scheduling and emergent composition |
| 5 | Environment and resource dynamics | Spatial fields, transport, replenishment, gradients and interventions |
| 6 | Information transfer and evolutionary evidence | Repeatable transfer, variation, propagation and adaptation measurements |
| 7 | Observation, provenance and causal instrumentation | Read-only inspection, event histories, snapshots and deterministic replay |
| 8 | Performance and multiscale execution | Benchmarked indexing, approximation and execution alternatives |
| 9 | Experimental orchestration and statistical controls | Trial matrices, matched controls, branching and analysis pipelines |
| 10 | Minimal scientific visualisation | Abstract world viewer, metrics, timelines and experiment comparison |
| 11 | Research programme and discovery cycles | Reproducible experiments targeting cumulative adaptation |
| 12 | Validation, hardening and release | Documented limitations, reproducibility package and stable release |

Phases are sequential gates, not calendar promises. Estimates are deferred until implementation language, runtime and benchmark scope are selected.

---

## Phase 1 — Repository and scientific foundations

**Goal:** Replace the former project with a coherent scientific software project and establish constraints against accidental drift into a conventional artificial-life simulator.

**Work**
1. Preserve the pre-transition state on an archive branch.
2. Replace active tracked files with Meatware documentation and minimal engineering infrastructure.
3. Commit the consolidated specification as the authoritative contract.
4. Commit this roadmap and architecture principles.
5. Add lightweight documentation validation and automated CI.
6. Define phase gates and decision-record conventions.
7. Inventory unresolved technical decisions without prematurely choosing implementations.

**Deliverables:** README, specification, roadmap, architecture principles, Phase 1 record, validator, tests, CI and recoverable archive branch.

**Acceptance:** archive points to exact old commit; active branch contains no old project material; core documents are cross-linked and consistent; validation and tests pass; no biological substrate primitive is introduced; language/runtime remain open pending evidence.

**Exit gate:** project identity and scientific contract are stable.

## Phase 2 — Substrate semantics and model contracts

**Goal:** Define the smallest coherent abstract model before implementing simulation behaviour.

**Work**
1. Specify entity identity, lifetime and state ownership.
2. Specify extensible property values and numerical state.
3. Define relationship identity, directionality, multiplicity, persistence and lifecycle.
4. Define recursive composition without a fixed hierarchy.
5. Define transformation contracts and conflict resolution.
6. Define state-transition atomicity and causal ordering.
7. Define configurable constraints, including conservation and compatibility models.
8. Specify deterministic random streams and seed ownership.
9. Define immutable model configuration and versioning.
10. Write invariants and property-based test cases.

**Deliverables:** substrate data model, transition contract, relationship/composition semantics, transformation semantics, configuration schema, determinism specification and architecture decision records for language/storage/runtime.

**Acceptance:** every primitive has precise operational meaning; biological concepts are unnecessary; conflicting transformations have defined resolution; recursive structures preserve identity and endpoint validity; invariants are independently testable; implementation and storage candidates are recorded without premature selection. Representative workload benchmarks are a Phase 3/8 deliverable, once executable prototypes exist.

**Exit gate:** reference model can be implemented without unresolved semantic ambiguity.

## Phase 3 — Deterministic reference runtime

**Goal:** Build a deliberately simple, trustworthy executable baseline.

**Work**
1. Implement entity and relationship storage.
2. Implement controlled state access and mutation.
3. Implement deterministic random streams.
4. Implement minimal scheduler and step boundary.
5. Implement canonical state serialization and hashing.
6. Implement snapshot save/load.
7. Implement deterministic replay for supported operations.
8. Add invariant checks and failure diagnostics.
9. Add reference fixtures and golden tests.
10. Benchmark without premature optimisation.

**Deliverables:** engine library, deterministic stepping API, serialization, snapshot/replay, unit/property/determinism tests and baseline performance report.

**Acceptance:** identical configuration and seed produce identical hashes in reference mode; snapshots round-trip; invalid transitions fail explicitly; lifecycle, relationships, recursive composition and ordering are tested; headless operation requires no UI.

**Exit gate:** reliable reference against which later optimisations can be measured.

## Phase 4 — General interaction and transformation system

**Goal:** Let novel configurations form through general rules rather than privileged biological operations.

**Work**
1. Define interaction eligibility and neighbourhood queries.
2. Implement pairwise and multi-party interactions.
3. Implement transformation proposals and conflict resolution.
4. Support creation, removal, duplication, fusion, fragmentation and reconfiguration as generic transformations.
5. Support relationship lifecycle.
6. Support recursive structures and emergent boundaries.
7. Add stochastic outcomes.
8. Add rule configuration and versioning.
9. Add interaction traces.
10. Test formation of persistent configurations without seeding them.

**Deliverables:** rule/interaction engine, transformation pipeline, relationship lifecycle, stochastic processes, event traces and composition experiments.

**Acceptance:** no organism/reproduction operation; structural changes use general contracts; deterministic mode is reproducible; ordering/conflicts are observable; aggregation, fragmentation and persistent relationships can be tested abstractly.

**Exit gate:** general interactions create and alter nontrivial structures with causal traces.

## Phase 5 — Environment and resource dynamics

**Goal:** Make persistence and propagation depend on a configurable external world.

**Work**
1. Implement 2D spatial domain and boundary conditions.
2. Implement spatial fields and gradients.
3. Implement resource replenishment/depletion.
4. Implement transport, diffusion and flow approximations.
5. Implement multiple resource forms and conversion rules.
6. Implement dissipation and selectable conservation constraints.
7. Implement waste/byproduct accumulation and reuse.
8. Implement temperature fields and thermal stochasticity.
9. Implement environmental drift, cycles and abrupt disturbances.
10. Record interventions as experiment events.

**Deliverables:** environment API, field storage, resource/transport modules, temperature/stochastic modules, intervention schedule and validation experiments.

**Acceptance:** field updates independently testable; accounting explicit per model; spatial and temporal resolution configurable; interventions reproducible; baseline definitions immutable.

**Exit gate:** controlled environments support comparable persistence and propagation trials.

## Phase 6 — Information transfer and evolutionary evidence

**Goal:** Detect whether general dynamics produce repeatable transfer, heritable variation and differential propagation.

**Work**
1. Define operational tests for repeatable state/organisation transfer.
2. Detect candidate successor relationships without assuming reproduction.
3. Measure transfer fidelity and variation.
4. Track ancestry/provenance as analytical metadata, not substrate primitives.
5. Measure propagation and persistence under matched conditions.
6. Define context-dependent performance measures.
7. Detect candidate selection dynamics.
8. Test cumulative changes across successive transitions.
9. Compare unstructured, structured and replicator-seeded conditions.
10. Test false positives, environmental reconstruction and non-heritable persistence.

**Deliverables:** transfer analysis, candidate lineage/provenance graph, variation/propagation measurements, performance measures, evidence reports and initial-condition comparisons.

**Acceptance:** distinguish environmental reconstruction from inheritance; measure rather than assume variation; separate propagation from longevity; require multi-step causal evidence for cumulative adaptation; analysis remains read-only.

**Exit gate:** identify and challenge candidate evolutionary processes without hardcoding them.

## Phase 7 — Observation, provenance and causal instrumentation

**Goal:** Make important outcomes inspectable, reproducible and explainable.

**Work**
1. Define append-only event and causal trace formats.
2. Instrument transformations, interactions and environmental changes.
3. Implement read-only queries and recursive inspection.
4. Implement snapshot/replay workflows.
5. Record configuration, code, seed, precision and approximation provenance.
6. Add branch-from-snapshot experiment histories.
7. Add anomaly capture and human annotations.
8. Add structured export.
9. Test observer non-interference.

**Deliverables:** event log, causal trace model, read-only API, snapshot/replay tooling, provenance metadata, exports and anomaly records.

**Acceptance:** saved runs reconstruct under declared execution mode; important changes trace to events/rules where instrumented; observers cannot mutate state; branches preserve provenance; missing causal data is explicit.

**Exit gate:** results can be inspected and replayed.

## Phase 8 — Performance and multiscale execution

**Goal:** Increase scale without losing scientific credibility.

**Work**
1. Establish representative workloads and performance budgets.
2. Benchmark dense, sparse and graph-oriented layouts.
3. Benchmark spatial indexing and hierarchical interaction ranges.
4. Compare sequential, parallel and event-driven execution.
5. Introduce process-specific precision.
6. Introduce distant/low-activity approximation tiers.
7. Introduce multiresolution representations with explicit conversion rules.
8. Quantify approximation error against reference execution.
9. Measure divergence in statistics, dynamics and causal histories.
10. Explore adaptive computational organisation as a separate mechanism.

**Deliverables:** benchmark suite, profiles, optimised paths, approximation contracts/error reports, execution comparison and adaptive-computation research note.

**Acceptance:** every optimisation has a workload and baseline; visual similarity is not sole validity criterion; error/divergence are reported; reference path remains available; modes declare reproducibility guarantees; performance does not silently change semantics.

**Exit gate:** defensible scale improvements with documented fidelity trade-offs.

## Phase 9 — Experimental orchestration and statistical controls

**Goal:** Turn individual runs into reproducible experiments.

**Work**
1. Define versioned experiment manifests.
2. Define matched controls and seed strategies.
3. Build trial-matrix execution.
4. Support immutable baselines and branches.
5. Add ablation and intervention protocols.
6. Add statistical summaries and uncertainty.
7. Track failed, interrupted and negative runs.
8. Add batch execution and resource budgeting.
9. Compare trajectories across initial-condition families.
10. Generate machine-readable reports.

**Deliverables:** manifest schema, trial runner, control/ablation support, statistical pipeline, comparison reports and reproducibility bundles.

**Acceptance:** experiments recreate from manifests; controls differ only in declared dimensions; seeds and execution settings are recorded; negative/incomplete results remain discoverable; claims report sample sizes and uncertainty.

**Exit gate:** controlled experiment families can be run and compared at scale.

## Phase 10 — Minimal scientific visualisation

**Goal:** Make the system understandable without turning the interface into the model.

**Work**
1. Select lightweight rendering stack based on benchmarks.
2. Render abstract entities, relationships and fields.
3. Add pan, zoom and selective inspection.
4. Add entity/relationship detail panels.
5. Add timeline and event navigation.
6. Add metrics and performance/approximation indicators.
7. Add snapshot, replay and branch controls.
8. Add side-by-side experiment comparison.
9. Distinguish observed state from analytical classification.
10. Ensure rendering rate does not affect simulation semantics.

**Deliverables:** world viewer, inspection panels, timeline/replay, dashboard and comparison view.

**Acceptance:** UI reads through observation APIs; rendering cannot mutate experiments; headless mode works; UI slowdown cannot silently change time/order; analytical labels are distinct from substrate properties.

**Exit gate:** users can inspect experiments without compromising the engine.

## Phase 11 — Research programme and discovery cycles

**Goal:** Conduct the first serious experimental investigation of cumulative adaptation.

**Work**
1. Establish baseline initial-condition families.
2. Run unstructured, structured and replicator-seeded trials.
3. Compare environmental regimes.
4. Measure persistent configurations and repeatable transfer.
5. Investigate heritable variation and differential propagation.
6. Test cumulative adaptation with matched controls.
7. Search for alternative organisational and inheritance pathways.
8. Investigate collapse, simplification, extinction and re-emergence.
9. Preserve surprises and formulate hypotheses.
10. Publish reproducible bundles and negative findings.

**Deliverables:** dataset, experiment reports, candidate discoveries/counterexamples, reproducible bundles and revised research agenda.

**Acceptance:** claims have reproducible evidence; competing explanations are tested; anomalies and negative results are preserved; observation, inference and speculation are distinguished.

**Exit gate:** at least one complete experimental cycle has been independently reconstructed.

## Phase 12 — Validation, hardening and release

**Goal:** Produce a stable, scientifically defensible research platform.

**Work**
1. Audit hidden category leakage.
2. Audit observer non-interference.
3. Audit replay and provenance.
4. Re-run benchmarks and statistical validation.
5. Document limitations and unsupported models.
6. Review experiment artifact integrity.
7. Stabilise APIs and file formats.
8. Produce setup, operation and reproduction guides.
9. Package reference experiments and analyses.
10. Tag a release with a reproducibility checklist.

**Deliverables:** stable release, scientific-method documentation, reproduction guide, benchmark report, limitations register and reference bundles.

**Acceptance:** new users can reproduce reference experiments; claims trace to artifacts; versions are declared; limitations are explicit; CI validates release state.

**Exit gate:** platform is ready for sustained research and external scrutiny.

---

## Cross-phase standards

### Scientific integrity
- Keep substrate, dynamics, analysis and presentation separate.
- Treat classifications as hypotheses.
- Record assumptions and constraints.
- Preserve negative results and anomalies.
- Use matched controls for causal claims.
- Distinguish evidence from interpretation.

### Software quality
- Prefer small, testable contracts over broad abstractions.
- Avoid dependencies until they solve a demonstrated problem.
- Keep a correct reference implementation.
- Make state transitions explicit.
- Test determinism and invariants continuously.
- Benchmark before optimising.

### Performance measurements
Track active entity/configuration count, relationship count, interaction evaluations per simulated unit, state transitions per second, simulated time per wall-clock second, memory per entity/relationship, event-log and snapshot overhead, approximation error, trajectory divergence and independent trials per wall-clock time.

### Architecture decision records
Record problem/constraints, options, evidence or benchmark, decision/rationale, scientific and computational trade-offs, and revisit conditions. Decisions without measurements are provisional.

### Phase management
A phase is complete only when deliverables, tests, evidence and exit criteria are satisfied. Unfinished work remains explicitly open.

## Immediate work after Phase 1

1. Define precise substrate semantics.
2. Compare implementation-language and storage-layout candidates with representative workloads.
3. Decide deterministic execution and state-transition contracts.
4. Write Phase 2 model tests before building the full runtime.
5. Record architecture decisions with measured evidence.
