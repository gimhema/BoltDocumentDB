use std::{collections::HashMap, hash::Hash};
use serde_json::{Value, Error};

pub struct BoltDocument {
    document : HashMap<i64, BoltDataObject>,
    top_key : i64
}

impl BoltDocument {
    
    pub fn new() -> Self {
        return BoltDocument{document : HashMap::new(), top_key : 0}
    }

    pub fn create_object(&mut self, query : String) {

        let parsed_result: Result<Value, Error> = serde_json::from_str(&query);

        match parsed_result {
            Ok(parsed) => {
                self.top_key += 1;
                self.document.insert(self.top_key.clone(), BoltDataObject::new(parsed));
                
            }
            Err(e) => {
                println!("Failed to parse JSON: {}", e);
            }
        }
    }

    pub fn read_object(&mut self, key : i64) -> Option<&BoltDataObject> {
        return self.document.get(&key)
    }

    pub fn update_object(&mut self, key : i64, query : String) {
        if let Some(existing_obj) = self.document.get_mut(&key) {
            let parsed_result: Result<Value, Error> = serde_json::from_str(&query);

            match parsed_result {
                Ok(new_data) => {
                    // Update the existing object with the new data
                    BoltDocument::merge_json(&mut existing_obj.data, &new_data);
                }
                Err(e) => {
                    println!("Failed to parse JSON: {}", e);
                }
            }
        } else {
            println!("No document found with the key: {}", key);
        }
    }

    pub fn delete_object(&mut self, key : i64) {
        self.document.remove(&key);
    }

    fn merge_json(existing: &mut Value, updates: &Value) {
        if let (Value::Object(existing_map), Value::Object(updates_map)) = (existing, updates) {
            for (key, value) in updates_map {
                match value {
                    Value::Object(_) => {
                        if let Some(existing_value) = existing_map.get_mut(key) {
                            BoltDocument::merge_json(existing_value, value);
                        } else {
                            existing_map.insert(key.clone(), value.clone());
                        }
                    }
                    _ => {
                        existing_map.insert(key.clone(), value.clone());
                    }
                }
            }
        }
    }
}

pub struct BoltDataObject {
    data : Value
}

impl BoltDataObject {
    pub fn new(_data : Value) -> Self {
        return BoltDataObject { data: _data }
    }
}