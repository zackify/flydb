use super::Message;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::sync::mpsc::Sender;

#[derive(Deserialize, Debug)]
struct JsonRequest {
    path: String,
    method: String,
    #[serde(default)]
    doc: Value,
}

pub async fn handle_client(mut stream: TcpStream, tx: Sender<Message>) {
    loop {
        let mut reader = BufReader::new(&mut stream);
        let mut content = String::new();

        match reader.read_line(&mut content).await {
            Err(e) => eprintln!("{:#?}", e),
            Ok(result) => {
                if result == 0 {
                    break;
                }
                let json: JsonRequest = serde_json::from_str(&content).unwrap();

                let response = match json.method.as_str() {
                    "create_or_replace" => {
                        let response = String::from(format!("{{\"path\": \"{}\"}}", &json.path));
                        println!("{}", &json.path);
                        tx.send(Message {
                            method: "create_or_replace".to_string(),
                            key: json.path,
                            value: json.doc,
                        })
                        .unwrap();

                        response
                    }
                    "get" => {
                        //if there is no method, we default to getting the document
                        //let data = storage.get(&json.path);
                        // let content = serde_json::to_string(&data).unwrap();
                        //println!("{:#?}", content);
                        // String::from(format!(
                        //     "{{\"path\": \"{}\", \"doc\": {}}}",
                        //     &json.path, content
                        // ))
                        String::from("{{}}")
                    }
                    unknown => {
                        println!("Unsupported method '{}' was called", unknown);
                        String::from("{\"path\": \"blah\"}")
                    }
                };

                stream
                    .write(format!("{}\r\n", response).as_bytes())
                    .await
                    .unwrap();
            }
        }
    }
}
