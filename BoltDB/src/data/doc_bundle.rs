use std::collections::HashMap;
use super::document::*;


pub struct BoltDocumentBundle {
    document_bundle : HashMap<String, BoltDocument>
}

impl BoltDocumentBundle {
    pub fn new() -> Self {
        return BoltDocumentBundle{document_bundle: HashMap::new()}
    }
}