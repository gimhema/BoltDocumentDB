
use data_manager::BoltDataManager;

use crate::data::*;


// Main Process

struct BoltAgent {

    dataManager : BoltDataManager

}

impl BoltAgent {

    pub fn new() -> Self {
        return BoltAgent { dataManager: BoltDataManager::new() }
    }

}