use super::task::Task;
use serde::Serialize;
use serde_json;

/**
The task manager is the only struct one exposed. It manages a vec of tasks.
Only one task can be active at a time. Running tasks are exclusive, starting
a task will stop all other tasks.
**/
#[derive(Debug, PartialEq, Serialize)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
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
        self.tasks
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
    }
}

pub fn taskmanager_as_string(tm: &TaskManager) -> String {
    serde_json::to_string_pretty(tm).unwrap()
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
