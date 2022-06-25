use std::time::{Instant, Duration};
#[derive(Clone, Copy)]
pub enum TaskStatus {
    Running(Instant),
    Idle,
}

pub struct Task {
    pub last_start_time: TaskStatus,
    pub logged_time: Duration,
}

impl From<TaskStatus> for Duration {
    fn from(item: TaskStatus) -> Self {
        match item {
            TaskStatus::Running(time0) => time0.elapsed(),
            TaskStatus::Idle => Duration::new(0, 0),
        }
    }
}
