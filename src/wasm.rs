use crate::event_log::EventLog;
use crate::{Configuration, Edit, Entity, Properties, Proposal, RandomStream, Value};
use wasm_bindgen::prelude::*;

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
        let log = EventLog::new(&state).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(World {
            state, log, random,
            coupling: 0.8, noise: 0.008, radius: 0.055,
        })
    }

    pub fn step(&mut self, steps: u32) -> Result<(), JsValue> {
        for _ in 0..steps.min(1000) {
            let ids: Vec<u64> = self.state.entities.keys().copied().collect();
            let n = ids.len();
            let mut da = vec![0.0; n];
            let mut db = vec![0.0; n];
            let mut dxs = vec![0.0; n];
            let mut dys = vec![0.0; n];
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
                    edits: vec![
                        Edit::SetEntityProperty { id, key: "a".into(), value: Value::Float(a) },
                        Edit::SetEntityProperty { id, key: "b".into(), value: Value::Float(b) },
                        Edit::SetEntityProperty { id, key: "x".into(), value: Value::Float(x) },
                        Edit::SetEntityProperty { id, key: "y".into(), value: Value::Float(y) },
                    ],
                });
            }
            self.log.append(&mut self.state, &proposals)
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

    pub fn replay_hash(&self) -> Result<String, JsValue> {
        self.log.replay().and_then(|s| s.state_hash())
            .map(|h| format!("{h:016x}"))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
