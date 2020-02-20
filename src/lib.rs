mod handle_client;
mod parse_request;
mod storage;
use handle_client::{handle_client, ClientData};
use std::net::TcpListener;

pub fn listen(address: String) -> TcpListener {
    TcpListener::bind(address).unwrap()
}

pub fn run() {
    let listener = listen(String::from("0.0.0.0:7272"));
    for stream in listener.incoming() {
        handle_client(ClientData {
            stream: &mut stream.unwrap(),
        });
    }
}
