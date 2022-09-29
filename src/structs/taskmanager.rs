use super::task::*;
use serde::Serialize;
use chrono::{Local};
use serde::ser::{Serializer, SerializeStruct};

const DAY_LENGTH_SECS: u64 = 7 * 60 * 60 + 30 * 60;

/// The task manager is the only struct one exposed.
/// It manages a vec of tasks.
/// Only one task can be active at a time.
/// Running tasks are exclusive, starting a task will stop all other tasks.
#[derive(Debug, PartialEq, Clone)]
pub struct TaskManager {
    tasks: Vec<Activity>,
    /// pretty system time timestamp for when the taskmanager started
    start_time_pretty: String,
    start_time: SecType,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            start_time_pretty: format!("{}", Local::now()),
            start_time: sys_now_secs(),
        }
    }

    pub fn start(&mut self, name: &str) {
        if !self.task_exists(name) {
            self.tasks.push(
                Activity::new(name)
            );
        }
        for task in self.tasks.iter_mut() {
            if task.name() == name {
                task.start();
            } else {
                task.stop();
            }
        }
    }

    pub fn stop(&mut self) {
        self.tasks
            .iter_mut()
            .for_each(|t| t.stop());
    }

    fn task_names(&self) -> Vec<String> {
        self.tasks.iter().map(|f| f.name()).collect::<Vec<String>>()
    }

    fn task_exists(&self, name: &str) -> bool {
        self.task_names().contains(&name.to_string())
    }

    pub fn times(&self) -> String {
        let mut result = format!("start time:         {}", self.start_time_pretty.to_owned());
        let (hh, mm) = &secs_to_hours_minutes(elapsed_since(self.start_time));
        result.push_str(&format!("\nelapsed day:        {:02}h:{:02}m", hh, mm));
        let total_activity_time: SecType = self.tasks
            .iter()
            .map(|t| t.secs_since_creation())
            .sum();
        let (hours, minutes) = secs_to_hours_minutes(total_activity_time);
        result.push_str(
            &format!("\ntotal acivity time: {:02}h:{:02}m", hours, minutes)
        );
        result.push('\n');
        result.push_str(
            &self.tasks
            .iter()
            .map(|t| {
                if t.is_active() {
                    format!("> {}", t.time_text())
                } else {
                    format!("  {}", t.time_text())
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
        );
        result.push('\n');
        result
    }

    fn total_activity_time(&self) -> SecType {
        self.tasks
            .iter()
            .map(|t| t.secs_since_creation())
            .sum()
    }
}

impl Serialize for TaskManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (hours, mins) = secs_to_hours_minutes(self.total_activity_time());
        let total_time = format!("{:02}h:{:02}m", hours, mins);
        let mut state = serializer.serialize_struct("Taskmanager", 3)?;
        let (hh, mm) = &secs_to_hours_minutes(elapsed_since(self.start_time));
        let elapsed_day = &format!("{:02}h:{:02}m", hh, mm);
        let time_diff = elapsed_since(self.start_time) - self.total_activity_time();
        let (tdelta_hh, tdelta_mm) = secs_to_hours_minutes(time_diff);
        let time_diff_pretty = &format!("{:02}h:{:02}m", tdelta_hh, tdelta_mm);

        state.serialize_field("tasks", &self.tasks)?;
        state.serialize_field("start_time_pretty", &self.start_time_pretty)?;
        state.serialize_field("elapsed_day", &elapsed_day)?;
        state.serialize_field("total_activity_time", &total_time)?;
        state.serialize_field("time_difference", &time_diff_pretty)?;
        state.serialize_field("start_time:", &self.start_time)?;
        state.serialize_field("display:", &self.times())?;

        let (day_len_hh, day_len_mm) = secs_to_hours_minutes(DAY_LENGTH_SECS);
        let day_length = &format!("{:02}h:{:02}m", day_len_hh, day_len_mm);
        state.serialize_field("day_length", day_length)?;

        let (time_left_hh, time_left_mm) = secs_to_hours_minutes(DAY_LENGTH_SECS - self.total_activity_time());
        let time_left = &format!("{:02}h:{:02}m", time_left_hh, time_left_mm);
        state.serialize_field("time_left", time_left)?;
        state.end()
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creating_task_manager() {
        let tm = TaskManager::new();
        assert_eq!(tm.tasks, Vec::new());
    }

    #[test]
    fn add_task() {
        let mut tm = TaskManager::new();
        let task_name = "task";
        tm.start(task_name);
        assert_eq!(tm.tasks.len(), 1);
        assert_eq!(tm.tasks[0].name(), task_name);
        assert_eq!(tm.tasks[0].secs_since_creation(), 0);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        tm.stop();
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        tm.start(task_name);
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].secs_since_creation(), 3);
    }

    #[test]
    fn multiple_tasks() {
        let task_1 = "alpha";
        let task_2 = "beta";
        let mut tm = TaskManager::new();
        tm.start(task_1);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        tm.start(task_2);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[1].secs_since_creation(), 1);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[1].secs_since_creation(), 2);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[1].secs_since_creation(), 3);
        tm.start(task_2);
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[1].secs_since_creation(), 3);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[1].secs_since_creation(), 4);
        tm.start(task_1);
        assert_eq!(tm.tasks[0].secs_since_creation(), 1);
        assert_eq!(tm.tasks[1].secs_since_creation(), 4);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 2);
        assert_eq!(tm.tasks[1].secs_since_creation(), 4);
        pause();
        assert_eq!(tm.tasks[0].secs_since_creation(), 3);
        assert_eq!(tm.tasks[1].secs_since_creation(), 4);
    }

    fn pause() {
        let pause_secs = 1;
        std::thread::sleep(std::time::Duration::from_secs(pause_secs));
    }

    #[test]
    fn no_duplicate_task_names() {
        let mut tm = TaskManager::new();
        tm.start("a");
        tm.start("a");
        tm.start("a");
        tm.start("a");
        assert_eq!(tm.task_names(), vec!["a"]);
    }

    #[test]
    fn taskmanager_json_has_all_fields() {
        let tm = TaskManager::new();
        let tm_json = serde_json::to_string(&tm).unwrap();
        assert!(tm_json.contains(&"tasks"));
        assert!(tm_json.contains(&"start_time_pretty"));
        assert!(tm_json.contains(&"elapsed_day"));
        assert!(tm_json.contains(&"total_activity_time"));
        assert!(tm_json.contains(&"time_difference"));
        assert!(tm_json.contains(&"start_time"));
        assert!(tm_json.contains(&"display"));
    }
}
