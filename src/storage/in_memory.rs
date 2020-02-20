use serde_json::Value;
use std::collections::HashMap;

pub struct InMemory {
    adapter: HashMap<String, Value>,
}

impl InMemory {
    pub fn new() -> InMemory {
        let adapter = HashMap::new();
        InMemory { adapter }
    }

    pub fn get(&self, key: &String) -> bool {
        println!("tried to get {:#?}", self.adapter.get(&key.to_owned()));
        serde_json::from_str::<Value>("{\"test\":\"true\"}").unwrap();
        true
    }

    pub fn create_or_replace(&mut self, key: String, value: Value) -> bool {
        println!("tried to set {} to {:#?}", key, value);
        self.adapter.insert(key, value);
        true
    }
}
