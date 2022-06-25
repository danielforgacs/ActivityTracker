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

#[cfg(test)]
mod test {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn task_timing_test() {
        let max_diff = Duration::from_millis(3);

        let mut task = Task::new();
        assert!(task.elapsed_time() < max_diff);
        let pause = Duration::from_secs(1);

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - pause);
        assert!(max_diff > task.elapsed_time() - pause);
        assert!(max_diff > task.elapsed_time() - pause);
        assert!(max_diff > task.elapsed_time() - pause);

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - (pause + pause));

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - (pause + pause + pause));
        task.stop();

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - (pause + pause + pause));

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - (pause + pause + pause));
        task.start();

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - (pause + pause + pause + pause));

        std::thread::sleep(pause);
        assert!(max_diff > task.elapsed_time() - (pause + pause + pause + pause + pause));
    }

    #[test]
    fn timing_multiple_tasks() {
        let task0 = Task::new();
        let task1 = Task::new();
        let task2 = Task::new();

        let max_diff = Duration::from_millis(3);
        let pause = Duration::from_secs(1);

        assert!(task0.elapsed_time() < max_diff);
        assert!(task1.elapsed_time() < max_diff);
        assert!(task2.elapsed_time() < max_diff);

        sleep(pause);

        assert!(task0.elapsed_time() - pause < max_diff);
        assert!(task1.elapsed_time() - pause < max_diff);
        assert!(task2.elapsed_time() - pause < max_diff);

        sleep(pause);

        assert!(task0.elapsed_time() - (pause * 2) < max_diff);
        assert!(task1.elapsed_time() - (pause * 2) < max_diff);
        assert!(task2.elapsed_time() - (pause * 2) < max_diff);
    }
}
