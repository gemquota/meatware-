# Substrate Model Contract

**Status:** Phase 2 contract  
**Version:** 1.0  
**Normative terms:** MUST, MUST NOT, SHOULD and MAY describe conformance requirements.

This document defines the abstract substrate independently of any programming language or storage engine. It specifies what a conforming implementation means, not how it stores bytes.

## 1. Scope and exclusions

The substrate represents distinguishable state-bearing items and relations between them. It does not represent biological categories. There are no built-in organisms, cells, genes, species, genomes, membranes, metabolisms, reproduction cycles, fitness values or evolutionary operations.

Analysis may infer categories from observations. Such categories MUST remain outside substrate identity, state and transition rules unless a separately versioned experiment deliberately introduces a neutral label as ordinary data.

## 2. Core vocabulary

- **Run:** one execution under one immutable model configuration, initial state and random-stream root.
- **Entity:** an individually addressable state-bearing item.
- **Relation:** an individually addressable, state-bearing association between one or more endpoints.
- **Endpoint:** a typed reference to an entity or relation.
- **Property:** a named value attached to an entity or relation.
- **Configuration:** the complete set of active entities, active relations and model-owned environmental state at a logical boundary.
- **Transformation:** a proposed change from one configuration to another.
- **Transition batch:** a set of transformations resolved and committed as one atomic boundary.
- **Observation:** a read-only description or measurement of a configuration.
- **Model:** a versioned selection of rules, constraints, numerical policies and environmental processes.

A configuration is not required to have a single root, a tree structure, a designated inside/outside, or a single level of organisation.

## 3. Identity and lifetime

3.1 Every entity and relation MUST have a unique, opaque identifier within its run.

3.2 Identifiers MUST NOT encode category, position, ancestry, size, creation order or implementation storage location.

3.3 An identifier MUST NOT be reused after retirement within the same run. A new item created from an old item receives a new identity even when its properties are identical.

3.4 Identity is distinct from state equality. Two items with equal properties are still distinct items.

3.5 Retirement removes an item from the active configuration. Historical records MAY retain its identifier and final state. Retired identifiers MUST NOT resolve as active endpoints.

3.6 Cross-run identity is not implied. Replaying the same run with the same reproducibility contract SHOULD reproduce identifiers. Comparing separate runs uses explicit correspondence or statistical matching, not assumed identity equality.

3.7 A relation has its own identity and may have properties and participate as an endpoint in another relation. This permits recursive relational organisation without introducing a privileged composite-object class.

## 4. State and property values

4.1 Entities and relations each own an extensible property map. Property keys are namespaced strings; values are typed data.

4.2 The portable value domain is:
- null;
- boolean;
- signed integer;
- finite floating-point number;
- UTF-8 string;
- ordered sequence of portable values;
- map from string keys to portable values.

4.3 Implementations MAY provide additional value types through versioned extensions. An extension MUST define validation, equality, canonical encoding, copying and numerical behaviour. Executable code, opaque process pointers and unserialisable runtime objects MUST NOT be stored as portable state.

4.4 Floating-point NaN and infinities are invalid in the portable core. Signed zero handling MUST be specified by the selected numeric policy. NaN-like semantics, if needed, belong in an explicit extension.

4.5 Missing property and property set to null are distinct states.

4.6 Property names have no intrinsic semantics. A model may interpret a property named `charge`, `temperature` or `capacity`, but the substrate does not assign those names universal behaviour.

4.7 State may be discrete, continuous, vector-valued, relational or mixed. No fixed number of dimensions or property catalogue is imposed.

4.8 Property maps are logically unordered. Sequence values are ordered. Canonical serialization sorts map keys by a declared bytewise ordering.

## 5. Relations and recursive organisation

5.1 A relation has:
- a unique relation identifier;
- a relation-kind label that is ordinary model data, not a biological type;
- one or more ordered or unordered endpoints as declared by the relation schema;
- its own property map;
- an active or retired lifecycle state.

5.2 A relation schema declares endpoint count constraints, endpoint ordering, allowed endpoint reference kinds and whether duplicate endpoints are meaningful. Schemas are model data and versioned.

5.3 Endpoints may refer to entities or relations. Relation-to-relation references allow recursive and nested organisation. Cycles are permitted unless a specific relation schema declares an acyclic constraint.

5.4 There is no universal parent pointer, owner, root or containment tree. A model may define ownership-like semantics through an explicit relation schema, but those semantics do not become substrate-wide rules.

5.5 Spatial proximity does not imply a persistent relation. A model may derive temporary interaction candidates from spatial queries. Persistent relations exist only when explicitly created through a transformation.

5.6 A relation may persist after its endpoints move apart. Endpoint retirement policy is explicit per relation schema: reject retirement while referenced, cascade retirement, detach/rewrite, or preserve a historical-only relation. No implicit policy is allowed.

