#[macro_use]
extern crate lazy_static;

mod data;
mod connection;
mod agent;
use agent::bolt_agent::*;

use std::sync::{RwLock, Arc};

lazy_static! {
    static ref BOLT_AGENT_INSTANCE: Arc<RwLock<BoltAgent>> = Arc::new(RwLock::new(BoltAgent::new()));
}

impl BoltAgent {
    pub fn get_instance() -> &'static Arc<RwLock<BoltAgent>> {
        &BOLT_AGENT_INSTANCE
    }
}

fn main() {
    
    print!("Start Bolt DB ");

    BoltAgent::get_instance().write().unwrap().wait();

}
