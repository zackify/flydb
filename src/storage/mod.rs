pub mod in_memory;
use in_memory::InMemory;

pub enum StorageType {
    InMemory,
    // File,
    // FileAndMemory,
}

pub fn get_storage(kind: StorageType) -> InMemory {
    match kind {
        StorageType::InMemory => InMemory::new(),
        // _ => Storage {
        //     get: in_memory::get,
        //     set: in_memory::set,
        // },
    }
}
