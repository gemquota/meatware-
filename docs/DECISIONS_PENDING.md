# Decisions Pending

This register prevents unresolved implementation choices from becoming accidental assumptions.

| Topic | Current position | Evidence needed | Target phase |
|---|---|---|---|
| Implementation language | Open | Prototype representative workloads in at least two candidates | 2–3 |
| Runtime/toolchain | Open | Build, test, profiling and deployment comparison | 2–3 |
| Physical storage | Logical dynamic graph; layout open | Dense/sparse/adjacency/hybrid benchmarks | 3, 8 |
| Memory allocation | Open | Allocation rate, fragmentation and peak-memory profiles | 3, 8 |
| Numeric precision | Per-process policy; exact formats open | Error, stability and throughput tests | 3, 8 |
| Random generator | Explicit keyed streams required; algorithm open | Statistical quality, speed and replay tests | 3 |
| Parallel execution | Multiple models to compare | Determinism, throughput and divergence tests | 8 |
| Scheduler | Hybrid/multiscale semantics required; implementation open | Event density and timescale workload tests | 3, 8 |
| Spatial index | Open | Neighbourhood density and movement benchmarks | 5, 8 |
| Serialization | Canonical state required; wire format open | Size, speed, compatibility and migration tests | 3, 7 |
| Causal history retention | Full causal trace desired; storage policy open | Trace volume and query workloads | 7 |
| Visual approximation acceptance | Visual similarity is useful for initial screening only | Separate perceptual and dynamical/statistical criteria | 8, 10 |

## Interpretation of visual similarity

The Round 6 preference to accept optimisation when visual results look similar is retained as a useful rapid screening criterion for presentation and exploratory previews. It is not treated as proof that simulation dynamics are equivalent. Any optimisation used for scientific claims must additionally report relevant statistical, dynamical and causal divergence against the reference path. This distinction prevents a renderer from becoming the scientific oracle.
