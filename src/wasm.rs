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
    properties.insert("strength".into(), Value::Float(0.72));
    properties.insert("age".into(), Value::Integer(0));
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
    motion: f64,
    damping: f64,
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
            properties.insert("vx".into(), Value::Float(0.0));
            properties.insert("vy".into(), Value::Float(0.0));
            state.entities.insert(id, Entity { id, properties });
        }
        state.next_id = state.entities.len() as u64 + 1;
        let initial_hash = state.state_hash().map_err(|e| JsValue::from_str(&e.to_string()))?;
        let initial_entities = state.entities.len();
        let log = EventLog::new(&state).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(World {
            state, log, random,
            coupling: 0.8, noise: 0.008, radius: 0.055,
            motion: 1.0, damping: 0.985,
            seed, initial_entities, initial_state_hash: format!("{initial_hash:016x}"),
            conflict_policy: ConflictPolicy::Reject,
            used_event_local_streams: false,
        })
    }

    /// Form generic relationships between entities within the fixed experimental radius.
    /// All candidate pairs are evaluated from one immutable source boundary.
    pub fn form_relations(&mut self, radius: f64) -> Result<u32, JsValue> {
        let rule = InteractionRule::entities(
            0x4d45_4154, 1, 2, 125_000, create_relation_proposal,
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

    /// [multi-entity components, largest component, mean relation strength, mean degree].
    pub fn structural_metrics(&self) -> Vec<f64> {
        let ids: Vec<u64> = self.state.entities.keys().copied().collect();
        let mut parent: Vec<usize> = (0..ids.len()).collect();
        fn root(parent: &mut [usize], mut i: usize) -> usize { while parent[i] != i { parent[i] = parent[parent[i]]; i = parent[i]; } i }
        let mut degree=vec![0usize;ids.len()]; let mut strength=0.0; let mut edges=0usize;
        for relation in self.state.relations.values() {
            if relation.endpoints.len()!=2 {continue;}
            let (a,b)=match (&relation.endpoints[0],&relation.endpoints[1]){(Endpoint::Entity(a),Endpoint::Entity(b))=>(*a,*b),_=>continue};
            let (Some(i),Some(j))=(ids.iter().position(|x|*x==a),ids.iter().position(|x|*x==b))else{continue};
            let ri=root(&mut parent,i);let rj=root(&mut parent,j);parent[ri]=rj;degree[i]+=1;degree[j]+=1;edges+=1;
            strength+=number(&relation.properties,"strength");
        }
        let mut sizes=std::collections::BTreeMap::<usize,usize>::new();
        for i in 0..ids.len(){let r=root(&mut parent,i);*sizes.entry(r).or_default()+=1;}
        vec![sizes.values().filter(|&&s|s>1).count() as f64,sizes.values().copied().max().unwrap_or(0) as f64,if edges>0{strength/edges as f64}else{0.0},degree.iter().sum::<usize>() as f64/ids.len().max(1) as f64]
    }

    /// Structural diagnostics derived from the generic relation graph:
    /// [connected components, largest component, mean degree, edge count].
    pub fn structure_metrics(&self) -> Vec<f64> {
        let ids: Vec<u64> = self.state.entities.keys().copied().collect();
        let mut parent: Vec<usize> = (0..ids.len()).collect();
        fn root(parent: &mut [usize], mut i: usize) -> usize {
            while parent[i] != i {
                let next = parent[i];
                parent[i] = parent[next];
                i = parent[i];
            }
            i
        }
        let index: std::collections::BTreeMap<u64, usize> =
            ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
        let mut degree = vec![0usize; ids.len()];
        for relation in self.state.relations.values() {
            if relation.endpoints.len() != 2 { continue; }
            let (Endpoint::Entity(a), Endpoint::Entity(b)) =
                (&relation.endpoints[0], &relation.endpoints[1]) else { continue; };
            let (Some(&i), Some(&j)) = (index.get(a), index.get(b)) else { continue; };
            degree[i] += 1;
            degree[j] += 1;
            let ri = root(&mut parent, i);
            let rj = root(&mut parent, j);
            if ri != rj { parent[rj] = ri; }
        }
        let mut sizes = std::collections::BTreeMap::<usize, usize>::new();
        for i in 0..ids.len() {
            let r = root(&mut parent, i);
            *sizes.entry(r).or_default() += 1;
        }
        let components = sizes.len();
        let largest = sizes.values().copied().max().unwrap_or(0);
        let mean_degree = degree.iter().sum::<usize>() as f64 / ids.len().max(1) as f64;
        vec![components as f64, largest as f64, mean_degree, self.state.relations.len() as f64]
    }

    pub fn relation_strengths(&self) -> Vec<f64> {
        self.state.relations.values().map(|r| match r.properties.get("strength") { Some(Value::Float(v)) => *v, _ => 0.72 }).collect()
    }

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
            out.push(number(&relation.properties, "strength"));
        }
        out
    }

    /// Advance the substrate with inertial motion, short-range exclusion,
    /// channel-mediated attraction/repulsion and persistent relation tension.
    pub fn step(&mut self, steps: u32) -> Result<(), JsValue> {
        for _ in 0..steps.min(1000) {
            let ids: Vec<u64> = self.state.entities.keys().copied().collect();
            let index_by_id: std::collections::BTreeMap<u64, usize> =
                ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
            let n = ids.len();
            let mut da = vec![0.0; n];
            let mut db = vec![0.0; n];
            let mut fx = vec![0.0; n];
            let mut fy = vec![0.0; n];
            let positions: Vec<(f64, f64, f64, f64, f64, f64)> = ids.iter().map(|id| {
                let p = &self.state.entities[id].properties;
                (number(p, "x"), number(p, "y"), number(p, "a"), number(p, "b"),
                 number(p, "vx"), number(p, "vy"))
            }).collect();
            let r2 = self.radius * self.radius;

            // Pair forces are equal and opposite. Similar state vectors attract;
            // contrasting vectors repel. A steep short-range term prevents collapse.
            for i in 0..n {
                for j in (i + 1)..n {
                    let mut dx = positions[j].0 - positions[i].0;
                    let mut dy = positions[j].1 - positions[i].1;
                    if dx > 0.5 { dx -= 1.0; } else if dx < -0.5 { dx += 1.0; }
                    if dy > 0.5 { dy -= 1.0; } else if dy < -0.5 { dy += 1.0; }
                    let d2 = dx * dx + dy * dy;
                    if d2 >= r2 { continue; }
                    let d = d2.sqrt().max(0.002);
                    let proximity = 1.0 - d / self.radius.max(0.005);
                    let similarity = ((positions[i].2 - 0.5) * (positions[j].2 - 0.5)
                        + (positions[i].3 - 0.5) * (positions[j].3 - 0.5)) * 8.0;
                    let collision_radius = 0.018;
                    let exclusion = if d < collision_radius { -(collision_radius - d) * 0.12 } else { 0.0 };
                    let force = (similarity * 0.0011 * proximity + exclusion)
                        * self.coupling * self.motion;
                    let inv_d = 1.0 / d;
                    let px = dx * inv_d * force;
                    let py = dy * inv_d * force;
                    fx[i] += px; fy[i] += py;
                    fx[j] -= px; fy[j] -= py;
                    // Local velocity alignment creates coherent collective motion.
                    let align = proximity * self.coupling * self.motion * 0.018;
                    fx[i] += (positions[j].4 - positions[i].4) * align;
                    fy[i] += (positions[j].5 - positions[i].5) * align;
                    fx[j] += (positions[i].4 - positions[j].4) * align;
                    fy[j] += (positions[i].5 - positions[j].5) * align;

                    let exchange = proximity * self.coupling * 0.018;
                    da[i] += (positions[j].2 - positions[i].2) * exchange;
                    da[j] += (positions[i].2 - positions[j].2) * exchange;
                    db[i] += (positions[j].3 - positions[i].3) * exchange;
                    db[j] += (positions[i].3 - positions[j].3) * exchange;
                }
            }

            // Persistent relationships exert a weak spring-like pull toward their
            // recorded separation. They are not decorative lines detached from dynamics.
            for relation in self.state.relations.values() {
                if relation.endpoints.len() != 2 { continue; }
                let (a, b) = match (&relation.endpoints[0], &relation.endpoints[1]) {
                    (Endpoint::Entity(a), Endpoint::Entity(b)) => (*a, *b),
                    _ => continue,
                };
                let (Some(i), Some(j)) = (ids.iter().position(|id| *id == a), ids.iter().position(|id| *id == b)) else { continue; };
                let mut dx = positions[j].0 - positions[i].0;
                let mut dy = positions[j].1 - positions[i].1;
                if dx > 0.5 { dx -= 1.0; } else if dx < -0.5 { dx += 1.0; }
                if dy > 0.5 { dy -= 1.0; } else if dy < -0.5 { dy += 1.0; }
                let d = (dx * dx + dy * dy).sqrt().max(0.002);
                let rest = match relation.properties.get("distance_sq") {
                    Some(Value::Float(v)) => v.max(0.0).sqrt(),
                    _ => 0.04,
                };
                let strength = number(&relation.properties, "strength").clamp(0.0, 1.0);
                let tension = (d - rest).clamp(-0.15, 0.15) * 0.008 * strength * self.motion;
                let px = dx / d * tension;
                let py = dy / d * tension;
                fx[i] += px; fy[i] += py;
                fx[j] -= px; fy[j] -= py;
            }

            let boundary = self.state.boundary;
            let mut proposals = Vec::with_capacity(n + self.state.relations.len());
            for i in 0..n {
                let vx = (positions[i].4 * self.damping + fx[i]
                    + (self.random.unit_f64() - 0.5) * self.noise * 0.12).clamp(-0.009, 0.009);
                let vy = (positions[i].5 * self.damping + fy[i]
                    + (self.random.unit_f64() - 0.5) * self.noise * 0.12).clamp(-0.006, 0.006);
                let speed = (vx * vx + vy * vy).sqrt();
                let (vx, vy) = if speed > 0.006 { (vx * 0.006 / speed, vy * 0.006 / speed) } else { (vx, vy) };
                let a = (positions[i].2 + da[i] + (self.random.unit_f64() - 0.5) * self.noise).clamp(0.0, 1.0);
                let b = (positions[i].3 + db[i] + (self.random.unit_f64() - 0.5) * self.noise).clamp(0.0, 1.0);
                let x = (positions[i].0 + vx).rem_euclid(1.0);
                let y = (positions[i].1 + vy).rem_euclid(1.0);
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
                        Edit::SetEntityProperty { id, key: "vx".into(), value: Value::Float(vx) },
                        Edit::SetEntityProperty { id, key: "vy".into(), value: Value::Float(vy) },
                    ],
                });
            }
            let mut all_proposals = proposals;
            // Relationship strength responds to actual separation and proximity.
            let mut relation_updates = Vec::new();
            for relation in self.state.relations.values() {
                if relation.endpoints.len() != 2 { continue; }
                let (a,b)=match (&relation.endpoints[0],&relation.endpoints[1]) {
                    (Endpoint::Entity(a),Endpoint::Entity(b))=>(*a,*b), _=>continue
                };
                let (Some(&i),Some(&j))=(index_by_id.get(&a),index_by_id.get(&b)) else {continue};
                let mut dx=positions[j].0-positions[i].0;
                let mut dy=positions[j].1-positions[i].1;
                if dx>0.5{dx-=1.0}else if dx< -0.5{dx+=1.0}
                if dy>0.5{dy-=1.0}else if dy< -0.5{dy+=1.0}
                let distance=(dx*dx+dy*dy).sqrt();
                let rest=match relation.properties.get("distance_sq"){Some(Value::Float(v))=>v.max(0.0).sqrt(),_=>0.04};
                let old=match relation.properties.get("strength"){Some(Value::Float(v))=>*v,_=>0.72};
                let next=if distance<self.radius*0.8{old+0.0025}else{old-(0.001+(distance-rest).max(0.0)*0.035)};
                let mut edits=Vec::new();
                if next<=0.08 { edits.push(Edit::RetireRelation{id:relation.id}); }
                else {
                    edits.push(Edit::SetRelationProperty{id:relation.id,key:"strength".into(),value:Value::Float(next.min(1.0))});
                    let age=match relation.properties.get("age"){Some(Value::Integer(v))=>*v,_=>0};
                    edits.push(Edit::SetRelationProperty{id:relation.id,key:"age".into(),value:Value::Integer(age.saturating_add(1))});
                }
                relation_updates.push(Proposal{key:10_000_000+relation.id,source_boundary:boundary,reads:Vec::new(),edits});
            }
            all_proposals.extend(relation_updates);
            self.log.append_with_policy(&mut self.state, &all_proposals, self.conflict_policy)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }

    pub fn set_coupling(&mut self, value: f64) { self.coupling = value.clamp(0.0, 2.0); }
    pub fn set_noise(&mut self, value: f64) { self.noise = value.clamp(0.0, 0.1); }
    pub fn set_radius(&mut self, value: f64) { self.radius = value.clamp(0.005, 0.25); }
    pub fn set_motion(&mut self, value: f64) { self.motion = value.clamp(0.1, 3.0); }
    pub fn set_damping(&mut self, value: f64) { self.damping = value.clamp(0.90, 0.999); }
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

    pub fn velocities(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.state.entities.len() * 2);
        for entity in self.state.entities.values() {
            out.push(number(&entity.properties, "vx"));
            out.push(number(&entity.properties, "vy"));
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
        parameters.insert("motion_strength".into(), Value::Float(self.motion));
        parameters.insert("velocity_damping".into(), Value::Float(self.damping));
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
