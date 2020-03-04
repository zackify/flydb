use super::StorageAdapter;
use serde_json::Value;
use std::collections::HashMap;

pub struct InMemoryAdapter {
    adapter: HashMap<String, Value>,
}

impl InMemoryAdapter {
    pub fn new() -> InMemoryAdapter {
        let adapter = HashMap::new();
        InMemoryAdapter { adapter }
    }
}

impl StorageAdapter for InMemoryAdapter {
    fn get(&self, key: &String) -> Value {
        //println!("tried to get {:#?}", self.adapter.get(&key.to_owned()));
        let raw_content = self.adapter.get(key).unwrap();
        raw_content.clone()
    }

    fn create_or_replace(&mut self, key: String, value: Value) -> bool {
        // println!("tried to set {} to {:#?}", key, value);
        self.adapter.insert(key, value);
        true
    }
}
