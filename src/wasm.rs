use crate::event_log::EventLog;
use crate::interactions::{evaluate_interactions, InteractionRule};
use crate::run_manifest::RunManifest;
use crate::{Configuration, ConflictPolicy, Edit, Endpoint, Entity, Properties, Proposal, RandomStream, ReadDependency, Value};
use wasm_bindgen::prelude::*;

fn relation_eligible(source: &Configuration, participants: &[Endpoint], parameters: &Properties) -> bool {
    if participants.len() != 2 { return false; }
    let (a, b) = match (&participants[0], &participants[1]) {
        (Endpoint::Entity(a), Endpoint::Entity(b)) => (*a, *b),
        _ => return false,
    };
    let relation_id = 1_000_000 + a * 501 + b;
    if source.relations.contains_key(&relation_id) { return false; }
    let (Some(pa), Some(pb)) = (source.entities.get(&a), source.entities.get(&b)) else { return false; };
    let mut dx = number(&pb.properties, "x") - number(&pa.properties, "x");
    let mut dy = number(&pb.properties, "y") - number(&pa.properties, "y");
    if dx > 0.5 { dx -= 1.0; } else if dx < -0.5 { dx += 1.0; }
    if dy > 0.5 { dy -= 1.0; } else if dy < -0.5 { dy += 1.0; }
    let radius = match parameters.get("radius") { Some(Value::Float(v)) => *v, _ => 0.055 };
    dx * dx + dy * dy < radius.powi(2)
}

fn create_relation_proposal(
    source: &Configuration,
    participants: &[Endpoint],
    random: &mut RandomStream,
) -> Option<Proposal> {
    let (a, b) = match (&participants[0], &participants[1]) {
        (Endpoint::Entity(a), Endpoint::Entity(b)) => (*a, *b),
        _ => return None,
    };
    let pa = &source.entities.get(&a)?.properties;
    let pb = &source.entities.get(&b)?.properties;
    let relation_id = 1_000_000 + a * 501 + b;
    let mut dx = number(pb, "x") - number(pa, "x");
    let mut dy = number(pb, "y") - number(pa, "y");
    if dx > 0.5 { dx -= 1.0; } else if dx < -0.5 { dx += 1.0; }
    if dy > 0.5 { dy -= 1.0; } else if dy < -0.5 { dy += 1.0; }
    let distance_sq = dx * dx + dy * dy;
    let mut properties = Properties::new();
    properties.insert("distance_sq".into(), Value::Float(distance_sq));
    properties.insert("weight".into(), Value::Float(random.unit_f64()));
    Some(Proposal {
        key: 0,
        source_boundary: source.boundary,
        reads: vec![
            ReadDependency::EntityProperty { id: a, key: "x".into(), expected: pa.get("x").cloned() },
            ReadDependency::EntityProperty { id: a, key: "y".into(), expected: pa.get("y").cloned() },
            ReadDependency::EntityProperty { id: b, key: "x".into(), expected: pb.get("x").cloned() },
            ReadDependency::EntityProperty { id: b, key: "y".into(), expected: pb.get("y").cloned() },
        ],
        edits: vec![Edit::CreateRelation {
            id: relation_id,
            kind: "association".into(),
            endpoints: participants.to_vec(),
            properties,
        }],
    })
}

fn number(properties: &Properties, key: &str) -> f64 {
    match properties.get(key) {
        Some(Value::Float(v)) => *v,
        Some(Value::Integer(v)) => *v as f64,
        _ => 0.0,
    }
}

