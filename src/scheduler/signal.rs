//! Protocol for scheduler.
use crate::task::Task;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// signal type.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SignalType {
    /// Submit Task signal.
    Submit,
    /// Request Task signal.
    Request,
    /// Cancel Task signal.
    Cancel,
    /// Heartbeat signal.
    Heartbeat,
}

/// Signal struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signal {
    /// The signal type.
    pub _type: SignalType,
    /// Task id.
    pub id: Option<Uuid>,
    /// The signal data.
    pub task: Option<Vec<Task>>,
}

/// tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_signal() {
        let code = "def test():
            print(\"test\")"
            .to_string()
            .into_bytes();

        let signal = Signal {
            _type: SignalType::Submit,
            id: Some(Uuid::new_v4()),
            task: Some(vec![Task::new(
                "test".to_string(),
                code,
                "test".to_string(),
            )]),
        };
        let json = serde_json::to_string(&signal).unwrap();
        println!("json: {json}");
        let signal: Signal = serde_json::from_str(&json).unwrap();
        println!("signal: {signal:?}");
    }
}
