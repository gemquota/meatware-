# Phase 4: General Interaction and Transformation System

**Status:** In progress  
**Current increment:** Deterministic interaction scheduling and proposal generation  
**Goal:** Allow configurable rules to inspect a common source state and propose general structural or state changes without privileged domain operations.

## Design boundary

The interaction layer is a dynamics module above the substrate. It may decide which configurations to inspect and what proposals to emit, but it cannot mutate the source configuration directly. Every accepted change still passes through the existing atomic transition contract and event log.

A rule is identified by a stable numeric key and explicit version. Its evaluator receives:
- An immutable source configuration.
- A stable, unordered participant combination.
- An event-local deterministic random stream.

It may return no proposal or one ordinary proposal containing read dependencies and generic edits. The scheduler assigns the proposal key and source boundary; evaluators cannot choose either. Proposals are sorted before they reach the transition engine.

## Implemented

- [x] Stable participant enumeration across entity and relation identities.
- [x] Pairwise and arbitrary fixed-arity participant combinations.
- [x] Explicit per-rule evaluation budgets.
- [x] Stable rule ordering and duplicate rule-key rejection.
- [x] Rule key/version included in deterministic proposal identity.
- [x] Serializable rule descriptors capture the scheduler parameters and version metadata; executable evaluator identity remains tied to the build.
- [x] Event-local random stream isolated by seed, boundary, proposal identity and rule channel.
- [x] Evaluators inspect one immutable source state; no in-place mutation path is exposed.
- [x] Generated changes use ordinary proposals and the existing atomic transition validator.
- [x] Relationship endpoint sets can be replaced through atomic edits with source-state endpoint read dependencies.
- [x] Endpoint replacement participates in write-conflict detection and rejects empty or dangling endpoint sets.
- [x] Tests for pairwise relation formation, reproducibility, source immutability, evaluation budgets, invalid rule definitions and endpoint-read/reconfiguration atomicity.

## Important limitations

This is a first scheduler, not yet a complete interaction language. Rule evaluators are Rust function pointers. Their executable implementation is identified by the build and rule key/version; serializable descriptors preserve scheduler parameters, but evaluator implementations are not yet a portable declarative rule language. Candidate generation currently enumerates combinations over all selected entities and/or relations; spatial neighbourhoods, eligibility predicates, directed permutations and efficient indexed candidate queries remain future work. Each rule has an explicit evaluation cap to prevent accidental combinatorial explosions, but reaching that cap is not currently emitted as a separate diagnostic.

The evaluator returns proposals, not accepted outcomes. Conflicts, invalid edits, stale reads and dangling endpoints are still resolved or rejected by the substrate transition engine. No proposal is evidence of a successful transformation until the transition commits.

## Verification

The Rust test suite is the authoritative check for this increment. CI must verify native compilation and tests before this work is treated as integrated. Browser MVP behaviour is unchanged; this increment does not claim to add visible interaction dynamics to the deployed page.

## Next increments

1. Add a portable declarative evaluator format only if experiments demonstrate that compiled evaluators are too restrictive.
2. Add eligibility predicates and indexed local-neighbourhood candidate generation.
3. Add directed participant ordering and multi-party interaction experiments.
4. Extend generic duplication/fusion/fragmentation compositions.
5. Add interaction and proposal outcome traces to the event log.
6. Add stochastic outcome and scheduler diagnostics, including budget exhaustion.
7. Build a headless composition experiment using the same scheduler and event log.
