
use data_manager::BoltDataManager;
use query::BoltInterpreter;
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

}