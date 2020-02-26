pub mod file;
pub mod in_memory;
pub use file::FileAdapter;
pub use in_memory::InMemory;
use serde_json::Value;

pub enum StorageType {
    InMemory(InMemory),       // File,
    FileAdapter(FileAdapter), // FileAndMemory
}

pub struct StorageAdapter {
    pub adapter: StorageType,
}

impl StorageAdapter {
    pub fn new(adapter: StorageType) -> StorageAdapter {
        StorageAdapter { adapter }
    }

    pub fn get(&self, key: &String) -> Value {
        match &self.adapter {
            StorageType::FileAdapter(store) => store.get(&key),
            StorageType::InMemory(store) => store.get(&key),
        }
    }

    pub fn create_or_replace(&mut self, key: String, value: Value) -> bool {
        match &mut self.adapter {
            StorageType::FileAdapter(store) => store.create_or_replace(key, value),
            StorageType::InMemory(store) => store.create_or_replace(key, value),
        }
    }
}

pub fn in_memory_adapter() -> StorageAdapter {
    StorageAdapter::new(StorageType::InMemory(InMemory::new()))
}

pub fn file_adapter() -> StorageAdapter {
    StorageAdapter::new(StorageType::FileAdapter(FileAdapter::new()))
}
