#[macro_use]
extern crate lazy_static;

mod data;
mod connection;
mod agent;
use agent::bolt_agent::*;
use connection::conn::BoltDBConnection;

use std::sync::{RwLock, Arc};

lazy_static! {
    static ref BOLT_AGENT_INSTANCE: Arc<RwLock<BoltAgent>> = Arc::new(RwLock::new(BoltAgent::new()));
    static ref BOLT_CONNECTION: Arc<RwLock<BoltDBConnection>> = Arc::new(RwLock::new(BoltDBConnection::new()));
}

impl BoltAgent {
    pub fn get_instance() -> &'static Arc<RwLock<BoltAgent>> {
        &BOLT_AGENT_INSTANCE
    }
}

impl BoltDBConnection {
    pub fn get_instance() -> &'static Arc<RwLock<BoltDBConnection>> {
        &BOLT_CONNECTION
    }
}

fn main() {
    
    print!("Start Bolt DB ");

    BoltAgent::get_instance().write().unwrap().wait(); // 비동기 처리 필요
    BoltDBConnection::get_instance().write().unwrap().wait(); // 비동기 처리 필요

}
