use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize};
use serde::ser::{SerializeStruct, Serializer};
use chrono::{Local};

pub type SecType = u64;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum TaskStatus {
    StartedAt(SecType),
    Idle,
}

/// Tasks have names and elapsed time. The last start time
/// is used to calculate the time spent on the task.
/// The time spent on the task is stored as logged time.
/// Crearing the task acts just like starting the timer
/// on an existing task.
#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    /// timestamp for when the activity is created
    added_at: String,
    /// timespamp for when the activitiy was last activated.
    /// This can be either TaskStatus::Idle when the task is stopped
    /// or TaskStatus::StartedAt when it's running.
    last_start_time: TaskStatus,
    /// when the activity is stopped all the the duration
    /// between the last start and the stopping time
    /// is added here.
    logged_time: SecType,
    name: String,
}

impl From<&str> for Task {
    fn from(name: &str) -> Self {
        Task::new(name)
    }
}

impl TaskStatus {
    fn as_sec(&self) -> SecType {
        match self {
            TaskStatus::StartedAt(time0) => elapsed_since(*time0),
            TaskStatus::Idle => 0,
        }
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let (hours, mins) = secs_to_time(self.logged_time);
                let total_time = format!("{}h:{:02}m", hours, mins);
                let mut state = serializer.serialize_struct("Task", 1)?;
                state.serialize_field("added_at", &self.added_at)?;
                state.serialize_field("last_start_time", &self.last_start_time)?;
                state.serialize_field("logged_time", &self.logged_time)?;
                state.serialize_field("name", &self.name)?;
                state.serialize_field("logged_time_pretty", &total_time)?;
                state.end()
    }
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self {
            added_at: format!("{}", Local::now()),
            last_start_time: TaskStatus::StartedAt(systime()),
            logged_time: 0,
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn start(&mut self) {
        self.logged_time += self.last_start_time.as_sec();
        self.last_start_time = TaskStatus::StartedAt(systime());
    }

    pub fn stop(&mut self) {
        self.logged_time += self.last_start_time.as_sec();
        self.last_start_time = TaskStatus::Idle;
    }

    pub fn elapsed_time(&self) -> SecType {
        self.logged_time + self.last_start_time.as_sec()
    }

    pub fn time_text(&self) -> String {
        let (hours, mins) = secs_to_time(self.elapsed_time());
        format!("{:>45}: {}h:{:02}m", self.name, hours, mins)
    }

    pub fn is_active(&self) -> bool {
        self.last_start_time != TaskStatus::Idle
    }
}

pub fn secs_to_time(secs: SecType) -> (u8, u8) {
    let hours = secs / (60 * 60);
    let minutes = (secs % (60 * 60)) / 60;
    (hours as u8, minutes as u8)
}

pub fn systime() -> SecType {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn elapsed_secs(t0: SecType, t1: SecType) -> SecType {
    Duration::from_secs(t1 - t0).as_secs()
}

fn elapsed_since(t0: SecType) -> SecType {
    elapsed_secs(t0, systime())
}




#[cfg(test)]
mod test {
    use super::*;
    use std::thread::sleep;
    use serde_json;

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

    #[test]
    fn human_readable_seconds() {
        let secs: SecType = 1;
        assert_eq!(secs_to_time(secs), (0, 0));
        let secs: SecType = 60;
        assert_eq!(secs_to_time(secs), (0, 1));
        let secs: SecType = 61;
        assert_eq!(secs_to_time(secs), (0, 1));
        let secs: SecType = 60 * 10;
        assert_eq!(secs_to_time(secs), (0, 10));
        let secs: SecType = 60 * 60;
        assert_eq!(secs_to_time(secs), (1, 0));
        let secs: SecType = 60 * 60 * 3;
        assert_eq!(secs_to_time(secs), (3, 0));
        let secs: SecType = 60 * 60 * 3 + 60;
        assert_eq!(secs_to_time(secs), (3, 1));
    }

    #[test]
    fn custom_task_serializer() {
        let task = Task::new("task");
        assert!(serde_json::to_string(&task).unwrap().contains("added_at"));
        assert!(serde_json::to_string(&task).unwrap().contains("last_start_time"));
        assert!(serde_json::to_string(&task).unwrap().contains("logged_time"));
        assert!(serde_json::to_string(&task).unwrap().contains("name"));
        assert!(serde_json::to_string(&task).unwrap().contains("logged_time_pretty"));
    }
}
