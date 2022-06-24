use std::time::{Instant, Duration};

enum Status {
    running(Instant),
    idle,
}

struct Task {
    last_start_time: Status,
    logged_time: Duration,
}

impl Status {
    fn to_duration(&self) -> Duration {
        match self {
            Self::running(start_time) => start_time.elapsed(),
            Self::idle => Duration::new(0, 0),
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
        self.logged_time += self.last_start_time.to_duration();
        self.last_start_time = Status::running(Instant::now());
    }

    fn stop(&mut self) {
        self.logged_time += self.last_start_time.to_duration();
        self.last_start_time = Status::idle;
    }

    fn elapsed_time(&self) -> Duration {
        self.logged_time + self.last_start_time.to_duration()
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
    println!("task1: {:?}", task1.elapsed_time());
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?}", task1.elapsed_time());
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?}", task1.elapsed_time());
    task1.stop();
    std::thread::sleep(Duration::new(1, 0));
    println!("task1: {:?}", task1.elapsed_time());


}
