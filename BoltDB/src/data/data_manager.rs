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

    pub fn new_doc_bundle(&mut self, bundle_name : String) {
        self.db_map.insert(bundle_name, BoltDocumentBundle::new());
    }

}