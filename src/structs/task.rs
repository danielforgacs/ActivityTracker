use std::time::{Instant, Duration};

type SecType = u64;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskStatus {
    Running(Instant),
    Idle,
}

/// Tasks have names and elapsed time. The last start time
/// is used to calculate the time spent on the task.
/// The time spent on the task is stored as logged time.
/// Crearing the task acts just like starting the timer
/// on an existing task.
#[derive(Debug, PartialEq)]
pub struct Task {
    last_start_time: TaskStatus,
    logged_time: SecType,
    name: String,
}

impl From<TaskStatus> for Duration {
    fn from(item: TaskStatus) -> Self {
        match item {
            TaskStatus::Running(time0) => time0.elapsed(),
            TaskStatus::Idle => Duration::new(0, 0),
        }
    }
}

impl From<&str> for Task {
    fn from(name: &str) -> Self {
        Task::new(name)
    }
}

impl TaskStatus {
    fn as_sec(&self) -> SecType {
        match self {
            TaskStatus::Running(time0) => time0.elapsed().as_secs(),
            TaskStatus::Idle => 0,
        }
    }
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self {
            last_start_time: TaskStatus::Running(Instant::now()),
            logged_time: 0,
            name: name.to_string(),
        }
    }

    pub fn start(&mut self) {
        self.logged_time += self.last_start_time.as_sec();
        self.last_start_time = TaskStatus::Running(Instant::now());
    }

    pub fn stop(&mut self) {
        self.logged_time += self.last_start_time.as_sec();
        self.last_start_time = TaskStatus::Idle;
    }

    pub fn elapsed_time(&self) -> SecType {
        self.logged_time + self.last_start_time.as_sec()
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn task_timing_test() {
        let pause_secs = 1;
        let pause = Duration::from_secs(pause_secs);

        let mut task = Task::new("asdf");
        assert_eq!(task.elapsed_time(), 0);

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 1);
        assert_eq!(task.elapsed_time(), pause_secs * 1);
        assert_eq!(task.elapsed_time(), pause_secs * 1);
        assert_eq!(task.elapsed_time(), pause_secs * 1);

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 2);

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 3);

        task.stop();

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 3);

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 3);
        task.start();

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 4);

        std::thread::sleep(pause);
        assert_eq!(task.elapsed_time(), pause_secs * 5);
    }

    #[test]
    fn timing_multiple_tasks() {
        let task0 = Task::new("sdasdf");
        let task1 = Task::new("sdasdf");
        let task2 = Task::new("sdasdf");

        let pause_secs = 1;
        let pause = Duration::from_secs(pause_secs);

        assert_eq!(task0.elapsed_time(), pause_secs * 0);
        assert_eq!(task1.elapsed_time(), pause_secs * 0);
        assert_eq!(task2.elapsed_time(), pause_secs * 0);

        sleep(pause);

        assert_eq!(task0.elapsed_time(), pause_secs * 1);
        assert_eq!(task1.elapsed_time(), pause_secs * 1);
        assert_eq!(task2.elapsed_time(), pause_secs * 1);

        sleep(pause);

        assert_eq!(task0.elapsed_time(), pause_secs * 2);
        assert_eq!(task1.elapsed_time(), pause_secs * 2);
        assert_eq!(task2.elapsed_time(), pause_secs * 2);
    }

    #[test]
    fn task_from_str() {
        let task = Task::from("taskname");
        assert_eq!("taskname", task.name);
    }
}
