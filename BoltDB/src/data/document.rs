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

    pub fn get_data(&self, _docs_key : i64, _json_key : String) -> String {
        let mut _result = "".to_string();

        _result = self.document.get(&_docs_key).unwrap().get_obj(_json_key.clone());

        return _result
    }

    pub fn set_data(&mut self, _docs_key : i64, _json_key : String, _value : String) {
        
        if let Some(data_obj) = self.document.get_mut(&_docs_key) {

            data_obj.set_obj(_json_key.clone(), _value.clone());

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

    pub fn get_obj(&self, key: String) -> String {
        if let Value::Object(ref map) = self.data {
            // 키에 해당하는 값을 가져와서 String으로 변환
            if let Some(value) = map.get(&key) {
                return value.to_string();
            }
        }
        // 키가 없거나 값이 Object가 아닐 경우 빈 문자열 반환
        "".to_string()
    }

    pub fn set_obj(&mut self, key: String, value: String) {
        if let Value::Object(ref mut map) = self.data {
            // 기존 맵에 새로운 key-value 쌍 추가
            map.insert(key, Value::String(value));
        }
    }
}