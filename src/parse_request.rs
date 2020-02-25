//This file is responsible for handling the correct action based on the message we receive
use super::storage::in_memory::InMemory;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
pub struct Message {
    path: String,
    method: String,
    #[serde(default)]
    doc: Value,
}

#[derive(Deserialize, Debug)]
pub struct JsonRequest {
    messages: Vec<Message>,
    id: usize,
    #[serde(default)]
    kind: String,
}

pub fn parse_request(content: String, storage: &Arc<Mutex<InMemory>>) -> String {
    let json: JsonRequest = serde_json::from_str(&content).unwrap();
    let mut store = storage.lock().unwrap();

    if json.kind == "success_only" {
        for item in json.messages {
            handle_message(item, &mut store);
        }
        let response = json!({
            "success": true,
            "id": json.id,
        });
        serde_json::to_string(&response).unwrap()
    } else {
        let mut messages = vec![];

        for item in json.messages {
            messages.push(handle_message(item, &mut store));
        }
        let response = json!({
            "messages": messages,
            "id": json.id,
        });
        serde_json::to_string(&response).unwrap()
    }
}

pub fn handle_message(json: Message, store: &mut InMemory) -> Value {
    match json.method.as_str() {
        "create_or_replace" => {
            let response = json!("");
            store.create_or_replace(json.path, json.doc);

            response
        }
        "get" => {
            //if there is no method, we default to getting the document
            let data = store.get(&json.path);
            json!({
                "path": &json.path,
                "doc": &data
            })
        }
        unknown => {
            println!("Unsupported method '{}' was called", unknown);
            json!({
                "path": "",
                "error": "Unsupported method"
            })
        }
    }
}
