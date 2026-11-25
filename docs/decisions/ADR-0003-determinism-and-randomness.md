# ADR-0003: Explicit Random Streams and Scoped Determinism

**Status:** Accepted  
**Phase:** 2

## Context

Stochastic behaviour is required, but global random state and incidental call order make results fragile under refactoring, parallelism and instrumentation.

## Decision

Randomness is supplied through explicit model-owned streams identified by stable run, subsystem and event keys. Generator algorithm/version, stream roots and numeric policy are recorded in the run manifest. Determinism is a declared contract, not an unconditional cross-platform promise.

## Consequences

- Instrumentation cannot consume hidden random draws.
- Independent events can avoid shifting each other's random sequences.
- Replays can reproduce runs when execution and numeric contracts match.
- Parallel implementations need stable event keys and deterministic reductions.
- Statistical equivalence must not be mislabeled bitwise determinism.

## Alternatives considered

1. **Ambient global PRNG:** rejected because call order and instrumentation affect unrelated outcomes.
2. **One stream per worker:** insufficient because worker scheduling changes assignment.
3. **Explicit keyed streams:** accepted.

## Revisit when

Benchmarking demonstrates that keyed stream management is too expensive or a selected generator cannot support the required reproducibility mode. Any replacement must preserve stream ownership and declared guarantees.
