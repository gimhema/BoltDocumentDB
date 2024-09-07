
use data_manager::*;
use query::*;
use crate::agent::*;
use crate::data::*;


// Main Process

struct BoltAgent {
    data_manager : BoltDataManager,
    query_interpreter : BoltInterpreter
}

impl BoltAgent {

    pub fn new() -> Self {
        return BoltAgent { data_manager: BoltDataManager::new(), 
            query_interpreter: BoltInterpreter::new() }
    }

    pub fn interprete(&mut self, input : String) {

        self.query_interpreter.set_input_query(input);

        let mut _result = self.query_interpreter.parse();
        let mut _queryType = _result.get_query_type();

        match _queryType {
            query::QueryType::NONE => println!("NONE"),
            query::QueryType::NEW => {
                println!("NEW");
                self.data_manager.new_doc_bundle(_result.get_bundle_name().clone());
            },
            query::QueryType::RESTORE => {
                println!("RESTORE");
                self.data_manager.restore_doc_bundle(); // Need Param
            },
            query::QueryType::BACKUP => {
                println!("BACKUP");
                self.data_manager.backup_doc_bundle(); // Need Param
            },
            query::QueryType::REMOVE => {
                println!("REMOVE");
                self.data_manager.remove_doc_bundle();  // Need Param
            },
            query::QueryType::CREATE => {
                println!("CREATE");
                self.data_manager.create(
                    _result.get_bundle_name().clone(), 
                    _result.get_document_name().clone(), 
                    _result.get_json_query().clone());
            },
            query::QueryType::READ => {
                println!("READ");
                self.data_manager.read(
                    _result.get_bundle_name().clone(), 
                    _result.get_document_name().clone(),
                    _result.get_key().clone());
            },
            query::QueryType::UPDATE => {
                println!("UPDATE");
                self.data_manager.update(
                    _result.get_bundle_name().clone(), 
                    _result.get_document_name().clone(),
                    _result.get_key().clone(),
                _result.get_json_query().clone());
            },
            query::QueryType::DELETE => {
                println!("DELETE");
                self.data_manager.delete(
                    _result.get_bundle_name().clone(), 
                    _result.get_document_name().clone(), 
                    _result.get_key().clone());
            },
            query::QueryType::SET => {
                println!("SET");
                self.data_manager.set_data(
                    _result.get_bundle_name().clone(), 
                    _result.get_document_name().clone(), 
                    _result.get_key().clone(), 
                    _result.get_json_key().clone(),
                    _result.get_json_value().clone());
            },
            query::QueryType::GET => {
                println!("GET");
                self.data_manager.get_data(
                    _result.get_bundle_name().clone(), 
                    _result.get_document_name().clone(), 
                    _result.get_key().clone(), 
                    _result.get_json_key().clone());
            },
            _ => println!("Parse Error"),
        }
    }

}