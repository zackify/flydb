pub mod file;
pub mod in_memory;
pub use file::FileAdapter;
pub use in_memory::InMemoryAdapter;
use serde_json::Value;

pub trait StorageAdapter {
    fn get(&self, key: &String) -> Value;
    fn create_or_replace(&mut self, key: String, value: Value) -> bool;
}
