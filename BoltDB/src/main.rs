
mod data;
mod connection;
mod agent;

use agent::bolt_agent::*;

fn main() {
    
    print!("Start Bolt DB ");

    let mut bolt_agent = BoltAgent::new();

    bolt_agent.wait();

}
