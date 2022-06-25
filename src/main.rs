use std::time::{Instant, Duration};
#[derive(Clone, Copy)]
enum TaskStatus {
    running(Instant),
    idle,
}

struct Task {
    last_start_time: TaskStatus,
    logged_time: Duration,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl From<TaskStatus> for Duration {
    fn from(item: TaskStatus) -> Self {
        match item {
            TaskStatus::running(time0) => time0.elapsed(),
            TaskStatus::idle => Duration::new(0, 0),
        }
    }
}

impl Task {
    fn new() -> Self {
        Self {
            last_start_time: TaskStatus::running(Instant::now()),
            logged_time: Duration::new(0, 0),
        }
    }

    fn start(&mut self) {
        self.logged_time += self.last_start_time.into();
        self.last_start_time = TaskStatus::running(Instant::now());
    }

    fn stop(&mut self) {
        self.logged_time += self.last_start_time.into();
        self.last_start_time = TaskStatus::idle;
    }

    fn elapsed_time(&self) -> Duration {
        self.logged_time + self.last_start_time.into()
    }
}

fn main() {
    let t0 = Instant::now();
    std::thread::sleep(Duration::new(1, 0));
    println!("{:#?}", t0.elapsed());
    let k = t0.elapsed() + Duration::from_secs(1);
    println!("{:?}", k);

    let mut task1 = Task::new();
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?} - expected: 1", task1.elapsed_time());
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?} - expected: 2", task1.elapsed_time());
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?} - expected: 3", task1.elapsed_time());
    task1.stop();
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?} - stopped", task1.elapsed_time());
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?} - stopped", task1.elapsed_time());
    task1.start();
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?} - expected: 4", task1.elapsed_time());
    task1.start();
    println!("task1: {:?} - expected: 4", task1.elapsed_time());
    task1.start();
    println!("task1: {:?} - expected: 4", task1.elapsed_time());
    task1.start();
    println!("task1: {:?} - expected: 4", task1.elapsed_time());
    task1.start();
    println!("task1: {:?} - expected: 4", task1.elapsed_time());


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
