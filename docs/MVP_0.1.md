# MVP 0.1 — Interactive abstract dynamics

**Status:** Deployed to GitHub Pages. Native Rust CI and the WebAssembly Pages build have passed for the current implementation.

## Purpose

Provide the first usable, public, end-to-end experiment surface. This MVP demonstrates configurable abstract dynamics and read-only observation. It does **not** claim to demonstrate evolution, inheritance, selection or adaptation.

## Included

- Rust simulation core compiled to WebAssembly.
- Browser interface hosted on GitHub Pages.
- Configurable initial entity count and random seed.
- Two continuous scalar state channels per entity.
- Local, distance-weighted pairwise exchange.
- Stochastic perturbation.
- Periodic 2D positions with channel-dependent movement.
- Pause/run, fixed-step execution, reset and deterministic 100-step replay.
- Read-only summary metrics and diagnostic state hash.
- Exportable JSON run manifest recording seed, model, parameters, transition count, runtime/build version and state/replay hashes.
- Versioned append-only event log containing the initial snapshot and every committed proposal batch.
- In-browser replay verification and event-log JSON export.
- CI validation of native Rust and browser WebAssembly builds.

## Model definition

Each abstract entity has an identity implicit in its stable vector position, a 2D coordinate and two scalar state values in [0, 1]. The world is periodic in both spatial dimensions. For each transition, every pair within the configured interaction radius exchanges a fraction of the difference between each scalar channel. The exchange weight decreases linearly with squared distance. Independent pseudorandom perturbations are applied to both channels. The resulting channel values influence small positional displacements. There is no creation, retirement, duplication, fusion, fitness, lineage, environmental resource field or reproduction mechanism in this MVP.

This is intentionally a narrow dynamics testbed, not a substitute for the complete specification. The initial implementation uses an O(n²) pair scan and caps the initial population at 2,000. The browser UI caps the population at 500 to keep the quadratic pair scan practical on mobile devices.

## Reproducibility scope

The random stream is deterministic for the same compiled runtime, seed and execution path. The browser experiment now runs its state changes through the general entity/property substrate and records each committed proposal batch in the versioned event log. The interface exposes replay verification, event-record counts and full event-log export. The exported manifest includes the deployed build commit, random generator identifier, parameters, terminal hash and replay hash. Cross-version, cross-browser and cross-architecture bitwise reproducibility are not claimed. The hash is a diagnostic checksum, not a cryptographic integrity mechanism.

The replay control reconstructs the initial state from the displayed seed and settings and runs exactly 100 transitions. The verify control replays the recorded proposal batches from the versioned initial snapshot and compares the resulting state hash with the live state. Changing parameters after a run changes the live model settings; reset is required to apply a new initial population or seed.

## Run locally

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --target web --release --out-dir site/pkg
python -m http.server 8000 --directory site
```

Open http://localhost:8000. A local HTTP server is required because browser module loading does not work from file://.

## MVP exit criteria

- Native Rust tests pass.
- wasm-pack build succeeds for wasm32-unknown-unknown.
- Pages build and deployment complete.
- Live page loads the generated WASM module.
- Run, pause, step, reset, replay and manifest export controls work.
- Same seed/configuration/replay path produces the same terminal state hash.
- Limitations are visible to users and no evolutionary claims are made.

## Next scientific work

1. Move the browser MVP onto the same general entity/relationship substrate contracts rather than maintaining a parallel specialised representation.
2. Extend the general runtime beyond property updates with generic entity/relation creation, retirement, endpoint changes and explicit transformation constraints.
3. Add environment fields and resource accounting as configurable model modules.
4. Add snapshot/replay provenance and controlled multi-seed trials.
5. Add observational measurements for persistent configurations and repeatable state transfer.
6. Only then investigate candidate inheritance and differential propagation.
