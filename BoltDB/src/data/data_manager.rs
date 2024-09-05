use std::collections::HashMap;

use super::doc_bundle::*;
use super::document::*;


struct BoldtDataManager {
    db_map : HashMap<String, BoltDocumentBundle>
}

impl BoldtDataManager {

    pub fn new() -> Self {
        return BoldtDataManager{db_map :HashMap::new()}
    }

    pub fn restore_doc_bundle(&mut self) {
        
    }

    pub fn backup_doc_bundle(&mut self) {

    }

    pub fn new_doc_bundle(&mut self, bundle_name : String) {

        let mut _bundle_name = bundle_name.clone();

        self.db_map.insert(bundle_name, BoltDocumentBundle::new(_bundle_name));
    }

    // document bundle crud
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

}