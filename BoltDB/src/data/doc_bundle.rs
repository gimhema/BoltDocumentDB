use std::collections::HashMap;
use super::document::*;


pub struct BoltDocumentBundle {
    document_bundle : HashMap<String, BoltDocument>
}

impl BoltDocumentBundle {
    pub fn new() -> Self {
        return BoltDocumentBundle{document_bundle: HashMap::new()}
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

    pub fn delete_object_in_document(&mut self, docs_name : String, key : i64,  merge_query : String) {
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
    
}