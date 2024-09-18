
use std::str::from_utf8;
use mio::event::Event;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::{net, thread, time};
use std::sync::{RwLock, Arc, RwLockReadGuard};
use mio::net::{TcpListener, TcpStream};

use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};


pub struct BoltDBConnection {
    ip_address : String,
    port : i64,
}

impl BoltDBConnection {
    pub fn new() -> Self {
        return BoltDBConnection{ ip_address : "".to_string(), port : 0 }
    }

    pub fn init_connect_info(&mut self, _ip_address : String, _port : i64) {
        self.ip_address = _ip_address.clone();
        self.port = _port;
    }

    pub fn wait() {
        
        // Spin . . .

    }
    
}

