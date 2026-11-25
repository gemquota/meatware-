//! Language-neutral substrate semantics, implemented as a deterministic Rust reference runtime.
pub mod event_log;

#[cfg(target_arch = "wasm32")]
mod wasm;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub type Id = u64;
pub type Properties = BTreeMap<String, Value>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null, Bool(bool), Integer(i64), Float(f64), String(String),
    Sequence(Vec<Value>), Map(BTreeMap<String, Value>),
}
impl Value {
    fn validate(&self) -> Result<(), RuntimeError> {
        match self {
            Self::Float(v) if !v.is_finite() => Err(RuntimeError::NonFiniteFloat),
            Self::Sequence(xs) => { for x in xs { x.validate()?; } Ok(()) },
            Self::Map(xs) => { for x in xs.values() { x.validate()?; } Ok(()) },
            _ => Ok(()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Endpoint { Entity(Id), Relation(Id) }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity { pub id: Id, pub properties: Properties }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Relation { pub id: Id, pub kind: String, pub endpoints: Vec<Endpoint>, pub properties: Properties }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub boundary: u64, pub next_id: Id,
    pub entities: BTreeMap<Id, Entity>, pub relations: BTreeMap<Id, Relation>,
    /// Retired identifiers are retained to prevent identity reuse.
    pub retired: BTreeSet<Id>,
}
impl Default for Configuration {
    fn default() -> Self { Self { boundary: 0, next_id: 1, entities: BTreeMap::new(), relations: BTreeMap::new(), retired: BTreeSet::new() } }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    DuplicateId(Id), UnknownEntity(Id), UnknownRelation(Id), InvalidEndpoint(Endpoint),
    NonFiniteFloat, EmptyRelationEndpoints, DuplicateProposalKey(u64), ConflictingWrite(String), ReadDependencyFailed(String),
    RetireWriteConflict(Id), BoundaryMismatch { expected: u64, actual: u64 },
    InvalidPropertyKey, InvalidSnapshot(String),
}
impl fmt::Display for RuntimeError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") } }
impl std::error::Error for RuntimeError {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Edit {
    CreateEntity { id: Id, properties: Properties },
    CreateRelation { id: Id, kind: String, endpoints: Vec<Endpoint>, properties: Properties },
    SetEntityProperty { id: Id, key: String, value: Value },
    RemoveEntityProperty { id: Id, key: String },
    SetRelationProperty { id: Id, key: String, value: Value },
    RemoveRelationProperty { id: Id, key: String },
    RetireEntity { id: Id }, RetireRelation { id: Id },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReadDependency {
 EntityExists { id: Id, expected: bool }, RelationExists { id: Id, expected: bool },
 EntityProperty { id: Id, key: String, expected: Option<Value> }, RelationProperty { id: Id, key: String, expected: Option<Value> },
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictPolicy { #[default] Reject, LowestProposalKeyWins, HighestProposalKeyWins }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposal { pub key: u64, pub source_boundary: u64, #[serde(default)] pub reads: Vec<ReadDependency>, pub edits: Vec<Edit> }

impl Configuration {
    pub fn allocate_id(&mut self) -> Result<Id, RuntimeError> {
        while self.entities.contains_key(&self.next_id) || self.relations.contains_key(&self.next_id) || self.retired.contains(&self.next_id) {
            self.next_id = self.next_id.checked_add(1).ok_or(RuntimeError::DuplicateId(self.next_id))?;
        }
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).ok_or(RuntimeError::DuplicateId(id))?;
        Ok(id)
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, RuntimeError> {
        serde_json::to_vec(self).map_err(|e| RuntimeError::InvalidSnapshot(e.to_string()))
    }
    /// FNV-1a 64-bit diagnostic hash, not a cryptographic integrity primitive.
    pub fn state_hash(&self) -> Result<u64, RuntimeError> {
        let mut h = 0xcbf29ce484222325u64;
        for byte in self.canonical_bytes()? { h ^= u64::from(byte); h = h.wrapping_mul(0x100000001b3); }
        Ok(h)
    }
    pub fn snapshot(&self) -> Result<Vec<u8>, RuntimeError> {
        self.validate()?;
        let envelope = crate::event_log::SnapshotEnvelope {
            format: "meatware-snapshot".into(),
            schema_version: crate::event_log::SNAPSHOT_SCHEMA_VERSION,
            state: self.clone(),
        };
        serde_json::to_vec(&envelope).map_err(|e| RuntimeError::InvalidSnapshot(e.to_string()))
    }
    pub fn from_snapshot(bytes: &[u8]) -> Result<Self, RuntimeError> {
        let value: serde_json::Value = serde_json::from_slice(bytes)
            .map_err(|e| RuntimeError::InvalidSnapshot(e.to_string()))?;
        let state = if value.get("format").and_then(serde_json::Value::as_str) == Some("meatware-snapshot") {
            let envelope: crate::event_log::SnapshotEnvelope = serde_json::from_value(value)
                .map_err(|e| RuntimeError::InvalidSnapshot(e.to_string()))?;
            if envelope.schema_version != crate::event_log::SNAPSHOT_SCHEMA_VERSION {
                return Err(RuntimeError::InvalidSnapshot(format!("unsupported snapshot schema version: {}", envelope.schema_version)));
            }
            envelope.state
        } else {
            // Read legacy unwrapped snapshots written before schema versioning.
            serde_json::from_value(value).map_err(|e| RuntimeError::InvalidSnapshot(e.to_string()))?
        };
        state.validate()?;
        Ok(state)
    }
    pub fn validate(&self) -> Result<(), RuntimeError> {
        let mut identities = BTreeSet::new();
        for (id, entity) in &self.entities {
            if *id != entity.id || !identities.insert(*id) || self.retired.contains(id) { return Err(RuntimeError::DuplicateId(*id)); }
            validate_properties(&entity.properties)?;
        }
        for (id, relation) in &self.relations {
            if *id != relation.id || !identities.insert(*id) || self.retired.contains(id) { return Err(RuntimeError::DuplicateId(*id)); }
            if relation.endpoints.is_empty() { return Err(RuntimeError::EmptyRelationEndpoints); }
            validate_properties(&relation.properties)?;
            for endpoint in &relation.endpoints { self.validate_endpoint(endpoint)?; }
        }
        for id in &self.retired {
            if !identities.insert(*id) { return Err(RuntimeError::DuplicateId(*id)); }
        }
        Ok(())
    }
    fn check_read(&self, dependency: &ReadDependency) -> Result<(), RuntimeError> {
        let valid = match dependency {
            ReadDependency::EntityExists { id, expected } => self.entities.contains_key(id) == *expected,
            ReadDependency::RelationExists { id, expected } => self.relations.contains_key(id) == *expected,
            ReadDependency::EntityProperty { id, key, expected } => self.entities.get(id).map(|e| e.properties.get(key)) == Some(expected.as_ref()),
            ReadDependency::RelationProperty { id, key, expected } => self.relations.get(id).map(|r| r.properties.get(key)) == Some(expected.as_ref()),
        };
        if valid { Ok(()) } else { Err(RuntimeError::ReadDependencyFailed(format!("{dependency:?}"))) }
    }
    fn validate_endpoint(&self, endpoint: &Endpoint) -> Result<(), RuntimeError> {
        let exists = match endpoint { Endpoint::Entity(id) => self.entities.contains_key(id), Endpoint::Relation(id) => self.relations.contains_key(id) };
        if exists { Ok(()) } else { Err(RuntimeError::InvalidEndpoint(endpoint.clone())) }
    }
    pub fn apply_batch(&mut self, proposals: &[Proposal]) -> Result<(), RuntimeError> {
        self.apply_batch_with_policy(proposals, ConflictPolicy::Reject)
    }
    pub fn apply_batch_with_policy(&mut self, proposals: &[Proposal], policy: ConflictPolicy) -> Result<(), RuntimeError> {
        let mut ordered = proposals.to_vec(); ordered.sort_by_key(|p| p.key);
        let mut seen_keys = BTreeSet::new();
        let mut retires = BTreeSet::new(); let mut edited = BTreeSet::new();
        for p in &ordered {
            if p.source_boundary != self.boundary { return Err(RuntimeError::BoundaryMismatch { expected: self.boundary, actual: p.source_boundary }); }
            if !seen_keys.insert(p.key) { return Err(RuntimeError::DuplicateProposalKey(p.key)); }
            for dependency in &p.reads { self.check_read(dependency)?; }
            for e in &p.edits {
                match e {
                    Edit::SetEntityProperty{id,key,..}|Edit::RemoveEntityProperty{id,key} => {
                        let _ = key; edited.insert(*id);
                    }
                    Edit::SetRelationProperty{id,key,..}|Edit::RemoveRelationProperty{id,key} => {
                        let _ = key; edited.insert(*id);
                    }
                    Edit::RetireEntity{id}|Edit::RetireRelation{id} => { retires.insert(*id); }
                    _ => {}
                }
            }
        }
        for id in &retires { if edited.contains(id) { return Err(RuntimeError::RetireWriteConflict(*id)); } }
        let mut owners: BTreeMap<String, Vec<u64>> = BTreeMap::new();
        for p in &ordered {
            let mut local = BTreeSet::new();
            for e in &p.edits {
                let key = match e {
                    Edit::SetEntityProperty{id,key,..}|Edit::RemoveEntityProperty{id,key} => Some(format!("e:{id}:{key}")),
                    Edit::SetRelationProperty{id,key,..}|Edit::RemoveRelationProperty{id,key} => Some(format!("r:{id}:{key}")),
                    _ => None,
                };
                if let Some(key) = key { if !local.insert(key.clone()) { return Err(RuntimeError::ConflictingWrite(key)); } owners.entry(key).or_default().push(p.key); }
            }
        }
        let mut winners = BTreeMap::new();
        for (key, keys) in owners {
            let winner = if keys.len() == 1 { keys[0] } else { match policy {
                ConflictPolicy::Reject => return Err(RuntimeError::ConflictingWrite(key)),
                ConflictPolicy::LowestProposalKeyWins => *keys.iter().min().unwrap(),
                ConflictPolicy::HighestProposalKeyWins => *keys.iter().max().unwrap(),
            }};
            winners.insert(key, winner);
        }
        let mut candidate=self.clone(); let mut created=BTreeSet::new();
        for p in &ordered { for e in &p.edits {
            let write_key = match e {
                Edit::SetEntityProperty{id,key,..}|Edit::RemoveEntityProperty{id,key} => Some(format!("e:{id}:{key}")),
                Edit::SetRelationProperty{id,key,..}|Edit::RemoveRelationProperty{id,key} => Some(format!("r:{id}:{key}")),
                _ => None,
            };
            if let Some(key) = write_key { if winners.get(&key) != Some(&p.key) { continue; } }
            match e {
                Edit::CreateEntity{id,properties} => {
                    ensure_fresh(&candidate,*id,&mut created)?; validate_properties(properties)?;
                    candidate.entities.insert(*id,Entity{id:*id,properties:properties.clone()});
                    candidate.next_id=candidate.next_id.max(id.checked_add(1).ok_or(RuntimeError::DuplicateId(*id))?);
                }
                Edit::CreateRelation{id,kind,endpoints,properties} => {
                    ensure_fresh(&candidate,*id,&mut created)?;
                    if endpoints.is_empty() { return Err(RuntimeError::EmptyRelationEndpoints); }
                    validate_properties(properties)?;
                    candidate.relations.insert(*id,Relation{id:*id,kind:kind.clone(),endpoints:endpoints.clone(),properties:properties.clone()});
                    candidate.next_id=candidate.next_id.max(id.checked_add(1).ok_or(RuntimeError::DuplicateId(*id))?);
                }
                Edit::SetEntityProperty{id,key,value} => {
                    value.validate()?; candidate.entities.get_mut(id).ok_or(RuntimeError::UnknownEntity(*id))?.properties.insert(valid_key(key)?,value.clone());
                }
                Edit::RemoveEntityProperty{id,key} => {
                    candidate.entities.get_mut(id).ok_or(RuntimeError::UnknownEntity(*id))?.properties.remove(&valid_key(key)?);
                }
                Edit::SetRelationProperty{id,key,value} => {
                    value.validate()?; candidate.relations.get_mut(id).ok_or(RuntimeError::UnknownRelation(*id))?.properties.insert(valid_key(key)?,value.clone());
                }
                Edit::RemoveRelationProperty{id,key} => {
                    candidate.relations.get_mut(id).ok_or(RuntimeError::UnknownRelation(*id))?.properties.remove(&valid_key(key)?);
                }
                Edit::RetireEntity{id} => { candidate.entities.remove(id).ok_or(RuntimeError::UnknownEntity(*id))?; candidate.retired.insert(*id); }
                Edit::RetireRelation{id} => { candidate.relations.remove(id).ok_or(RuntimeError::UnknownRelation(*id))?; candidate.retired.insert(*id); }
            }
        }}
        candidate.boundary=candidate.boundary.checked_add(1).ok_or(RuntimeError::InvalidSnapshot("boundary overflow".into()))?;
        candidate.validate()?; *self=candidate; Ok(())
    }
}
fn ensure_fresh(s:&Configuration,id:Id,created:&mut BTreeSet<Id>)->Result<(),RuntimeError>{
    if s.entities.contains_key(&id)||s.relations.contains_key(&id)||s.retired.contains(&id)||!created.insert(id){Err(RuntimeError::DuplicateId(id))}else{Ok(())}
}
fn valid_key(k:&str)->Result<String,RuntimeError>{if k.trim().is_empty(){Err(RuntimeError::InvalidPropertyKey)}else{Ok(k.to_owned())}}
fn validate_properties(p:&Properties)->Result<(),RuntimeError>{for(k,v)in p{valid_key(k)?;v.validate()?;}Ok(())}

/// Explicit SplitMix64 stream. Generator algorithm/version and stream key belong in the run manifest.
#[derive(Clone,Debug)]
pub struct RandomStream{state:u64}
impl RandomStream{
    pub fn new(seed:u64,stream_key:u64)->Self{
        let mut x=seed^stream_key.wrapping_mul(0x9e3779b97f4a7c15);
        x=(x^(x>>30)).wrapping_mul(0xbf58476d1ce4e5b9); x=(x^(x>>27)).wrapping_mul(0x94d049bb133111eb);
        Self{state:x^(x>>31)}
    }
    pub fn next_u64(&mut self)->u64{
        self.state=self.state.wrapping_add(0x9e3779b97f4a7c15); let mut z=self.state;
        z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9); z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb); z^(z>>31)
    }
    pub fn unit_f64(&mut self)->f64{(self.next_u64()>>11)as f64*(1.0/((1u64<<53)as f64))}
}

#[cfg(test)]
mod tests{
 use super::*;
 fn p(key:u64,edits:Vec<Edit>)->Proposal{Proposal{key,source_boundary:0,reads:Vec::new(),edits}}
 #[test]fn independent_edits_commit(){let mut s=Configuration::default();for id in 1..=2{s.entities.insert(id,Entity{id,properties:Properties::new()});}s.next_id=3;
 s.apply_batch(&[p(2,vec![Edit::SetEntityProperty{id:2,key:"x".into(),value:Value::Integer(2)}]),p(1,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(1)}])]).unwrap();
 assert_eq!(s.boundary,1);assert_eq!(s.entities[&1].properties["x"],Value::Integer(1));}
 #[test]fn conflicts_reject_without_mutation(){let mut s=Configuration::default();s.entities.insert(1,Entity{id:1,properties:Properties::new()});let before=s.state_hash().unwrap();
 let r=s.apply_batch(&[p(1,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(1)}]),p(2,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(2)}])]);
 assert!(matches!(r,Err(RuntimeError::ConflictingWrite(_))));assert_eq!(s.state_hash().unwrap(),before);}
 #[test]fn relation_can_reference_relation(){let mut s=Configuration::default();s.entities.insert(1,Entity{id:1,properties:Properties::new()});
 s.relations.insert(2,Relation{id:2,kind:"link".into(),endpoints:vec![Endpoint::Entity(1)],properties:Properties::new()});
 s.relations.insert(3,Relation{id:3,kind:"bundle".into(),endpoints:vec![Endpoint::Relation(2)],properties:Properties::new()});assert!(s.validate().is_ok());}
 #[test]fn dangling_endpoint_rejects_atomically(){let mut s=Configuration::default();s.entities.insert(1,Entity{id:1,properties:Properties::new()});
 s.relations.insert(2,Relation{id:2,kind:"link".into(),endpoints:vec![Endpoint::Entity(1)],properties:Properties::new()});let before=s.state_hash().unwrap();
 assert!(matches!(s.apply_batch(&[p(1,vec![Edit::RetireEntity{id:1}])]),Err(RuntimeError::InvalidEndpoint(_))));assert_eq!(s.state_hash().unwrap(),before);}
 #[test]fn snapshot_round_trip(){let mut s=Configuration::default();s.entities.insert(7,Entity{id:7,properties:[("v".into(),Value::Float(1.25))].into_iter().collect()});let b=s.snapshot().unwrap();let t=Configuration::from_snapshot(&b).unwrap();assert_eq!(s.state_hash().unwrap(),t.state_hash().unwrap());}
 #[test]fn random_stream_is_reproducible(){let mut a=RandomStream::new(42,9);let mut b=RandomStream::new(42,9);assert_eq!((0..20).map(|_|a.next_u64()).collect::<Vec<_>>(),(0..20).map(|_|b.next_u64()).collect::<Vec<_>>());}
 #[test]fn read_dependency_rejects_stale_observation_atomically(){
 let mut s=Configuration::default();s.entities.insert(1,Entity{id:1,properties:[("x".into(),Value::Integer(1))].into_iter().collect()});let before=s.state_hash().unwrap();
 let mut q=p(1,vec![Edit::SetEntityProperty{id:1,key:"y".into(),value:Value::Bool(true)}]);q.reads.push(ReadDependency::EntityProperty{id:1,key:"x".into(),expected:Some(Value::Integer(2))});
 assert!(matches!(s.apply_batch(&[q]),Err(RuntimeError::ReadDependencyFailed(_))));assert_eq!(s.state_hash().unwrap(),before);
 }
 #[test]fn absent_entity_does_not_satisfy_absent_property_read(){
 let mut s=Configuration::default();let mut q=p(1,vec![]);q.reads.push(ReadDependency::EntityProperty{id:404,key:"x".into(),expected:None});
 assert!(matches!(s.apply_batch(&[q]),Err(RuntimeError::ReadDependencyFailed(_))));assert_eq!(s.boundary,0);
 }
 #[test]fn explicit_conflict_policy_uses_stable_proposal_key(){
 let mut s=Configuration::default();s.entities.insert(1,Entity{id:1,properties:Properties::new()});let a=p(9,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(9)}]);let b=p(2,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(2)}]);
 s.apply_batch_with_policy(&[a,b],ConflictPolicy::LowestProposalKeyWins).unwrap();assert_eq!(s.entities[&1].properties["x"],Value::Integer(2));
 }
 #[test]fn default_conflict_policy_still_rejects(){
 let mut s=Configuration::default();s.entities.insert(1,Entity{id:1,properties:Properties::new()});let a=p(9,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(9)}]);let b=p(2,vec![Edit::SetEntityProperty{id:1,key:"x".into(),value:Value::Integer(2)}]);
 assert!(matches!(s.apply_batch(&[a,b]),Err(RuntimeError::ConflictingWrite(_))));
 }
 #[test]fn failed_batch_does_not_advance(){let mut s=Configuration::default();assert!(s.apply_batch(&[p(1,vec![Edit::SetEntityProperty{id:99,key:"x".into(),value:Value::Bool(true)}])]).is_err());assert_eq!(s.boundary,0);}
 #[test]fn versioned_snapshot_has_envelope(){let s=Configuration::default();let bytes=s.snapshot().unwrap();let v:serde_json::Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(v["format"],"meatware-snapshot");assert_eq!(v["schema_version"],1);}
 #[test]fn legacy_unwrapped_snapshot_still_loads(){let s=Configuration::default();let bytes=serde_json::to_vec(&s).unwrap();let loaded=Configuration::from_snapshot(&bytes).unwrap();assert_eq!(s.state_hash().unwrap(),loaded.state_hash().unwrap());}
 #[test]fn unknown_snapshot_version_is_rejected(){let s=Configuration::default();let mut v:serde_json::Value=serde_json::from_slice(&s.snapshot().unwrap()).unwrap();v["schema_version"]=serde_json::json!(999);let bytes=serde_json::to_vec(&v).unwrap();assert!(matches!(Configuration::from_snapshot(&bytes),Err(RuntimeError::InvalidSnapshot(_))));}
}
