use super::parse_request::parse_request;
use super::storage::StorageAdapter;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use std::sync::{Arc, Mutex};

pub async fn handle_client(mut stream: TcpStream, storage: Arc<Mutex<dyn StorageAdapter + Send>>) {
    loop {
        let mut reader = BufReader::new(&mut stream);
        let mut content = String::new();

        match reader.read_line(&mut content).await {
            Err(e) => eprintln!("{:#?} error!!!", e),
            Ok(result) => {
                if result == 0 {
                    break;
                }
                let response = parse_request(content, &storage);

                stream
                    .write(format!("{}\n", response).as_bytes())
                    .await
                    .unwrap();
            }
        }
    }
}
