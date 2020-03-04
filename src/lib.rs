mod handle_client;
mod parse_request;
mod storage;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use handle_client::handle_client;
use storage::{create_adapter, Storage};

pub async fn initialize(storage: Storage) {
    let listener = TcpListener::bind(String::from("0.0.0.0:7272"))
        .await
        .unwrap();

    while let Some(stream) = listener.incoming().next().await {
        let stream = stream.unwrap();
        let storage_clone = storage.clone();

        task::spawn(handle_client(stream, storage_clone));
    }
}

pub async fn run() {
    let storage = create_adapter(String::from("InMemory"));

    initialize(storage).await
}
