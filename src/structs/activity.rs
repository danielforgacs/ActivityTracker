use chrono::Local;
use chrono::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub type SecType = u64;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Status {
    ActiveSince(SecType),
    Idle,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Activity {
    /// timestamp for when the activity is created
    added_at: String,
    // All timestamps when this activity got activated.
    started_at: Vec<String>,
    active_days: Vec<String>,
    /// timespamp for when the activitiy was last activated.
    /// This can be either TaskStatus::Idle when the task is stopped
    /// or TaskStatus::StartedAt when it's running.
    status: Status,
    /// when the activity is stopped all the the duration
    /// between the last start and the stopping time
    /// is added here.
    logged_secs: SecType,
    name: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivitySerial {
    added_at: String,
    started_at: Vec<String>,
    active_days: Vec<String>,
    status: Status,
    logged_secs: SecType,
    name: String,
    logged_pretty: String,
}

impl From<Activity> for ActivitySerial {
    fn from(activity: Activity) -> Self {
        let (hours, mins) = secs_to_hours_minutes(activity.secs_since_creation());
        let logged_pretty = format!("{:02}h:{:02}m", hours, mins);
        Self {
            added_at: activity.added_at,
            started_at: activity.started_at,
            active_days: activity.active_days,
            status: activity.status,
            logged_secs: activity.logged_secs,
            name: activity.name,
            logged_pretty,
        }
    }
}

impl Status {
    /// calculates elapsed time for queries.
    /// If the task has been idle, the elapsed time is 0.
    /// Active tasks elapsed time is the diff from
    /// since the task is active to the queries system time.
    fn as_elapsed_secs(&self) -> SecType {
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
        S: serde::Serializer,
    {
        // This value seems to be unused in serde.
        let number_of_fields = 255;
        let mut state = serializer.serialize_struct("Task", number_of_fields)?;
        state.serialize_field("added_at", &self.added_at)?;
        state.serialize_field("started_at", &self.started_at)?;
        state.serialize_field("active_days", &self.active_days)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("logged_secs", &self.logged_secs)?;
        state.serialize_field("name", &self.name)?;
        let (hours, mins) = secs_to_hours_minutes(self.secs_since_creation());
        let all_time_pretty = format!("{}h:{:02}m", hours, mins);
        state.serialize_field("all_time_pretty", &all_time_pretty)?;
        state.end()
    }
}

impl Activity {
    pub fn new(name: &str) -> Self {
        Self {
            added_at: format!("{}", Local::now()),
            started_at: vec![format!("{}", Local::now())],
            // Days on which the activity was active
            active_days: vec![Utc::now().date_naive().to_string()],
            status: Status::ActiveSince(sys_now_secs()),
            logged_secs: 0,
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn start(&mut self) {
        // on starting an activity the latest elapsed secs
        // are added to the logged time. This needs to be
        // done becouse the new stored status active since value.
        // If this is not added and an active task is
        // activated again the start time stamp will change,
        // but the logged time remains the same!
        self.logged_secs += self.status.as_elapsed_secs();
        self.status = Status::ActiveSince(sys_now_secs());
        let date = Utc::now().date_naive().to_string();
        if !self.active_days.contains(&date) {
            self.active_days.push(date);
        }
        self.started_at.push(format!("{}", Local::now()));
    }

    pub fn stop(&mut self) {
        self.logged_secs += self.status.as_elapsed_secs();
        self.status = Status::Idle;
    }

    /// all logged secs plus tha latest active time secs if any.
    pub fn secs_since_creation(&self) -> SecType {
        self.logged_secs + self.status.as_elapsed_secs()
    }

    pub fn time_text(&self) -> String {
        let (hours, mins) = secs_to_hours_minutes(self.secs_since_creation());
        format!("{:>45}: {}h:{:02}m", self.name, hours, mins)
    }

    pub fn is_active(&self) -> bool {
        self.status != Status::Idle
    }
}

pub fn secs_to_hours_minutes(secs: SecType) -> (u8, u8) {
    let hours = secs / (60 * 60);
    let minutes = (secs % (60 * 60)) / 60;
    (hours as u8, minutes as u8)
}

pub fn sys_now_secs() -> SecType {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn elapsed_secs(t_start: SecType, t_end: SecType) -> SecType {
    Duration::from_secs(t_end - t_start).as_secs()
}

pub fn elapsed_since(t_start: SecType) -> SecType {
    elapsed_secs(t_start, sys_now_secs())
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use std::thread::sleep;

    #[test]
    fn task_timing_test() {
        let pause_secs = 1;
        let pause = Duration::from_secs(pause_secs);

        let mut task = Activity::new("asdf");
        assert_eq!(task.secs_since_creation(), 0);

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 1);
        assert_eq!(task.secs_since_creation(), pause_secs * 1);
        assert_eq!(task.secs_since_creation(), pause_secs * 1);
        assert_eq!(task.secs_since_creation(), pause_secs * 1);

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 2);

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 3);

        task.stop();

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 3);

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 3);
        task.start();

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 4);

        std::thread::sleep(pause);
        assert_eq!(task.secs_since_creation(), pause_secs * 5);
    }

    #[test]
    fn timing_multiple_tasks() {
        let task0 = Activity::new("sdasdf");
        let task1 = Activity::new("sdasdf");
        let task2 = Activity::new("sdasdf");

        let pause_secs = 1;
        let pause = Duration::from_secs(pause_secs);

        assert_eq!(task0.secs_since_creation(), pause_secs * 0);
        assert_eq!(task1.secs_since_creation(), pause_secs * 0);
        assert_eq!(task2.secs_since_creation(), pause_secs * 0);

        sleep(pause);

        assert_eq!(task0.secs_since_creation(), pause_secs * 1);
        assert_eq!(task1.secs_since_creation(), pause_secs * 1);
        assert_eq!(task2.secs_since_creation(), pause_secs * 1);

        sleep(pause);

        assert_eq!(task0.secs_since_creation(), pause_secs * 2);
        assert_eq!(task1.secs_since_creation(), pause_secs * 2);
        assert_eq!(task2.secs_since_creation(), pause_secs * 2);
    }

    #[test]
    fn task_from_str() {
        let task = Activity::new("taskname");
        assert_eq!("taskname", task.name);
    }

    #[test]
    fn human_readable_seconds() {
        let secs: SecType = 1;
        assert_eq!(secs_to_hours_minutes(secs), (0, 0));
        let secs: SecType = 60;
        assert_eq!(secs_to_hours_minutes(secs), (0, 1));
        let secs: SecType = 61;
        assert_eq!(secs_to_hours_minutes(secs), (0, 1));
        let secs: SecType = 60 * 10;
        assert_eq!(secs_to_hours_minutes(secs), (0, 10));
        let secs: SecType = 60 * 60;
        assert_eq!(secs_to_hours_minutes(secs), (1, 0));
        let secs: SecType = 60 * 60 * 3;
        assert_eq!(secs_to_hours_minutes(secs), (3, 0));
        let secs: SecType = 60 * 60 * 3 + 60;
        assert_eq!(secs_to_hours_minutes(secs), (3, 1));
    }

    #[test]
    fn custom_task_serializer() {
        let task = Activity::new("task");
        assert!(serde_json::to_string(&task).unwrap().contains("added_at"));
        assert!(serde_json::to_string(&task).unwrap().contains("status"));
        assert!(serde_json::to_string(&task)
            .unwrap()
            .contains("logged_secs"));
        assert!(serde_json::to_string(&task).unwrap().contains("name"));
        assert!(serde_json::to_string(&task)
            .unwrap()
            .contains("all_time_pretty"));
    }

    #[test]
    fn test_activityserial_from_activity() {
        let mut activity = Activity::new("test-act01");
        activity.stop();
        activity.logged_secs = 120;
        let activity_serial = ActivitySerial::from(activity);
        assert_eq!(activity_serial.logged_pretty, "00h:02m");
    }

    #[test]
    fn test_activityserial_from_activity_02() {
        let mut activity = Activity::new("test-act01");
        activity.stop();
        activity.logged_secs = 60 * 60;
        let activity_serial: ActivitySerial = activity.into();
        assert_eq!(activity_serial.logged_pretty, "01h:00m");
    }

    #[test]
    fn test_activityserial_from_activity_03() {
        let mut activity = Activity::new("test-act01");
        activity.stop();
        activity.logged_secs = (60 * 60 * 2) + (60 * 5) + 30;
        let activity_serial = ActivitySerial::from(activity);
        assert_eq!(activity_serial.logged_pretty, "02h:05m");
    }
}
