pub mod file;
pub mod in_memory;

use serde_json::Value;
use std::sync::{Arc, Mutex};

pub trait StorageAdapter {
    fn get(&self, key: &String) -> Value;
    fn create_or_replace(&mut self, key: String, value: Value) -> bool;
}

// Public interface, helper type alias and function to return an arc, mutexed storage adapter
pub type Storage = Arc<Mutex<dyn StorageAdapter + Send>>;

// list of adapters go here
pub fn in_memory() -> Storage {
    Arc::new(Mutex::new(in_memory::InMemoryAdapter::new()))
}

pub fn file_adapter() -> Storage {
    Arc::new(Mutex::new(file::FileAdapter::new()))
}

// take cli argument and return the adapter
pub fn create_adapter(kind: String) -> Storage {
    match kind.as_str() {
        "FileAdapter" => file_adapter(),
        "InMemory" => in_memory(),
        _ => in_memory(),
    }
}
