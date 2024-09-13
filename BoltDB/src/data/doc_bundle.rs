use std::collections::HashMap;
use super::document::*;


pub struct BoltDocumentBundle {
    name : String,
    document_bundle : HashMap<String, BoltDocument>,
    data_path : String
}

impl BoltDocumentBundle {
    pub fn new(_name : String) -> Self {
        return BoltDocumentBundle{name : _name, document_bundle: HashMap::new(), data_path : "".to_string()}
    }

    pub fn add_object_to_document(&mut self, docs_name : String, query : String) {
        if let Some(document) = self.document_bundle.get_mut(&docs_name) {
            // Try to get the object from the document by its key
            document.create_object(query);
        }        
    }

    pub fn update_object_to_document(&mut self, docs_name : String, key : i64, merge_query : String) {
        if let Some(document) = self.document_bundle.get_mut(&docs_name) {
            // Try to get the object from the document by its key
            document.update_object(key, merge_query);
        }
    }

    pub fn delete_object_in_document(&mut self, docs_name : String, key : i64) {
        if let Some(document) = self.document_bundle.get_mut(&docs_name) {
            // Try to get the object from the document by its key
            document.delete_object(key);
        }
    }

    pub fn get_object_from_document(&mut self, docs_name: String, key: i64) -> Option<&BoltDataObject> {
        // Try to find the document by its name (mutable borrow)
        if let Some(document) = self.document_bundle.get_mut(&docs_name) {
            // Try to get the object from the document by its key
            return document.read_object(key);
        }
        // Return None if the document or object is not found
        None
    }

    pub fn get_data(&mut self, docs_name: String, key: i64, _json_key : String) -> String {
        let _result = "".to_string();

        return _result
    }

    pub fn set_data(&mut self, docs_name: String, key: i64, _json_key : String, _value : String) {
        
    }

    pub fn get_data_path(&self) -> String {
        return self.data_path.clone()
    }

    pub fn set_data_path(&mut self, _path : String) {
        self.data_path = _path;
    }
    
    pub fn save_db_as_json(&mut self) {

    }

    pub fn load_db_from_json(&mut self) {
        
    }
}