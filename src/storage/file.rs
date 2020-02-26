use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub struct FileAdapter {}

impl FileAdapter {
    pub fn new() -> FileAdapter {
        FileAdapter {}
    }
    pub fn get(&self, key: &String) -> Value {
        let file = File::open(format!("backup/{}", key)).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).unwrap()
    }

    pub fn create_or_replace(&mut self, key: String, value: Value) -> bool {
        // println!("tried to set {} to {:#?}", key, value);
        std::fs::create_dir_all(format!("backup")).unwrap();
        let file = File::create(format!("backup/{}", key)).unwrap();

        serde_json::to_writer(&file, &value).unwrap();
        true
    }
}
