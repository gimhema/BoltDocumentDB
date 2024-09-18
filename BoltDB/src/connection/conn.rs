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

use super::security::*;

const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 1000;


pub struct BoltDBConnection {
    ip_address : String,
    port : i64,
    connections : HashMap<Token, TcpStream>
}

impl BoltDBConnection {
    pub fn new() -> Self {
        return BoltDBConnection{ ip_address : "".to_string(), port : 0, connections : HashMap::new() }
    }

    pub fn init_connect_info(&mut self, _ip_address : String, _port : i64) {
        self.ip_address = _ip_address.clone();
        self.port = _port;
    }

    pub fn wait(&mut self) -> io::Result<()> 
    {
        env_logger::init();

        let mut userCount: i64 = 0;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let mut server = TcpListener::bind("127.0.0.1:8080".parse().unwrap())?;
    
        // Register the server with poll we can receive events for it.
        poll.registry().register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    
        // Map of `Token` -> `TcpStream`.
        // let mut connections = HashMap::new();

        let mut unique_token = Token(SERVER.0 + 1);


        loop {
            // println!("Set Poll Step : {}", self.step);
            poll.poll(&mut events, Some(Duration::from_millis(SERVER_TICK)))?;
    
            // println!("Iterate Event For Loop");
            for event in events.iter() {
                if event.token() == Token(0) && event.is_writable() {
                    println!("Writeable Event . . .");
                }
    
                match event.token() {
                    SERVER => loop {
                        // Received an event for the TCP server socket, which
                        // indicates we can accept an connection.
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) =>  (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // If we get a `WouldBlock` error we know our
                                // listener has no more incoming connections queued,
                                // so we can return to polling and wait for some
                                // more.
                                break;
                            }
                            Err(e) => {
                                // If it was any other kind of error, something went
                                // wrong and we terminate with an error.
                                return Err(e);
                            }
                        };
                        println!("Accepted connection from: {}", address);
    
                        let token = next(&mut unique_token);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;
                        println!("Add New Player");
                        let mut sendConnect = connection;
                        
                        // get_connection_handler().write().unwrap().new_tcp_connection(sendConnect, token);
                        // get_user_connection_info().write().unwrap().push(token);

                        println!("SendGamePacket End");
                    },
                    token => {

                        let done = {
                            // let mut handler = get_connection_handler().write().unwrap();
                            // if let Some(connection) = handler.get_tcp_connection_by_token(token) {
                            //     println!("Handle Connection Event");
                            //     handle_connection_event(poll.registry(), connection, event)?
                            // } else {
                            //     // Sporadic events happen, we can safely ignore them.
                            //     false
                            // }
                        };


                       //if done {
                       //         println!("Disconn search . . .");
                       //         if let Some(mut connection)  = 
                       //         get_connection_handler().write().unwrap().get_tcp_connection_by_token(token)
                       //         {
                       //             println!("User Disconnected . . 1");
                       //             poll.registry().deregister(connection);
                       //             
                       //             get_connection_handler().write().unwrap().del_tcp_connection(token);
                       //             // get_send_connection_handler().write().unwrap().del_tcp_connection(token);
                       //             // self.remove_connection(token);
                       //         }
                       //     }
                    }
                }
                // thread::sleep(Duration::from_secs(1));
            }
            // self.step += 1;
            // println!("server run...")
        }

    }
    
}


fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    println!("Handle Connection Event Start . . ");

    if event.is_readable() {
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        // We can (maybe) read from the connection.
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {

            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {

                let vec_of_bytes: Vec<u8> = received_data.to_vec();
                let recvMsg = String::from(from_utf8(received_data).unwrap());
                // listen_event(recvMsg);
                
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }
    println!("Handle Connection Event End . . ");
    Ok(false)
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}

