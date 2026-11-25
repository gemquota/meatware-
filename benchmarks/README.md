# Candidate workload

This Phase 3 workload compares C++ and Rust implementations of the same sparse graph update.

- 200,000 state-bearing items.
- 1,200,000 directed relations in CSR form.
- Six outgoing relations per item.
- Eight synchronous state-update steps.
- Competing proposals sorted by stable target/key and resolved deterministically.
- Checksum parity is required; timing is informational.

Run from the repository root:

```sh
g++ -O3 -DNDEBUG -std=c++20 benchmarks/cpp/main.cpp -o /tmp/meatware-cpp
/tmp/meatware-cpp > /tmp/cpp.json
cargo run --release --manifest-path benchmarks/rust/Cargo.toml > /tmp/rust.json
python benchmarks/compare.py /tmp/cpp.json /tmp/rust.json
```

This is a narrow candidate workload, not a complete performance ranking. It does not yet measure spatial queries, environmental fields, serialization, stochastic scheduling or large-scale graph mutation. Repeat measurements on controlled hardware before selecting a language.
