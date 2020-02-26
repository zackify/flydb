use super::StorageAdapter;
use serde_json::Value;
use std::collections::HashMap;

pub struct File {
    adapter: HashMap<String, Value>,
}

impl File {
    pub fn new() -> File {
        let adapter = HashMap::new();
        File { adapter }
    }
}

impl StorageAdapter for File {
    fn get(&self, key: &String) -> &Value {
        //println!("tried to get {:#?}", self.adapter.get(&key.to_owned()));
        let raw_content = self.adapter.get(key).unwrap();
        raw_content
    }

    fn create_or_replace(&mut self, key: String, value: Value) -> bool {
        // println!("tried to set {} to {:#?}", key, value);
        self.adapter.insert(key, value);
        true
    }
}
