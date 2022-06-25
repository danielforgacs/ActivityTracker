use super::task::Task;

#[derive(Debug, PartialEq)]
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn new_task(&mut self, name: &str) {

    }

    fn stop_task(&mut self, name: &str) {

    }

    fn stop_all(&mut self) {

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
        tm.new_task(task_name);
    }
}
