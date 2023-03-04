//! Scheduler library.

pub mod signal;

use crate::task::Task;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use uuid::Uuid;

/// Scheduler server struct.
pub struct Scheduler {
    /// The scheduler id.
    pub id: Uuid,
    /// The scheduler address.
    pub address: String,
    /// The scheduler port.
    pub port: u16,
    /// The scheduler tasks.
    pub tasks: Vec<Task>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            address: String::from("http://localhost"),
            port: 27021,
            tasks: Vec::new(),
        }
    }
}

impl Scheduler {
    /// New scheduler.
    pub fn new(id: Uuid, address: String, port: u16) -> Self {
        Self {
            id,
            address: address.clone(),
            port,
            tasks: Vec::new(),
        }
    }

    /// Run scheduler.
    pub fn run(&mut self) {
        // spawn a process to run server
        let addr = format!("{}:{}", self.address, self.port);
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
