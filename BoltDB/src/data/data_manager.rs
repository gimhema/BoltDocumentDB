use std::collections::HashMap;

use super::doc_bundle::*;
use super::document::*;


pub struct BoltDataManager {
    db_map : HashMap<String, BoltDocumentBundle>
}

impl BoltDataManager {

    pub fn new() -> Self {
        return BoltDataManager{db_map :HashMap::new()}
    }

    // document bundle control
    pub fn new_doc_bundle(&mut self, bundle_name : String) {

        let mut _bundle_name = bundle_name.clone();

        self.db_map.insert(bundle_name, BoltDocumentBundle::new(_bundle_name));
    }

    pub fn restore_doc_bundle(&mut self) {
        
    }

    pub fn backup_doc_bundle(&mut self) {

    }

    pub fn remove_doc_bundle(&mut self) {
        
    }

    // data control
    pub fn create(&mut self, bundle_name : String, docs_name : String, query : String) {
        if let Some(bundle) = self.db_map.get_mut(&docs_name) {
            bundle.add_object_to_document(docs_name, query);
        }
    }

    pub fn update(&mut self, bundle_name : String, docs_name : String, key : i64, query : String) {
        if let Some(bundle) = self.db_map.get_mut(&docs_name) {
            bundle.update_object_to_document(docs_name, key, query);
        }        
    }

    pub fn delete(&mut self, bundle_name : String, docs_name : String, key : i64) {
        if let Some(bundle) = self.db_map.get_mut(&docs_name) {
            bundle.delete_object_in_document(docs_name, key);
        }
    }

    pub fn read(&mut self, bundle_name : String, docs_name : String, key : i64)  -> Option<&BoltDataObject>  {
        if let Some(bundle) = self.db_map.get_mut(&docs_name) {
            let _result = bundle.get_object_from_document(docs_name, key);

            return _result
        }
        // Return None if the document or object is not found
        None
    }

    pub fn get_data(&mut self, bundle_name : String, 
        docs_name : String, key : i64, json_key : String) -> String {
        
        let mut _result = "".to_string();


        if let Some(bundle) = self.db_map.get_mut(&docs_name) {
            let _dataObj = bundle.get_object_from_document(docs_name, key);

            // _result = _dataObj.unwrap().
        }



        return _result;
    }

    pub fn set_data(&mut self, bundle_name : String, 
        docs_name : String, key : i64, json_key : String, json_value : String){
        
    }

}