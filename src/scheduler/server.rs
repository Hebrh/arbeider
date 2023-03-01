//! Run scheduler server.

use std::net::TcpListener;

pub struct Server {
    /// Address of scheduler server.
    pub addr: String,
    /// The port of scheduler server.
    pub port: u16,
}

impl Server {
    /// New scheduler server.
    pub fn new(addr: String, port: u16) -> Self {
        Self { addr, port }
    }

    /// Run scheduler server.
    pub fn run(&self) {
        let addr = format!("{}:{}", self.addr, self.port);
        let listener = TcpListener::bind(&addr);

        match listener {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            println!("New connection: {}", stream.peer_addr().unwrap());
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
