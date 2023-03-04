//! Server and protocol for transmitting data.

use crate::task::Task;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

/// Transmit server struct
pub struct Server {
    /// Address of transmit server.
    pub addr: String,
    /// The port of transmit server.
    pub port: u16,
    /// The tasks of transmit server.
    pub tasks: Vec<Task>,
}

impl Server {
    /// New transmit server.
    pub fn new(addr: String, port: u16, tasks: Vec<Task>) -> Self {
        Self { addr, port, tasks }
    }

    /// Run transmit server.
    pub fn run(&mut self) {
        let addr = format!("{}:{}", self.addr, self.port);
        let listener = TcpListener::bind(&addr);

        match listener {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let mut stream = stream;
                            let task = self.parse(&mut stream);

                            // put task to queue
                            self.tasks.push(task.unwrap());
                        }
                        Err(e) => {
                            println!("Error: {e}");
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }

    /// Parse the task from stream.
    pub fn parse(&self, stream: &mut TcpStream) -> Option<Task> {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        let task = String::from_utf8_lossy(&buf[..]);
        let task = task.trim_matches(char::from(0));
        let task: Task = serde_json::from_str(task).unwrap();

        Some(task)
    }
}
