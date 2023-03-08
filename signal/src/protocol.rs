//! Protocol for scheduler.
use serde::{Deserialize, Serialize};
use std::io::Read;
use task::Back;
use task::Job;
use uuid::Uuid;

/// signal type.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Category {
    /// Worker register to scheduler.
    Register,
    RegisterBack,

    /// Client Submit Task signal.
    Submit,
    SubmitBack,

    /// Client get task.
    Get,
    GetBack,

    /// Worker Request Task signal.
    // request do not need.
    Request,
    RequestBack,

    /// Client Cancel Task signal.
    Cancel,
    CancelBack,
    /// Heartbeat signal.
    Heartbeat,
    HeartbeatBack,
}

/// Signal struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signal {
    /// The signal type.
    pub category: Category,
    /// id. Job id or Worker id.
    pub id: Option<Uuid>,
    /// The signal data.
    pub job: Option<Job>,
    /// Signal result. true is success, false is fail.
    pub back: Option<Back>,
    /// The signal result. if Success is true, else is false.
    pub effect: bool,
}

impl Signal {
    /// Create signal from TcpStream.
    pub fn from_stream(stream: &mut std::net::TcpStream) -> Option<Self> {
        let mut buffer = Vec::new();

        if stream.read_to_end(&mut buffer).is_err() {
            return None;
        }

        let signal = String::from_utf8_lossy(&buffer[..]);
        let signal = signal.trim_matches(char::from(0));
        let result = serde_json::from_str(signal);

        match result {
            Ok(signal) => Some(signal),
            Err(_) => None,
        }
    }
}

/// tests
#[cfg(test)]
mod tests {
    use super::*;
    use task::Task;

    #[test]
    fn test_signal() {
        let code = "def test():
            print(\"test\")"
            .to_string()
            .into_bytes();

        let signal = Signal {
            category: Category::Submit,
            id: Some(Uuid::new_v4()),
            job: Some(Job::new(
                Uuid::new_v4(),
                vec![Task::new(Uuid::new_v4(), code, "test".to_string())],
                vec![],
            )),
            back: None,
            effect: true,
        };
        let json = serde_json::to_string(&signal).unwrap();
        println!("json: {json}");
        let signal: Signal = serde_json::from_str(&json).unwrap();
        println!("signal: {signal:?}");
        let code = String::from_utf8(signal.job.unwrap().tasks[0].code.clone()).unwrap();
        println!("code: {code}");
        assert_eq!(
            code,
            "def test():
            print(\"test\")"
        );
    }
}
