# Transition and Conflict-Resolution Contract

**Status:** Phase 2 contract  
**Version:** 1.0

This document defines how proposed changes become a new configuration. It deliberately avoids prescribing a programming language, storage layout or scheduler implementation.

## 1. State transition model

Let a completed configuration be (S_n), identified by run and logical boundary (n). A transition evaluates a set of proposals against a declared source configuration and commits an accepted, conflict-resolved batch:

[
S_{n+1} = operatorname{Commit}(S_n, operatorname{Resolve}(operatorname{Evaluate}(S_n, P_n)))
]

Where:
- (P_n) is the set of proposals generated for boundary (n);
- evaluation is read-only with respect to (S_n);
- resolution is deterministic under the declared resolution policy;
- commit is atomic;
- the next completed configuration is (S_{n+1}).

A scheduler may use different internal phases or event times, but it must expose an equivalent causal ordering and atomic commit boundary.

## 2. Proposal structure

A proposal contains:

| Field | Meaning |
|---|---|
| proposal key | Stable unique key within a source boundary |
| source boundary | Configuration against which it was evaluated |
| reads | Entity/relation/property versions or predicates relied upon |
| writes | Property assignments and removals |
| creates | New entities and relations with initial state |
| retires | Existing entities or relations to retire |
| endpoint edits | Explicit relation endpoint changes |
| preconditions | Predicates that must still hold at resolution |
| causal references | Inputs, events and rule identifiers that motivated it |
| priority | Optional model-defined priority, never an implicit universal ranking |

Proposal keys MUST be unique within a boundary. They MUST be independent of incidental iteration order.

Proposals are data. Evaluating a proposal cannot directly mutate the source configuration.

## 3. Read dependencies and stale proposals

A proposal declares the state it depended on. The resolver checks those dependencies against the source configuration and any already accepted edits according to the selected policy.

A dependency may refer to:
- an item's continued active existence;
- a specific property version/value;
- a relation endpoint set;
- a model-level predicate;
- an environmental field or resource value.

A proposal whose required dependency is invalid MUST be rejected or re-evaluated under an explicitly selected policy. Silent application to stale state is prohibited.

## 4. Conflict classes

At minimum, the resolver identifies:

1. **Write/write:** multiple proposals assign different values to the same property.
2. **Write/remove:** one proposal writes a property another removes.
3. **Retire/write:** one proposal retires an item another modifies.
4. **Retire/reference:** retirement leaves a relation with an invalid endpoint.
5. **Create/identity:** multiple proposals request the same identifier.
6. **Endpoint conflict:** proposals make incompatible edits to one relation.
7. **Constraint conflict:** a combined batch violates a declared model constraint.
8. **Read/write:** a proposal's assumptions are invalidated by another accepted proposal.

Conflict detection is explicit and produces a record. The implementation must not silently fall back to last-write-wins.

## 5. Resolution policies

The model configuration selects a policy per conflict class or transformation domain. Supported policy families:

- **Reject-all-conflicting:** reject every proposal in a conflict set.
- **Stable-priority:** order by an explicitly supplied model priority, then stable proposal key.
- **Commutative reducer:** combine values through a declared associative/commutative operation with specified numeric semantics.
- **Exclusive reservation:** first accepted proposal reserves the affected resource under a declared stable ordering.
- **Model-specific resolver:** a versioned deterministic resolver with explicit tests and provenance.

There is no universal best policy. The policy itself changes system behaviour and is therefore part of the experiment configuration.

If a policy cannot resolve a conflict deterministically under the active execution contract, the batch MUST fail rather than commit an order-dependent result.

## 6. Atomicity and commit

The resolver constructs a complete candidate result without exposing partial changes. It validates identity uniqueness, endpoint validity, property types, model constraints and all required preconditions before commit.

Commit either:
- applies the complete accepted batch and advances the boundary; or
- applies none of it and records a failed transition.

There is no partially committed transition.

A failed transition records the source boundary, proposal identifiers, conflict/validation failures and relevant model version. Failure does not silently advance logical time unless the scheduler explicitly records a separate no-op boundary.

## 7. Creation and retirement

Created items receive fresh identifiers. Identifiers are allocated deterministically from stable proposal keys or an equivalent replay-safe mechanism. A proposal cannot choose an identifier that collides with an active or retired identifier.

New items become visible only after commit. Under the reference contract they do not participate in proposals generated from the source boundary.

Retirement is explicit. The resolver applies the endpoint-retirement policy of every affected relation schema. It never silently leaves dangling active endpoints.

## 8. Causal order and event records

Every accepted edit is associated with:
- source boundary;
- proposal key;
- rule/model version;
- read dependencies;
- conflict decisions;
- constraint checks;
- created/modified/retired item identifiers.

A causal record is not required to claim philosophical causality. It documents the declared computational dependencies and ordering that produced a state change.

## 9. Determinism

The reference deterministic contract requires:
- stable proposal keys;
- stable ordering of proposal and conflict sets;
- explicit random streams;
- canonical property/map ordering;
- deterministic identifier allocation;
- deterministic reducers;
- declared floating-point and numeric policies;
- no dependency on thread scheduling or hash-map iteration order.

Parallel implementations may produce the same result only if they preserve this contract. Otherwise they declare a weaker reproducibility mode and are compared against the reference statistically and dynamically.

## 10. Observer isolation

Observers read completed configurations and immutable event records. They cannot submit proposals through an observation interface. Deliberate interventions use a separate control channel, are versioned, and become part of the experiment record.

## 11. Minimum conformance scenarios

1. Independent property writes both commit.
2. Conflicting writes trigger the selected conflict policy.
3. Write/remove conflicts are recorded and resolved explicitly.
4. Retirement cannot leave a dangling active relation.
5. Duplicate creation identifiers fail deterministically.
6. A failed batch leaves the source configuration unchanged.
7. Created items are not eligible in the source boundary.
8. Reordered input iteration does not alter deterministic output.
9. Same manifest and seed reproduce canonical state hashes.
10. Observation cannot mutate the returned or live state.
