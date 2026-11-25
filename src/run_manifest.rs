use crate::{ConflictPolicy, Properties};
use serde::{Deserialize, Serialize};

pub const RUN_MANIFEST_SCHEMA_VERSION: u32 = 1;
pub const RUN_MANIFEST_FORMAT: &str = "meatware-run-manifest";

/// Reproduction inputs and terminal verification data for one execution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunManifest {
    pub format: String,
    pub schema_version: u32,
    pub runtime: String,
    pub build_commit: String,
    pub model: String,
    pub random_generator: String,
    pub random_stream_scheme: String,
    pub seed: u64,
    pub initial_entities: usize,
    pub parameters: Properties,
    pub conflict_policy: ConflictPolicy,
    pub initial_state_hash: String,
    pub transitions: u64,
    pub event_records: usize,
    pub terminal_state_hash: String,
    pub replay_state_hash: String,
    pub determinism: String,
}

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    Serialization(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ManifestError {}

impl RunManifest {
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.format != RUN_MANIFEST_FORMAT {
            return Err(ManifestError::InvalidFormat("unsupported run manifest format".into()));
        }
        if self.schema_version != RUN_MANIFEST_SCHEMA_VERSION {
            return Err(ManifestError::InvalidFormat(format!(
                "unsupported run manifest schema version: {}", self.schema_version
            )));
        }
        for (name, value) in [
            ("runtime", self.runtime.as_str()),
            ("model", self.model.as_str()),
            ("random_generator", self.random_generator.as_str()),
            ("random_stream_scheme", self.random_stream_scheme.as_str()),
            ("initial_state_hash", self.initial_state_hash.as_str()),
            ("terminal_state_hash", self.terminal_state_hash.as_str()),
            ("replay_state_hash", self.replay_state_hash.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ManifestError::InvalidFormat(format!("missing {name}")));
            }
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, ManifestError> {
        self.validate()?;
        serde_json::to_string_pretty(self)
            .map_err(|e| ManifestError::Serialization(e.to_string()))
    }

    pub fn from_json(input: &str) -> Result<Self, ManifestError> {
        let manifest: Self = serde_json::from_str(input)
            .map_err(|e| ManifestError::Serialization(e.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    fn sample() -> RunManifest {
        RunManifest {
            format: RUN_MANIFEST_FORMAT.into(),
            schema_version: RUN_MANIFEST_SCHEMA_VERSION,
            runtime: "meatware-wasm-0.1.0".into(),
            build_commit: "test-commit".into(),
            model: "two-channel-local-exchange-v1".into(),
            random_generator: "splitmix64-v1".into(),
            random_stream_scheme: "single-sequential-stream-v1".into(),
            seed: 17,
            initial_entities: 32,
            parameters: [("coupling".into(), Value::Float(0.8))].into_iter().collect(),
            conflict_policy: ConflictPolicy::Reject,
            initial_state_hash: "0123456789abcdef".into(),
            transitions: 4,
            event_records: 4,
            terminal_state_hash: "1111111111111111".into(),
            replay_state_hash: "1111111111111111".into(),
            determinism: "proposal replay verified terminal state".into(),
        }
    }

    #[test]
    fn manifest_json_round_trip() {
        let manifest = sample();
        let decoded = RunManifest::from_json(&manifest.to_json().unwrap()).unwrap();
        assert_eq!(manifest, decoded);
    }

    #[test]
    fn unknown_manifest_version_is_rejected() {
        let mut manifest = sample();
        manifest.schema_version += 1;
        assert!(matches!(manifest.to_json(), Err(ManifestError::InvalidFormat(_))));
    }
}
