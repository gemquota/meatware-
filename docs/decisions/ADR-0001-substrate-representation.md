# ADR-0001: Logical Substrate Representation

**Status:** Accepted for logical semantics; physical implementation deferred  
**Phase:** 2

## Context

The experiment requires extensible state, recursive organisation, persistent relations, overlapping configurations and structures that may not fit a predefined hierarchy. A fixed object taxonomy or parent/child tree would silently privilege one organisational form.

## Decision

Represent the logical substrate as a dynamic graph-like system of state-bearing entities and state-bearing relations. Relation endpoints may refer to entities or relations. Properties are extensible typed maps. No universal root, owner or containment tree exists.

## Consequences

- Recursive and cyclic relational structures are representable.
- Relations can persist and carry state independently of endpoint proximity.
- A composite is an observed configuration, not a privileged object category.
- Physical storage may use arrays, sparse tables, adjacency structures, arenas or hybrids.
- Graph semantics do not require pointer-heavy object graphs.

## Alternatives considered

1. **Fixed typed object hierarchy:** rejected because it encourages hardcoded categories and makes new organisational forms expensive.
2. **Tree with parent ownership:** rejected because it excludes overlapping, distributed and cyclic organisation.
3. **Pure untyped edge list:** insufficient on its own because identity, endpoint semantics and relation-owned state still require explicit contracts.
4. **Dynamic graph semantics with replaceable physical layout:** accepted.

## Revisit when

Representative Phase 3/8 workloads show that the logical contract imposes unacceptable storage or query costs. Revisit physical layout without changing logical semantics unless a versioned experiment explicitly changes the model.
