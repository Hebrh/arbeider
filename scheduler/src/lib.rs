//! Scheduler library.

use signal::{Category, Signal};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use task::Back;
use task::Job;
use uuid::Uuid;

/// Scheduler server struct.
pub struct Scheduler {
    /// The scheduler id.
    pub id: Uuid,
    /// The scheduler address.
    pub address: String,
    /// The scheduler port.
    pub port: u16,
    /// The scheduler job. job is a task list.
    pub job: Vec<Job>,
    /// Job execute result.
    pub back: Vec<Back>,
    /// Worker list.
    pub worker: Vec<Uuid>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            address: String::from("http://localhost"),
            port: 27021,
            job: Vec::new(),
            back: Vec::new(),
            worker: Vec::new(),
        }
    }
}

impl Scheduler {
    /// New scheduler.
    pub fn new(id: Uuid, address: String, port: u16) -> Self {
        Self {
            id,
            address,
            port,
            job: Vec::new(),
            back: Vec::new(),
            worker: Vec::new(),
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
                            self.process(&mut stream);
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

    /// Process stream.
    fn process(&mut self, stream: &mut TcpStream) {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        let signal = String::from_utf8_lossy(&buf[..]);
        let signal = signal.trim_matches(char::from(0));
        let signal: Signal = serde_json::from_str(signal).unwrap();

        // Worker register
        if signal.category == Category::Register {
            // save worker id
            self.worker.push(signal.id.unwrap());

            // send register back to worker
            let signal = Signal {
                category: Category::Register,
                id: Some(self.id),
                job: None,
                back: None,
                effect: true,
            };

            // send Register signal to worker
            self.send(stream, signal)
        }

        // Client submit task
        if signal.category == Category::Submit {
            // push task to queue
            self.job.push(signal.job.unwrap());

            // send submit back to client
            let signal = Signal {
                category: Category::Submit,
                id: signal.id,
                job: None,
                back: None,
                effect: true,
            };
            self.send(stream, signal)
        }

        // Worker get task
        if signal.category == Category::Get {
            // search self.result by id
            let id = signal.id.unwrap();
            let back = self.back.iter().find(|back| back.id == id);

            let signal = if let Some(back) = back {
                Signal {
                    category: Category::GetBack,
                    id: signal.id,
                    job: None,
                    back: Some(back.clone()),
                    effect: true,
                }
            } else {
                Signal {
                    category: Category::GetBack,
                    id: signal.id,
                    job: None,
                    back: None,
                    effect: true,
                }
            };

            // send result to client
            self.send(stream, signal)
        }

        // Worker get task to run.
        if signal.category == Category::Request {
            // new Send signal
            let job = self.job.pop();

            let signal = match job {
                Some(job) => Signal {
                    category: Category::RegisterBack,
                    id: signal.id,
                    job: Some(job),
                    back: None,
                    effect: true,
                },
                None => Signal {
                    category: Category::RegisterBack,
                    id: signal.id,
                    job: None,
                    back: None,
                    effect: false,
                },
            };

            // send to worker
            self.send(stream, signal)
        }

        // Client cancel task.
        if signal.category == Category::Cancel {
            // search _job by id
            let id = signal.id.unwrap();
            let job = self.job.iter().find(|job| job.id == id);

            let effect = if let Some(_job) = job {
                // remove _job from _job list
                self.job.retain(|job| job.id != id);
                true
            } else {
                false
            };

            // new Cancel signal
            let signal = Signal {
                category: Category::Cancel,
                id: signal.id,
                job: None,
                back: None,
                effect,
            };

            // send to client
            self.send(stream, signal)
        }

        // Heartbeat.
        if signal.category == Category::Heartbeat {
            // new Heartbeat signal
            let signal = Signal {
                category: Category::HeartbeatBack,
                id: signal.id,
                job: None,
                back: None,
                effect: true,
            };

            // send to client
            self.send(stream, signal)
        }
    }

    /// Send signal to stream.
    fn send(&self, stream: &mut TcpStream, signal: Signal) {
        let json = serde_json::to_string(&signal).unwrap();
        let json = json.as_bytes();
        stream.write_all(json).unwrap();
    }
}
