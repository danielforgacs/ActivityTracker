use super::task::*;
use serde::Serialize;
use chrono::{DateTime, Local};
use serde::ser::{Serializer, SerializeStruct};

/**
The task manager is the only struct one exposed. It manages a vec of tasks.
Only one task can be active at a time. Running tasks are exclusive, starting
a task will stop all other tasks.
**/
#[derive(Debug, PartialEq, Clone)]
pub struct TaskManager {
    tasks: Vec<Task>,
    start_time: String,
}

impl TaskManager {
    pub fn new() -> Self {
        let now: DateTime<Local> = Local::now();
        Self {
            tasks: Vec::new(),
            start_time: format!("start time: {}", now),
        }
    }

    pub fn activate(&mut self, name: &str) {
        if !self.task_exists(name) {
            self.tasks.push(
                Task::from(name)
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

    pub fn stop(&mut self, name: &str) -> bool {
        for task in self.tasks.iter_mut() {
            if task.name() == name {
                task.stop();
                return true;
            }
        }
        false
    }

    fn task_names(&self) -> Vec<String> {
        self.tasks.iter().map(|f| f.name()).collect::<Vec<String>>()
    }

    fn task_exists(&self, name: &str) -> bool {
        self.task_names().contains(&name.to_string())
    }

    pub fn times(&self) -> String {
        let mut result = self.start_time.to_owned();
        let total_activity_time: SecType = self.tasks
            .iter()
            .map(|t| t.elapsed_time())
            .sum();
        let (hours, minutes) = secs_to_time(total_activity_time);
        result.push_str(
            &format!("\ntotal acivity time: {:02}h:{:02}m\n\n", hours, minutes)
        );
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

    fn total_time(&self) -> SecType {
        self.tasks
            .iter()
            .map(|t| t.elapsed_time())
            .sum()
    }
}

impl Serialize for TaskManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (hours, mins) = secs_to_time(self.total_time());
        let total_time = format!("{}h:{:02}m", hours, mins);
        let mut state = serializer.serialize_struct("Taskmanager", 3)?;
        state.serialize_field("tasks", &self.tasks)?;
        state.serialize_field("start_time", &self.start_time)?;
        state.serialize_field("total_time:", &total_time)?;
        state.serialize_field("display:", &self.times())?;
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
        tm.activate(task_name);
        assert_eq!(tm.tasks.len(), 1);
        assert_eq!(tm.tasks[0].name(), task_name);
        assert_eq!(tm.tasks[0].elapsed_time(), 0);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        tm.stop(task_name);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        tm.activate(task_name);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].elapsed_time(), 3);
    }

    #[test]
    fn multiple_tasks() {
        let task_1 = "alpha";
        let task_2 = "beta";
        let mut tm = TaskManager::new();
        tm.activate(task_1);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        tm.activate(task_2);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 1);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 2);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 3);
        tm.activate(task_2);
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 3);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 4);
        tm.activate(task_1);
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 4);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        assert_eq!(tm.tasks[1].elapsed_time(), 4);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 3);
        assert_eq!(tm.tasks[1].elapsed_time(), 4);
    }

    fn pause() {
        let pause_secs = 1;
        std::thread::sleep(std::time::Duration::from_secs(pause_secs));
    }

    #[test]
    fn no_duplicate_task_names() {
        let mut tm = TaskManager::new();
        tm.activate("a");
        tm.activate("a");
        tm.activate("a");
        tm.activate("a");
        assert_eq!(tm.task_names(), vec!["a"]);
    }
}