#[wasm_bindgen]
pub struct World {
    state: Configuration,
    log: EventLog,
    random: RandomStream,
    coupling: f64,
    noise: f64,
    radius: f64,
    seed: u32,
    initial_entities: usize,
    initial_state_hash: String,
    conflict_policy: ConflictPolicy,
    used_event_local_streams: bool,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize, seed: u32) -> Result<World, JsValue> {
        let mut random = RandomStream::new(seed as u64, 0);
        let mut state = Configuration::default();
        for id in 1..=count.clamp(8, 500) as u64 {
            let mut properties = Properties::new();
            properties.insert("x".into(), Value::Float(random.unit_f64()));
            properties.insert("y".into(), Value::Float(random.unit_f64()));
            properties.insert("a".into(), Value::Float(random.unit_f64()));
            properties.insert("b".into(), Value::Float(random.unit_f64()));
            state.entities.insert(id, Entity { id, properties });
        }
        state.next_id = state.entities.len() as u64 + 1;
        let initial_hash = state.state_hash().map_err(|e| JsValue::from_str(&e.to_string()))?;
        let initial_entities = state.entities.len();
        let log = EventLog::new(&state).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(World {
            state, log, random,
            coupling: 0.8, noise: 0.008, radius: 0.055,
            seed, initial_entities, initial_state_hash: format!("{initial_hash:016x}"),
            conflict_policy: ConflictPolicy::Reject,
            used_event_local_streams: false,
        })
    }

    /// Form generic relationships between entities within the fixed experimental radius.
    /// All candidate pairs are evaluated from one immutable source boundary.
    pub fn form_relations(&mut self, radius: f64) -> Result<u32, JsValue> {
        let rule = InteractionRule::entities(
            0x4d45_4154,
            1,
            2,
            125_000,
            create_relation_proposal,
        ).with_eligibility(relation_eligible).with_parameters({
            let mut parameters = Properties::new();
            parameters.insert("radius".into(), Value::Float(radius.clamp(0.005, 0.25)));
            parameters
        });
        let proposals = evaluate_interactions(&self.state, self.seed as u64, &[rule])
            .map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
        let count = proposals.len();
        if count > 0 {
            self.log.append_with_policy(&mut self.state, &proposals, self.conflict_policy)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.used_event_local_streams = true;
        }
        Ok(count as u32)
    }

    pub fn relation_count(&self) -> usize { self.state.relations.len() }

    /// Flattened endpoint coordinates for drawing relationship edges.
    pub fn relation_edges(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.state.relations.len() * 4);
        for relation in self.state.relations.values() {
            if relation.endpoints.len() != 2 { continue; }
            let ids = match (&relation.endpoints[0], &relation.endpoints[1]) {
                (Endpoint::Entity(a), Endpoint::Entity(b)) => (*a, *b),
                _ => continue,
            };
            let (Some(a), Some(b)) = (self.state.entities.get(&ids.0), self.state.entities.get(&ids.1)) else { continue; };
            out.push(number(&a.properties, "x"));
            out.push(number(&a.properties, "y"));
            out.push(number(&b.properties, "x"));
            out.push(number(&b.properties, "y"));
        }
        out
    }

    pub fn step(&mut self, steps: u32) -> Result<(), JsValue> {
        for _ in 0..steps.min(1000) {
            let ids: Vec<u64> = self.state.entities.keys().copied().collect();
            let n = ids.len();
            let mut da = vec![0.0; n];
            let mut db = vec![0.0; n];
            let positions: Vec<(f64, f64, f64, f64)> = ids.iter().map(|id| {
                let p = &self.state.entities[id].properties;
                (number(p, "x"), number(p, "y"), number(p, "a"), number(p, "b"))
            }).collect();
            let r2 = self.radius * self.radius;

            for i in 0..n {
                for j in (i + 1)..n {
                    let mut dx = positions[j].0 - positions[i].0;
                    let mut dy = positions[j].1 - positions[i].1;
                    if dx > 0.5 { dx -= 1.0; } else if dx < -0.5 { dx += 1.0; }
                    if dy > 0.5 { dy -= 1.0; } else if dy < -0.5 { dy += 1.0; }
                    let d2 = dx * dx + dy * dy;
                    if d2 < r2 {
                        let weight = (1.0 - d2 / r2) * self.coupling * 0.025;
                        da[i] += (positions[j].2 - positions[i].2) * weight;
                        da[j] += (positions[i].2 - positions[j].2) * weight;
                        db[i] += (positions[j].3 - positions[i].3) * weight;
                        db[j] += (positions[i].3 - positions[j].3) * weight;
                    }
                }
            }

            let boundary = self.state.boundary;
            let mut proposals = Vec::with_capacity(n);
            for i in 0..n {
                let a = (positions[i].2 + da[i] + (self.random.unit_f64() - 0.5) * self.noise).clamp(0.0, 1.0);
                let b = (positions[i].3 + db[i] + (self.random.unit_f64() - 0.5) * self.noise).clamp(0.0, 1.0);
                let x = (positions[i].0 + (a - 0.5) * 0.00045 + (self.random.unit_f64() - 0.5) * 0.00015).rem_euclid(1.0);
                let y = (positions[i].1 + (b - 0.5) * 0.00045 + (self.random.unit_f64() - 0.5) * 0.00015).rem_euclid(1.0);
                let id = ids[i];
                proposals.push(Proposal {
                    key: id,
                    source_boundary: boundary,
                    reads: Vec::new(),
                    edits: vec![
                        Edit::SetEntityProperty { id, key: "a".into(), value: Value::Float(a) },
                        Edit::SetEntityProperty { id, key: "b".into(), value: Value::Float(b) },
                        Edit::SetEntityProperty { id, key: "x".into(), value: Value::Float(x) },
                        Edit::SetEntityProperty { id, key: "y".into(), value: Value::Float(y) },
                    ],
                });
            }
            self.log.append_with_policy(&mut self.state, &proposals, self.conflict_policy)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }

    pub fn set_coupling(&mut self, value: f64) { self.coupling = value.clamp(0.0, 2.0); }
    pub fn set_noise(&mut self, value: f64) { self.noise = value.clamp(0.0, 0.1); }
    pub fn set_radius(&mut self, value: f64) { self.radius = value.clamp(0.005, 0.25); }
    pub fn count(&self) -> usize { self.state.entities.len() }
    pub fn tick(&self) -> u64 { self.state.boundary }
    pub fn record_count(&self) -> usize { self.log.records.len() }

    pub fn positions(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.state.entities.len() * 2);
        for entity in self.state.entities.values() {
            out.push(number(&entity.properties, "x"));
            out.push(number(&entity.properties, "y"));
        }
        out
    }

    pub fn channels(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.state.entities.len() * 2);
        for entity in self.state.entities.values() {
            out.push(number(&entity.properties, "a"));
            out.push(number(&entity.properties, "b"));
        }
        out
    }

    pub fn metrics(&self) -> Vec<f64> {
        let n = self.state.entities.len() as f64;
        let mut ma = 0.0;
        let mut mb = 0.0;
        for entity in self.state.entities.values() {
            ma += number(&entity.properties, "a");
            mb += number(&entity.properties, "b");
        }
        ma /= n;
        mb /= n;
        let mut va = 0.0;
        let mut vb = 0.0;
        for entity in self.state.entities.values() {
            va += (number(&entity.properties, "a") - ma).powi(2);
            vb += (number(&entity.properties, "b") - mb).powi(2);
        }
        vec![n, ma, mb, (va / n).sqrt(), (vb / n).sqrt()]
    }

    pub fn state_hash(&self) -> Result<String, JsValue> {
        self.state.state_hash().map(|h| format!("{h:016x}"))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn export_event_log(&self) -> Result<String, JsValue> {
        self.log.to_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn export_manifest(&self, build_commit: &str) -> Result<String, JsValue> {
        let terminal_state_hash = self.state_hash()?;
        let replay_state_hash = self.replay_hash()?;
        let mut parameters = Properties::new();
        parameters.insert("coupling".into(), Value::Float(self.coupling));
        parameters.insert("noise".into(), Value::Float(self.noise));
        parameters.insert("interaction_radius".into(), Value::Float(self.radius));
        let manifest = RunManifest {
            format: "meatware-run-manifest".into(),
            schema_version: 1,
            runtime: "meatware-wasm-0.1.0".into(),
            build_commit: build_commit.into(),
            model: "two-channel-local-exchange-v1".into(),
            random_generator: "splitmix64-v1".into(),
            random_stream_scheme: if self.used_event_local_streams { "sequential-plus-event-local-v1".into() } else { "single-sequential-stream-v1".into() },
            seed: self.seed as u64,
            initial_entities: self.initial_entities,
            parameters,
            conflict_policy: self.conflict_policy,
            initial_state_hash: self.initial_state_hash.clone(),
            transitions: self.state.boundary,
            event_records: self.log.records.len(),
            terminal_state_hash,
            replay_state_hash,
            determinism: "event-log replay checked against terminal state".into(),
        };
        manifest.to_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn replay_hash(&self) -> Result<String, JsValue> {
        let replayed = self.log.replay().map_err(|e| JsValue::from_str(&e.to_string()))?;
        let hash = replayed.state_hash().map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(format!("{hash:016x}"))
    }
}