5.7 A relation may carry state that affects future transformations. This is a general property of state-bearing relations, not a special capability.

## 6. Composition and boundaries

6.1 Composition is represented by entities, relations and their state. No separate biological composite type is required.

6.2 A configuration may be partitioned analytically into overlapping subsets. A subset is not automatically an entity or an owner.

6.3 Emergent boundaries are observations about differences in connectivity, state, interaction rates, persistence or other measured properties. The substrate does not create a boundary object unless a model's ordinary transformation rules create one.

6.4 Nested, overlapping, distributed and cyclic arrangements are valid unless a selected model constraint disallows them.

6.5 A composite pattern may exhibit behaviour not present in isolated components. Such behaviour is measured from transitions, not declared by its label.

## 7. Time and configuration boundaries

7.1 The reference semantics use a monotonically increasing logical boundary index. The index is an ordering coordinate, not a claim that every process has one physical timescale.

7.2 A configuration is observed at a completed boundary. A transition batch reads a defined source configuration and produces a new completed configuration.

7.3 Continuous-time values, local clocks, asynchronous events and hybrid schedules are model-level scheduling extensions. They MUST map their commits to an explicit causal order.

7.4 A transition batch is atomic at the configuration level: observers see either the source configuration or the committed result, never a partially applied batch.

7.5 The scheduler MUST declare whether newly created items can participate in the same boundary. The reference contract defaults to no: created items first become eligible at the next boundary.

## 8. Transformations

The substrate does not expose a privileged operation for reproduction, evolution, adaptation or organism formation. It exposes generic state transformations.

A transformation proposal contains:
- a stable proposal key within the source boundary;
- the source configuration identifier or boundary;
- read dependencies;
- zero or more property writes/removals;
- zero or more entity/relation creations;
- zero or more entity/relation retirements;
- explicit relation endpoint changes;
- declared preconditions;
- causal references to the observations/events that motivated the proposal.

Creation, retirement, duplication, fusion, fragmentation, exchange and reconfiguration are patterns expressible through these generic edits. They are not mandatory named domain operations.

A proposal MUST NOT mutate live state while it is being constructed or evaluated.

## 9. Randomness and determinism

9.1 Randomness is supplied by explicit model-owned streams, not ambient global state.

9.2 A stream is identified by a stable key derived from run identity, model subsystem identity and an explicit stream key. The exact generator algorithm and version are part of the run manifest.

9.3 Event-local random draws SHOULD be keyed so unrelated scheduling or parallel execution does not silently shift another event's random sequence.

9.4 Deterministic mode requires a declared execution contract. Identical initial state, model version, stream roots, numerical policy and ordered inputs MUST reproduce the same canonical state hashes under that contract.

9.5 Cross-platform bitwise equality is not promised until numeric representation, transcendental functions and execution order are specified. Statistical equivalence is a different, weaker claim and must be labelled as such.

## 10. Model constraints

Conservation, boundedness, compatibility, geometry, energy accounting, thermal behaviour and other physical-like restrictions are selectable model constraints.

Each constraint declares:
- its scope;
- the state quantities it reads;
- the transformations it permits or rejects;
- its numerical tolerance or exactness policy;
- whether it is local or global;
- how violations are reported.

The substrate does not impose a universal conservation law. A selected model may impose one or more explicit constraints.

## 11. Observation and non-interference

Observation APIs are read-only. An observation may return copies, immutable views or serialized values. It MUST NOT expose mutable references into live simulation state.

Analysis may annotate external records with classifications, hypotheses, confidence and provenance. It MUST NOT silently write those annotations into substrate properties.

Instrumentation may consume resources and affect wall-clock performance. It MUST NOT alter model state, random streams, logical ordering or transformation eligibility. Any deliberate instrumentation intervention is an explicit experiment intervention and is recorded as such.

## 12. Canonical state and equality

Canonical state encoding includes active entity IDs and properties, active relation IDs, schemas, endpoints and properties, plus all model-owned state required to continue the run.

Canonical encoding excludes presentation-only labels, transient UI state and analytical classifications unless those are explicitly model-owned inputs.

Two configurations are exactly equal only when their canonical encodings are equal under the same model and encoding version. Structural or statistical similarity is a separate analysis result.

## 13. Conformance checklist

A conforming substrate implementation MUST demonstrate:
- opaque, non-reused identities;
- extensible typed properties;
- first-class state-bearing relations;
- relation endpoints that can reference relations;
- explicit lifecycle and endpoint-retirement policies;
- atomic transition batches;
- declared conflict resolution;
- explicit random streams;
- canonical state encoding;
- read-only observation;
- no biological primitives or privileged evolutionary operations.
