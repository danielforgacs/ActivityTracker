use super::task::*;
use serde::Serialize;
use chrono::{Local};
use serde::ser::{Serializer, SerializeStruct};
use std::io::prelude::*;

const DAY_LENGTH_SECS: u64 = 7 * 60 * 60 + 30 * 60;

/// The task manager is the only struct one exposed.
/// It manages a vec of tasks.
/// Only one task can be active at a time.
/// Running tasks are exclusive, starting a task will stop all other tasks.
#[derive(Debug, PartialEq, Clone)]
pub struct TaskManager {
    path: std::path::PathBuf,
    // tasks: Vec<Activity>,
    /// pretty system time timestamp for when the taskmanager started
    start_time_pretty: String,
    start_time: SecType,
}

impl TaskManager {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self {
            // tasks: Vec::new(),
            start_time_pretty: format!("{}", Local::now()),
            start_time: sys_now_secs(),
            path,
        }
    }

    fn read(&self) -> Vec<Activity> {
        let mut file_handle = std::fs::File::open(&self.path).unwrap();
        let mut buf = String::new();
        file_handle.read_to_string(&mut buf).unwrap();
        let data: Vec<Activity> = serde_json::from_str(&buf.as_str()).unwrap();
        data
    }

    fn write(&self, data: Vec<Activity>) {
        let data_serialised = serde_json::to_string_pretty(&data).unwrap();
        dbg!(&data);
        let mut file_handle = std::fs::File::create(&self.path).unwrap();
        file_handle.write_all(data_serialised.as_bytes()).expect("CAN NOT WRITE ALL.");
    }

    pub fn start(&mut self, name: &str) {
        let mut data = self.read();
        if !data.iter().map(|f| f.name()).collect::<Vec<String>>().contains(&name.to_string()) {
            data.push(
                Activity::new(name)
            );
        }
        for task in data.iter_mut() {
            if task.name() == name {
                task.start();
            } else {
                task.stop();
            }
        }
        self.write(data);
    }

    pub fn stop(&mut self) {
        let mut data = self.read();
        data
            .iter_mut()
            .for_each(|t| t.stop());
        self.write(data);
    }

    fn task_names(&self) -> Vec<String> {
        self.read().iter().map(|f| f.name()).collect::<Vec<String>>()
    }

    fn task_exists(&self, name: &str) -> bool {
        self.task_names().contains(&name.to_string())
    }

    pub fn times(&self) -> String {
        let mut result = format!("start time:         {}", self.start_time_pretty.to_owned());
        let (hh, mm) = &secs_to_hours_minutes(elapsed_since(self.start_time));
        result.push_str(&format!("\nelapsed day:        {:02}h:{:02}m", hh, mm));
        let total_activity_time: SecType = self.read()
            .iter()
            .map(|t| t.secs_since_creation())
            .sum();
        let (hours, minutes) = secs_to_hours_minutes(total_activity_time);
        result.push_str(
            &format!("\ntotal acivity time: {:02}h:{:02}m", hours, minutes)
        );
        result.push('\n');
        result.push_str(
            &self.read()
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
        self.read()
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
        let time_diff = elapsed_since(self.start_time).saturating_sub(self.total_activity_time());
        let (tdelta_hh, tdelta_mm) = secs_to_hours_minutes(time_diff);
        let time_diff_pretty = &format!("{:02}h:{:02}m", tdelta_hh, tdelta_mm);

        state.serialize_field("tasks", &self.read())?;
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
        let path = std::path::Path::new("test_creating_task_manager.json").to_path_buf();
        std::fs::File::create(&path).unwrap().write_all(b"[]").unwrap();
        let tm = TaskManager::new(path);
        assert_eq!(tm.read(), Vec::new());
    }

    #[test]
    fn add_task() {
        let path = std::path::Path::new("test_add_task.json").to_path_buf();
        std::fs::File::create(&path).unwrap().write_all(b"[]").unwrap();
        let mut tm = TaskManager::new(path);
        let task_name = "task";
        tm.start(task_name);
        assert_eq!(tm.read().len(), 1);
        assert_eq!(tm.read()[0].name(), task_name);
        assert_eq!(tm.read()[0].secs_since_creation(), 0);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        tm.stop();
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        tm.start(task_name);
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.read()[0].secs_since_creation(), 3);
    }

    #[test]
    fn multiple_tasks() {
        let task_1 = "alpha";
        let task_2 = "beta";
        let path = std::path::Path::new("test_multiple_tasks.json").to_path_buf();
        std::fs::File::create(&path).unwrap().write_all(b"[]").unwrap();
        let mut tm = TaskManager::new(path);
        tm.start(task_1);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        tm.start(task_2);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[1].secs_since_creation(), 1);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[1].secs_since_creation(), 2);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[1].secs_since_creation(), 3);
        tm.start(task_2);
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[1].secs_since_creation(), 3);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[1].secs_since_creation(), 4);
        tm.start(task_1);
        assert_eq!(tm.read()[0].secs_since_creation(), 1);
        assert_eq!(tm.read()[1].secs_since_creation(), 4);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 2);
        assert_eq!(tm.read()[1].secs_since_creation(), 4);
        pause();
        assert_eq!(tm.read()[0].secs_since_creation(), 3);
        assert_eq!(tm.read()[1].secs_since_creation(), 4);
    }

    fn pause() {
        let pause_secs = 1;
        std::thread::sleep(std::time::Duration::from_secs(pause_secs));
    }

    #[test]
    fn no_duplicate_task_names() {
        let path = std::path::Path::new("test_no_duplicate_task_names.json").to_path_buf();
        std::fs::File::create(&path).unwrap().write_all(b"[]").unwrap();
        let mut tm = TaskManager::new(path);
        tm.start("a");
        tm.start("a");
        tm.start("a");
        tm.start("a");
        assert_eq!(tm.task_names(), vec!["a"]);
    }

    #[test]
    fn taskmanager_json_has_all_fields() {
        let path = std::path::Path::new("test_taskmanager_json_has_all_fields.json").to_path_buf();
        std::fs::File::create(&path).unwrap().write_all(b"[]").unwrap();
        let mut tm = TaskManager::new(path);
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
