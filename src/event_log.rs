use crate::{Configuration, Proposal, RuntimeError};
use serde::{Deserialize, Serialize};

pub const SNAPSHOT_SCHEMA_VERSION: u32 = 1;
pub const EVENT_LOG_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotEnvelope {
    pub format: String,
    pub schema_version: u32,
    pub state: Configuration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransitionRecord {
    pub schema_version: u32,
    pub source_boundary: u64,
    pub target_boundary: u64,
    pub source_hash: u64,
    pub target_hash: u64,
    pub proposals: Vec<Proposal>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventLog {
    pub format: String,
    pub schema_version: u32,
    pub initial_snapshot: Vec<u8>,
    pub records: Vec<TransitionRecord>,
}

#[derive(Debug)]
pub enum EventLogError {
    Runtime(RuntimeError),
    InvalidFormat(String),
    BoundaryMismatch { expected: u64, actual: u64 },
    HashMismatch { boundary: u64, expected: u64, actual: u64 },
    InvalidRecord(String),
    Serialization(String),
}

impl From<RuntimeError> for EventLogError {
    fn from(value: RuntimeError) -> Self { Self::Runtime(value) }
}

impl std::fmt::Display for EventLogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for EventLogError {}

impl EventLog {
    pub fn new(initial: &Configuration) -> Result<Self, EventLogError> {
        initial.validate()?;
        Ok(Self {
            format: "meatware-event-log".into(),
            schema_version: EVENT_LOG_SCHEMA_VERSION,
            initial_snapshot: initial.snapshot()?,
            records: Vec::new(),
        })
    }

    /// Apply a transition to a private candidate first. The live state and log are
    /// updated together only after the candidate passes every runtime invariant.
    pub fn append(
        &mut self,
        state: &mut Configuration,
        proposals: &[Proposal],
    ) -> Result<&TransitionRecord, EventLogError> {
        if self.format != "meatware-event-log" || self.schema_version != EVENT_LOG_SCHEMA_VERSION {
            return Err(EventLogError::InvalidFormat("unsupported event log envelope".into()));
        }
        let expected = if let Some(last) = self.records.last() {
            last.target_hash
        } else {
            Configuration::from_snapshot(&self.initial_snapshot)?.state_hash()?
        };
        let actual = state.state_hash()?;
        if actual != expected {
            return Err(EventLogError::HashMismatch {
                boundary: state.boundary, expected, actual,
            });
        }
        let source_boundary = state.boundary;
        let source_hash = actual;
        let mut candidate = state.clone();
        candidate.apply_batch(proposals)?;
        let target_hash = candidate.state_hash()?;
        let record = TransitionRecord {
            schema_version: EVENT_LOG_SCHEMA_VERSION,
            source_boundary,
            target_boundary: candidate.boundary,
            source_hash,
            target_hash,
            proposals: proposals.to_vec(),
        };
        *state = candidate;
        self.records.push(record);
        Ok(self.records.last().expect("record was just appended"))
    }

    pub fn replay(&self) -> Result<Configuration, EventLogError> {
        if self.format != "meatware-event-log" || self.schema_version != EVENT_LOG_SCHEMA_VERSION {
            return Err(EventLogError::InvalidFormat("unsupported event log envelope".into()));
        }
        let mut state = Configuration::from_snapshot(&self.initial_snapshot)?;
        for record in &self.records {
            if record.schema_version != EVENT_LOG_SCHEMA_VERSION {
                return Err(EventLogError::InvalidRecord("unsupported transition record version".into()));
            }
            if state.boundary != record.source_boundary {
                return Err(EventLogError::BoundaryMismatch {
                    expected: record.source_boundary, actual: state.boundary,
                });
            }
            let source_hash = state.state_hash()?;
            if source_hash != record.source_hash {
                return Err(EventLogError::HashMismatch {
                    boundary: state.boundary, expected: record.source_hash, actual: source_hash,
                });
            }
            state.apply_batch(&record.proposals)?;
            if state.boundary != record.target_boundary {
                return Err(EventLogError::BoundaryMismatch {
                    expected: record.target_boundary, actual: state.boundary,
                });
            }
            let target_hash = state.state_hash()?;
            if target_hash != record.target_hash {
                return Err(EventLogError::HashMismatch {
                    boundary: state.boundary, expected: record.target_hash, actual: target_hash,
                });
            }
        }
        Ok(state)
    }

    pub fn to_json(&self) -> Result<String, EventLogError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| EventLogError::Serialization(e.to_string()))
    }

    pub fn from_json(input: &str) -> Result<Self, EventLogError> {
        let log: Self = serde_json::from_str(input)
            .map_err(|e| EventLogError::Serialization(e.to_string()))?;
        if log.format != "meatware-event-log" || log.schema_version != EVENT_LOG_SCHEMA_VERSION {
            return Err(EventLogError::InvalidFormat("unsupported event log envelope".into()));
        }
        Ok(log)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edit, Entity, Properties, Value};

    fn initial() -> Configuration {
        let mut state = Configuration::default();
        state.entities.insert(1, Entity {
            id: 1,
            properties: Properties::new(),
        });
        state.next_id = 2;
        state
    }

    fn proposal(boundary: u64, key: u64, value: i64) -> Proposal {
        Proposal {
            key,
            source_boundary: boundary,
            reads: Vec::new(),
            edits: vec![Edit::SetEntityProperty {
                id: 1,
                key: "signal".into(),
                value: Value::Integer(value),
            }],
        }
    }

    #[test]
    fn log_replays_to_identical_state() {
        let mut state = initial();
        let mut log = EventLog::new(&state).unwrap();
        log.append(&mut state, &[proposal(0, 1, 7)]).unwrap();
        log.append(&mut state, &[proposal(1, 2, 9)]).unwrap();
        let replayed = log.replay().unwrap();
        assert_eq!(state.state_hash().unwrap(), replayed.state_hash().unwrap());
        assert_eq!(state.boundary, replayed.boundary);
    }

    #[test]
    fn json_round_trip_preserves_replay() {
        let mut state = initial();
        let mut log = EventLog::new(&state).unwrap();
        log.append(&mut state, &[proposal(0, 1, 7)]).unwrap();
        let encoded = log.to_json().unwrap();
        let decoded = EventLog::from_json(&encoded).unwrap();
        assert_eq!(state.state_hash().unwrap(), decoded.replay().unwrap().state_hash().unwrap());
    }

    #[test]
    fn failed_transition_changes_neither_state_nor_log() {
        let mut state = initial();
        let mut log = EventLog::new(&state).unwrap();
        let before = state.state_hash().unwrap();
        let bad = Proposal {
            key: 1,
            source_boundary: 0,
            reads: Vec::new(),
            edits: vec![Edit::SetEntityProperty {
                id: 99, key: "missing".into(), value: Value::Bool(true),
            }],
        };
        assert!(log.append(&mut state, &[bad]).is_err());
        assert_eq!(state.state_hash().unwrap(), before);
        assert!(log.records.is_empty());
    }

    #[test]
    fn external_mutation_is_detected_before_append() {
        let mut state = initial();
        let mut log = EventLog::new(&state).unwrap();
        state.entities.get_mut(&1).unwrap().properties.insert("outside".into(), Value::Bool(true));
        assert!(matches!(
            log.append(&mut state, &[proposal(0, 1, 3)]),
            Err(EventLogError::HashMismatch { .. })
        ));
        assert!(log.records.is_empty());
    }

    #[test]
    fn tampered_record_is_detected_during_replay() {
        let mut state = initial();
        let mut log = EventLog::new(&state).unwrap();
        log.append(&mut state, &[proposal(0, 1, 7)]).unwrap();
        log.records[0].target_hash ^= 1;
        assert!(matches!(log.replay(), Err(EventLogError::HashMismatch { .. })));
    }
}
