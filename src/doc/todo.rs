

pub struct Todo{
    tasks: Vec<Task>
}

pub struct Task{
    description: String,
    status: TaskStatus
}

pub enum TaskStatus{
    NotStarted,
    InProgress,
    Done
}