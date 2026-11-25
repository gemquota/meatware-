use meatware::{Configuration, Entity, Properties, Value};
fn main() {
    let mut state = Configuration::default();
    let mut properties = Properties::new();
    properties.insert("signal".into(), Value::Float(0.5));
    state.entities.insert(1, Entity { id: 1, properties });
    state.next_id = 2;
    println!("Meatware reference runtime");
    println!("boundary: {}", state.boundary);
    println!("entities: {}", state.entities.len());
    println!("relations: {}", state.relations.len());
    println!("state_hash: {:016x}", state.state_hash().expect("valid state"));
}
