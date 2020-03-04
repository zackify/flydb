//This file is responsible for handling the correct action based on the message we receive
use super::storage::StorageAdapter;
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

pub fn parse_request(content: String, storage: &Arc<Mutex<dyn StorageAdapter + Send>>) -> String {
    let json: JsonRequest = serde_json::from_str(&content).unwrap();

    if json.kind == "success_only" {
        for item in json.messages {
            handle_message(item, &storage);
        }
        let response = json!({
            "success": true,
            "id": json.id,
        });
        serde_json::to_string(&response).unwrap()
    } else {
        let mut messages = vec![];

        for item in json.messages {
            messages.push(handle_message(item, &storage));
        }
        let response = json!({
            "messages": messages,
            "id": json.id,
        });
        serde_json::to_string(&response).unwrap()
    }
}

pub fn handle_message(json: Message, storage: &Arc<Mutex<dyn StorageAdapter + Send>>) -> Value {
    let mut store = storage.lock().unwrap();

    match json.method.as_str() {
        "create_or_replace" => {
            let response = json!({"path": &json.path});
            store.create_or_replace(json.path, json.doc);

            response
        }
        "get" => {
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

// #[cfg(test)]
// mod tests {
//     use super::super::storage::in_memory_adapter;
//     use super::*;
//     #[test]
//     fn handle_success_only_create_or_replace() {
//         let mut store = Arc::new(Mutex::new(in_memory_adapter()));

//         //request from tcp client
//         let request = json!({
//             "id": 6,
//             "kind": "success_only",
//             "messages":[
//                 {
//                     "path": "blah" ,
//                     "method": "create_or_replace",
//                     "doc": {
//                         "test": "this just got inserted",
//                     }
//                 }
//             ]
//         })
//         .to_string();

//         // response from parse message, sending to tcp client
//         let response = json!({
//             "id": 6,
//             "success": true,
//         })
//         .to_string();

//         assert_eq!(parse_request(request, &mut store), response)
//     }

//     #[test]
//     fn handle_normal_create_or_replace() {
//         let mut store = Arc::new(Mutex::new(in_memory_adapter()));

//         //request from tcp client
//         let request = json!({
//             "id": 1,
//             "messages":[
//                 {
//                     "path": "blah" ,
//                     "method": "create_or_replace",
//                     "doc": {
//                         "test": "this just got inserted",
//                     }
//                 }
//             ]
//         })
//         .to_string();

//         // response from parse message, sending to tcp client
//         let response = json!({
//             "id": 1,
//             "messages": [
//                 {
//                     "path": "blah"
//                 }
//             ]
//         })
//         .to_string();

//         assert_eq!(parse_request(request, &mut store), response)
//     }

//     #[test]
//     fn handle_getting_stored_document() {
//         let mut store = Arc::new(Mutex::new(in_memory_adapter()));

//         //store the document so it is in the store for the test below
//         parse_request(
//             json!({
//                 "id": 1,
//                 "messages":[
//                     {
//                         "path": "blah" ,
//                         "method": "create_or_replace",
//                         "doc": {
//                             "test": "this just got inserted",
//                         }
//                     }
//                 ]
//             })
//             .to_string(),
//             &mut store,
//         );

//         //request from tcp client
//         let request = json!({
//             "id": 1,
//             "messages":[
//                 {
//                     "path": "blah" ,
//                     "method": "get",
//                 }
//             ]
//         })
//         .to_string();

//         // response from parse message, sending to tcp client
//         let response = json!({
//             "id": 1,
//             "messages": [
//                 {
//                     "path": "blah",
//                     "doc": {
//                         "test": "this just got inserted",
//                     }
//                 }
//             ]
//         })
//         .to_string();

//         assert_eq!(parse_request(request, &mut store), response,)
//     }
// }
