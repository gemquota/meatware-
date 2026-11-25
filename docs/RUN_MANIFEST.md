# Run manifest schema v1

**Format identifier:** `meatware-run-manifest`  
**Schema version:** `1`

A run manifest is a compact description of the inputs and terminal verification state of one execution. It is not a substitute for the event log: the manifest describes a run, while the event log contains the transitions required to replay it.

## Fields

| Field | Meaning |
|---|---|
| `format`, `schema_version` | Format discriminator and schema version |
| `runtime` | Runtime implementation/version identifier |
| `build_commit` | Source commit used to build the executable |
| `model` | Explicit model/rule-set identifier |
| `random_generator` | PRNG algorithm/version |
| `random_stream_scheme` | How the run consumes or derives random streams |
| `seed` | Initial seed |
| `initial_entities` | Entity count in the initial configuration |
| `parameters` | Model parameters as typed values |
| `conflict_policy` | Transition write-conflict resolver |
| `initial_state_hash` | Diagnostic hash of the initial configuration |
| `transitions` | Number of committed logical transitions |
| `event_records` | Number of recorded transition batches |
| `terminal_state_hash` | Diagnostic hash of the final live state |
| `replay_state_hash` | Diagnostic hash after replaying the event log |
| `determinism` | Human-readable statement of the verification performed |

## Browser MVP v0.1 values

The current browser model records:

- PRNG: `splitmix64-v1`
- Stream scheme: `single-sequential-stream-v1`
- Conflict policy: `Reject`
- Model: `two-channel-local-exchange-v1`

The browser currently uses one sequential stream for initialization and subsequent perturbations. The runtime also exposes event-local stream derivation, but the browser MVP does not yet use it. The manifest deliberately records the stream scheme actually used rather than implying otherwise.

## Integrity and compatibility

The manifest parser rejects unknown format identifiers and schema versions. It is not cryptographically signed. State hashes use the runtime's diagnostic FNV-1a 64-bit encoding and must not be treated as tamper-proof evidence. Reproducibility still depends on retaining the matching runtime, model implementation, event log and execution semantics.

The event log records the conflict policy per transition. Older records that omit the field are interpreted using the historical default, `Reject`.

## Example

```json
{
  "format": "meatware-run-manifest",
  "schema_version": 1,
  "runtime": "meatware-wasm-0.1.0",
  "build_commit": "source-commit-sha",
  "model": "two-channel-local-exchange-v1",
  "random_generator": "splitmix64-v1",
  "random_stream_scheme": "single-sequential-stream-v1",
  "seed": 17,
  "initial_entities": 320,
  "parameters": {
    "coupling": 0.8,
    "noise": 0.008,
    "interaction_radius": 0.055
  },
  "conflict_policy": "Reject",
  "initial_state_hash": "diagnostic-hash",
  "transitions": 100,
  "event_records": 100,
  "terminal_state_hash": "diagnostic-hash",
  "replay_state_hash": "diagnostic-hash",
  "determinism": "event-log replay checked against terminal state"
}
```
