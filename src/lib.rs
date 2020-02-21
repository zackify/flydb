mod handle_client;
mod parse_request;
mod storage;
use async_std::task;
use std::sync::{Arc,Mutex};
use async_std::net::TcpListener;
use async_std::prelude::*;
use handle_client::handle_client;
use storage::in_memory::InMemory;

pub async fn initialize(storage_data: Arc<Mutex<InMemory>>) {
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
    let storage_data = storage::get_storage(storage::StorageType::InMemory);
    let storage_arc = Arc::new(Mutex::new(storage_data));

    initialize(storage_arc).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    #[test]
    fn connect() {
        async_std::task::spawn(run());

        use std::io::{BufRead, BufReader, Write};
        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(500));

        TcpStream::connect("127.0.0.1:7272")
            .and_then(|mut stream| {
                println!("connected");
                stream.write(b"{\"path\":\"blah\",\"method\":\"create_or_replace\",\"doc\":{\"test\":true,\"zach\":{\"hello\":\"yes i am some json\"}}}\n").unwrap();
                let mut reader = BufReader::new(stream);
                let mut content = String::new();
                reader.read_line(&mut content).unwrap();
                    
                println!("{} here", content);
                Ok(())
                //stream.write(b"{\"path\":\"blah\",\"method\":\"get\"}").unwrap();

                //stream.shutdown(Shutdown::Both)
            })
            .map_err(|e| panic!("error: {:?}", e))
            .unwrap()
    }
}
