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

impl Task {
    pub fn new() -> Self {
        Self {
            last_start_time: TaskStatus::Running(Instant::now()),
            logged_time: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.logged_time += self.last_start_time.into();
        self.last_start_time = TaskStatus::Running(Instant::now());
    }

    pub fn stop(&mut self) {
        self.logged_time += self.last_start_time.into();
        self.last_start_time = TaskStatus::Idle;
    }

    pub fn elapsed_time(&self) -> Duration {
        self.logged_time + self.last_start_time.into()
    }
}
