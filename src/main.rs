use std::time::{Instant, Duration};
#[derive(Clone, Copy)]
enum Status {
    running(Instant),
    idle,
}

struct Task {
    last_start_time: Status,
    logged_time: Duration,
}

impl From<Status> for Duration {
    fn from(item: Status) -> Self {
        match item {
            Status::running(time0) => time0.elapsed(),
            Status::idle => Duration::new(0, 0),
        }
    }
}

impl Task {
    fn new() -> Self {
        Self {
            last_start_time: Status::running(Instant::now()),
            logged_time: Duration::new(0, 0),
        }
    }

    fn start(&mut self) {
        self.logged_time += self.last_start_time.into();
        self.last_start_time = Status::running(Instant::now());
    }

    fn stop(&mut self) {
        self.logged_time += self.last_start_time.into();
        self.last_start_time = Status::idle;
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

    #[test]
    fn task_timing_test() {
        let max_diff = Duration::from_millis(10);
        let mut task = Task::new();
        assert!(task.elapsed_time() < Duration::from_millis(10));
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
}
