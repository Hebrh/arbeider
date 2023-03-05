//! Protocol for scheduler.
use serde::{Deserialize, Serialize};
use task::Back;
use task::Job;
use uuid::Uuid;

/// signal type.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Category {
    /// Client Submit Task signal.
    Submit,
    /// Client get task result signal.
    Get,
    /// Worker Request Task signal.
    Request,
    /// Scheduler send task to worker.
    Send,
    /// Worker send task result to scheduler.
    Result,
    /// Client Cancel Task signal.
    Cancel,
    /// Heartbeat signal.
    Heartbeat,
}

/// Signal struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signal {
    /// The signal type.
    pub category: Category,
    /// Task id.
    pub id: Option<Uuid>,
    /// The signal data.
    pub job: Option<Job>,
    /// Signal result. true is success, false is fail.
    pub back: Option<Back>,
    /// The signal result. if Success is true, else is false.
    pub effect: bool,
}

/// tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
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
