use std::time::{Instant, Duration};

struct Task {
    last_start_time: Instant,
    logged_time: Duration,
}

impl Task {
    fn new() -> Self {
        Self {
            last_start_time: Instant::now(),
            logged_time: Duration::new(0, 0),
        }
    }
}

fn main() {
    let t0 = Instant::now();
    std::thread::sleep(Duration::new(1, 0));
    println!("{:#?}", t0.elapsed());
    let k = t0.elapsed() + Duration::from_secs(1);
    println!("{:?}", k);
}
