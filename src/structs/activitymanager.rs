use crate::prelude::*;

/// The task manager is the only struct one exposed.
/// It manages a vec of tasks.
/// Only one task can be active at a time.
/// Running tasks are exclusive, starting a task will stop all other tasks.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TaskManager {
    path: std::path::PathBuf,
    /// pretty system time timestamp for when the taskmanager started
    start_time_pretty: String,
    start_time: SecType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityManagerSerial {
    date: String,
    activities: Vec<Activity>,
    start_time_pretty: String,

    tasks: Vec<ActivitySerial>,
    elapsed_day: String,
    total_activity_time: String,
    time_difference: String,
    start_time: SecType,
    display: String,
    day_length: String,
    time_left: String,
}

impl TaskManager {
    pub fn new(path: std::path::PathBuf) -> Self {
        let now = Utc::now();
        Self {
            start_time_pretty: format!("{} {}", now.date_naive(), now.time().format("%H:%M:%S")),
            start_time: sys_now_secs(),
            path,
        }
    }

    pub fn start_activity(&mut self, name: &str) {
        let mut data = db_io::read(&self.path);
        if !data.iter().any(|x| x.name() == *name) {
            data.push(Activity::new(name));
        }
        for task in data.iter_mut() {
            if task.name() == name {
                task.start();
            } else {
                task.stop();
            }
        }
        db_io::write(&self.path, data);
    }

    pub fn stop(&mut self) {
        let mut data = db_io::read(&self.path);
        data.iter_mut().for_each(|t| t.stop());
        db_io::write(&self.path, data);
    }

    pub fn pretty(&self) -> String {
        let mut result = format!("start time:         {}", self.start_time_pretty.to_owned());
        let (hh, mm) = &secs_to_hours_minutes(elapsed_since(self.start_time));
        result.push_str(&format!("\nelapsed day:        {:02}h:{:02}m", hh, mm));
        let total_activity_time: SecType = db_io::read(&self.path)
            .iter()
            .map(|t| t.secs_since_creation())
            .sum();
        let (hours, minutes) = secs_to_hours_minutes(total_activity_time);
        result.push_str(&format!(
            "\ntotal acivity time: {:02}h:{:02}m",
            hours, minutes
        ));
        result.push('\n');
        result.push_str(
            &db_io::read(&self.path)
                .iter()
                .map(|t| {
                    if t.is_active() {
                        format!("> {}", t.time_text())
                    } else {
                        format!("  {}", t.time_text())
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),
        );
        result.push('\n');
        result
    }

    fn total_activity_time(&self) -> SecType {
        db_io::read(&self.path)
            .iter()
            .map(|t| t.secs_since_creation())
            .sum()
    }

    pub fn today_times(&self) -> ActivityManagerSerial {
        let (hours, mins) = secs_to_hours_minutes(self.total_activity_time());
        let total_time = format!("{:02}h:{:02}m", hours, mins);
        let (hh, mm) = &secs_to_hours_minutes(elapsed_since(self.start_time));
        let elapsed_day = format!("{:02}h:{:02}m", hh, mm);
        let time_diff = elapsed_since(self.start_time).saturating_sub(self.total_activity_time());
        let (tdelta_hh, tdelta_mm) = secs_to_hours_minutes(time_diff);
        let time_diff_pretty = format!("{:02}h:{:02}m", tdelta_hh, tdelta_mm);
        let total_activity_time = total_time;
        let time_difference = time_diff_pretty;
        let start_time = self.start_time;
        let display = self.pretty();
        let (day_len_hh, day_len_mm) = secs_to_hours_minutes(DAY_LENGTH_SECS);
        let day_length = format!("{:02}h:{:02}m", day_len_hh, day_len_mm);
        let (time_left_hh, time_left_mm) =
            secs_to_hours_minutes(DAY_LENGTH_SECS - self.total_activity_time());
        let time_left = format!("{:02}h:{:02}m", time_left_hh, time_left_mm);
        let time_left = time_left;
        let date = Utc::now().date_naive().to_string();
        let activities = db_io::read_as_serialised(&self.path)
            .into_iter()
            .filter(|x| x.get_active_dates().contains(&date))
            .collect();
        ActivityManagerSerial {
            date,
            activities: db_io::read(&self.path),
            start_time_pretty: format!("start time:         {}", self.start_time_pretty.to_owned()),
            tasks: activities,
            elapsed_day,
            total_activity_time,
            time_difference,
            start_time,
            display,
            day_length,
            time_left,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creating_task_manager() {
        let path = std::path::Path::new("test_creating_task_manager.json").to_path_buf();
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"[]")
            .unwrap();
        assert_eq!(db_io::read(&path), Vec::new());
    }

    #[test]
    fn add_task() {
        let path = std::path::Path::new("test_add_task.json").to_path_buf();
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"[]")
            .unwrap();
        let mut tm = TaskManager::new(path.clone());
        let task_name = "task";
        tm.start_activity(task_name);
        assert_eq!(db_io::read(&path).len(), 1);
        assert_eq!(db_io::read(&path)[0].name(), task_name);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 0);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        tm.stop();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        tm.start_activity(task_name);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 3);
    }

    #[test]
    fn multiple_tasks() {
        let task_1 = "alpha";
        let task_2 = "beta";
        let path = std::path::Path::new("test_multiple_tasks.json").to_path_buf();
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"[]")
            .unwrap();
        let mut tm = TaskManager::new(path.clone());
        tm.start_activity(task_1);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        tm.start_activity(task_2);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 1);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 2);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 3);
        tm.start_activity(task_2);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 3);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 4);
        tm.start_activity(task_1);
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 1);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 4);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 2);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 4);
        pause();
        assert_eq!(db_io::read(&path)[0].secs_since_creation(), 3);
        assert_eq!(db_io::read(&path)[1].secs_since_creation(), 4);
    }

    fn pause() {
        let pause_secs = 1;
        std::thread::sleep(std::time::Duration::from_secs(pause_secs));
    }

    #[test]
    fn no_duplicate_task_names() {
        let path = std::path::Path::new("test_no_duplicate_task_names.json").to_path_buf();
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"[]")
            .unwrap();
        let mut tm = TaskManager::new(path.clone());
        tm.start_activity("a");
        tm.start_activity("a");
        tm.start_activity("a");
        tm.start_activity("a");
        assert_eq!(
            db_io::read(&path)
                .iter()
                .map(|f| f.name())
                .collect::<Vec<String>>(),
            vec!["a"]
        );
    }

    #[test]
    fn taskmanager_json_has_all_fields() {
        let path = std::path::Path::new("test_taskmanager_json_has_all_fields.json").to_path_buf();
        std::fs::File::create(&path)
            .unwrap()
            .write_all(b"[]")
            .unwrap();
        let tm = TaskManager::new(path);
        let tm_json = serde_json::to_string(&tm.today_times()).unwrap();
        assert!(tm_json.contains(&"tasks"));
        assert!(tm_json.contains(&"start_time_pretty"));
        assert!(tm_json.contains(&"elapsed_day"));
        assert!(tm_json.contains(&"total_activity_time"));
        assert!(tm_json.contains(&"time_difference"));
        assert!(tm_json.contains(&"start_time"));
        assert!(tm_json.contains(&"display"));
    }
}
