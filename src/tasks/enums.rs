use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    InProgress(DateTime<Local>),
    Complete(DateTime<Local>),
}

#[derive(Debug, Clone)]
pub enum ExecutionOrder {
    Series,
    Parallel,
}
