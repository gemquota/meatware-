# Decisions Pending

This register prevents unresolved implementation choices from becoming accidental assumptions.

| Topic | Current position | Evidence needed | Target phase |
|---|---|---|---|
| Implementation language | Rust selected for reference runtime; C++ retained as benchmark baseline | Broader workload and portability evidence may revisit | 3, 8 |
| Physical storage | Ordered maps in reference runtime; optimised layout remains open | Dense/sparse/adjacency/hybrid benchmarks | 3, 8 |
| Memory allocation | Standard Rust allocation initially | Allocation rate, fragmentation and peak-memory profiles | 3, 8 |
| Numeric precision | f64 in initial portable float values; process-specific policy remains open | Error, stability and throughput tests | 3, 8 |
| Random generator | Versioned SplitMix64 reference stream | Statistical quality, speed and replay tests | 3 |
| Parallel execution | Sequential reference path | Determinism, throughput and divergence tests | 8 |
| Scheduler | Synchronous logical boundaries in reference runtime | Event density and timescale workload tests | 3, 8 |
| Spatial index | Open | Neighbourhood density and movement benchmarks | 5 |
| Serialization | Ordered JSON snapshot prototype | Size, speed, compatibility and migration tests | 3, 7 |
| Causal history retention | Full causal trace desired; storage policy open | Trace volume and query workloads | 7 |
| Visual approximation acceptance | Visual similarity is useful for initial screening only | Separate perceptual and dynamical/statistical criteria | 8, 10 |

## Interpretation of visual similarity

The Round 6 preference to accept optimisation when visual results look similar is retained as a useful rapid screening criterion for presentation and exploratory previews. It is not treated as proof that simulation dynamics are equivalent. Any optimisation used for scientific claims must additionally report relevant statistical, dynamical and causal divergence against the reference path.
