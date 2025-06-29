#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}
