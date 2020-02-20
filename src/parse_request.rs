//This file is responsible for handling the correct action based on the message we receive
use super::storage::in_memory::InMemory;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct JsonRequest {
    path: String,
    method: String,
    #[serde(default)]
    doc: Value,
}

pub fn parse_request(content: String, storage: &mut InMemory) -> String {
    let json: JsonRequest = serde_json::from_str(&content).unwrap();

    match json.method.as_str() {
        "create_or_replace" => {
            storage.create_or_replace(json.path, json.doc);
            String::from("{\"path\": \"blah\"}")
        }
        "get" => {
            //if there is no method, we default to getting the document
            let data = storage.get(&json.path);
            let content = serde_json::to_string(&data).unwrap();
            println!("{:#?}", content);
            content
        }
        unknown => {
            println!("Unsupported method '{}' was called", unknown);
            String::from("{\"path\": \"blah\"}")
        }
    }
}
