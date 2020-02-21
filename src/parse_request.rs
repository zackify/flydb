//This file is responsible for handling the correct action based on the message we receive
use super::storage::in_memory::InMemory;
use serde::Deserialize;
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
struct JsonRequest {
    path: String,
    method: String,
    #[serde(default)]
    doc: Value,
}

pub fn parse_request(content: String, storage: &Arc<Mutex<InMemory>>) -> String {
    let json: JsonRequest = serde_json::from_str(&content).unwrap();

    let mut store = storage.lock().unwrap();
    match json.method.as_str() {
        "create_or_replace" => {
            let response = String::from(format!("{{\"path\": \"{}\"}}", &json.path));
            println!("{}", &json.path);
            store.create_or_replace(json.path, json.doc);

            response
        }
        "get" => {
            //if there is no method, we default to getting the document
            let data = store.get(&json.path);
            let content = serde_json::to_string(&data).unwrap();
            println!("{:#?}", content);
            String::from(format!(
                "{{\"path\": \"{}\", \"doc\": {}}}",
                &json.path, content
            ))
        }
        unknown => {
            println!("Unsupported method '{}' was called", unknown);
            String::from("{\"path\": \"blah\"}")
        }
    }
}
