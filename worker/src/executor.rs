//! Worker execute.

use signal::{Category, Signal};
use std::net::TcpStream;
use uuid::Uuid;

/// Executor struct.
pub struct Executor {
    /// Worker id.
    id: Uuid,
    /// Scheduler address.
    scheduler: String,
}

impl Executor {
    /// Create a new executor.
    pub fn new(scheduler: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, scheduler }
    }

    /// Start executor.
    pub fn start(&self) {
        println!("start executor");
    }

    /// Register to scheduler.
    pub fn register(&self) {
        // Connect to scheduler.
        let stream = TcpStream::connect(self.scheduler.clone());
        match stream {
            Ok(stream) => {
                let mut stream = stream.try_clone().unwrap();
                let signal = Signal::from_stream(&mut stream);

                match signal {
                    Some(signal) => {
                        if signal.category == Category::RegisterBack {
                            if signal.effect {
                                println!("register success");
                            } else {
                                println!("register failed");
                            }
                        } else {
                            println!("signal is not register back");
                        }
                    }
                    None => {
                        println!("signal analysis failed, return None");
                    }
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }

    /// Stop executor.
    pub fn stop(&self) {
        println!("stop executor");
    }

    /// Restart executor.
    pub fn restart(&self) {
        println!("restart executor");
    }
}
