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

    pub fn get(&self, key: &String) -> &Value {
        //println!("tried to get {:#?}", self.adapter.get(&key.to_owned()));
        let raw_content = self.adapter.get(key).unwrap();
        raw_content
    }

    pub fn create_or_replace(&mut self, key: String, value: Value) -> bool {
        // println!("tried to set {} to {:#?}", key, value);
        self.adapter.insert(key, value);
        true
    }
}
