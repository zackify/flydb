pub mod in_memory;
pub use in_memory::InMemory;

pub enum StorageType {
    InMemory(InMemory), // File,
                        // FileAndMemory
}

pub struct StorageAdapter {
    pub adapter: StorageType,
}

impl StorageAdapter {
    pub fn new(adapter: StorageType) -> StorageAdapter {
        StorageAdapter { adapter }
    }
}

pub fn in_memory_adapter() -> StorageAdapter {
    StorageAdapter::new(StorageType::InMemory(InMemory::new()))
}
