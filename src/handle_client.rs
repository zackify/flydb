use super::parse_request::parse_request;
use super::storage;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct ClientData<'a> {
    pub stream: &'a mut TcpStream,
}

pub fn handle_client(data: ClientData) {
    let mut connection = BufReader::new(data.stream);
    let mut storage_data = storage::get_storage(storage::StorageType::InMemory);

    loop {
        let mut content = String::new();

        match connection.read_line(&mut content) {
            Err(e) => eprintln!("{:#?}", e),
            Ok(result) => {
                if result == 0 {
                    break;
                }
                parse_request(content, &mut storage_data);
            }
        }
    }
}
