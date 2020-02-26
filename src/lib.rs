mod handle_client;
mod parse_request;
mod storage;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use handle_client::handle_client;
use std::sync::{Arc, Mutex};
use storage::{in_memory_adapter, StorageAdapter};

pub async fn initialize(storage_data: Arc<Mutex<StorageAdapter>>) {
    let listener = TcpListener::bind(String::from("0.0.0.0:7272"))
        .await
        .unwrap();

    while let Some(stream) = listener.incoming().next().await {
        let stream = stream.unwrap();
        let storage_data_clone = storage_data.clone();

        task::spawn(handle_client(stream, storage_data_clone));
    }
}

pub async fn run() {
    let storage_data = in_memory_adapter();
    let storage_arc = Arc::new(Mutex::new(storage_data));

    initialize(storage_arc).await
}
