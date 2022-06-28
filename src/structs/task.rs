use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize};
use serde::ser::{SerializeStruct};
use chrono::{Local};

pub type SecType = u64;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum Status {
    ActiveSince(SecType),
    Idle,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Activity {
    /// timestamp for when the activity is created
    added_at: String,
    /// timespamp for when the activitiy was last activated.
    /// This can be either TaskStatus::Idle when the task is stopped
    /// or TaskStatus::StartedAt when it's running.
    last_start_time: Status,
    /// when the activity is stopped all the the duration
    /// between the last start and the stopping time
    /// is added here.
    logged_secs: SecType,
    name: String,
}

impl Status {
    /// calculates elapsed time for queries.
    /// If the task has been idle, the elapsed time is 0.
    /// Active tasks elapsed time is the diff from
    /// since the task is active to the queries system time.
    fn as_sec(&self) -> SecType {
        match self {
            Status::ActiveSince(time0) => elapsed_since(*time0),
            Status::Idle => 0,
        }
    }
}

impl Serialize for Activity {
    /// The custom serialiser adds fields that are not worth storing.
    /// Those fields are calculated on demand from stored
    /// values, like pretty prints.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let (hours, mins) = secs_to_time(self.logged_secs);
                let total_time = format!("{}h:{:02}m", hours, mins);
                // This value seems to be unused in serde.
                let number_of_fields = 255;
                let mut state = serializer.serialize_struct("Task", number_of_fields)?;
                state.serialize_field("added_at", &self.added_at)?;
                state.serialize_field("last_start_time", &self.last_start_time)?;
                state.serialize_field("logged_secs", &self.logged_secs)?;
                state.serialize_field("name", &self.name)?;
                state.serialize_field("logged_time_pretty", &total_time)?;
                state.end()
    }
}

impl Activity {
    pub fn new(name: &str) -> Self {
        Self {
            added_at: format!("{}", Local::now()),
            last_start_time: Status::ActiveSince(systime()),
            logged_secs: 0,
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn start(&mut self) {
        self.logged_secs += self.last_start_time.as_sec();
        self.last_start_time = Status::ActiveSince(systime());
    }

    pub fn stop(&mut self) {
        self.logged_secs += self.last_start_time.as_sec();
        self.last_start_time = Status::Idle;
    }

    pub fn elapsed_time(&self) -> SecType {
        self.logged_secs + self.last_start_time.as_sec()
    }

    pub fn time_text(&self) -> String {
        let (hours, mins) = secs_to_time(self.elapsed_time());
        format!("{:>45}: {}h:{:02}m", self.name, hours, mins)
    }

    pub fn is_active(&self) -> bool {
        self.last_start_time != Status::Idle
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

fn elapsed_secs(t_start: SecType, t_end: SecType) -> SecType {
    Duration::from_secs(t_end - t_start).as_secs()
}

fn elapsed_since(t_start: SecType) -> SecType {
    elapsed_secs(t_start, systime())
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

        let mut task = Activity::new("asdf");
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
        let task0 = Activity::new("sdasdf");
        let task1 = Activity::new("sdasdf");
        let task2 = Activity::new("sdasdf");

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
        let task = Activity::new("taskname");
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
        let task = Activity::new("task");
        assert!(serde_json::to_string(&task).unwrap().contains("added_at"));
        assert!(serde_json::to_string(&task).unwrap().contains("last_start_time"));
        assert!(serde_json::to_string(&task).unwrap().contains("logged_time"));
        assert!(serde_json::to_string(&task).unwrap().contains("name"));
        assert!(serde_json::to_string(&task).unwrap().contains("logged_time_pretty"));
    }
}
