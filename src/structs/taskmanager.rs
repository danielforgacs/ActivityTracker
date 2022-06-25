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
        self.tasks.push(
            Task::from(name)
        );
        self.start_task(name);
    }

    fn stop_task(&mut self, name: &str) {
        for task in self.tasks.iter_mut() {
            if task.name() == name {
                task.stop();
            }
        }
    }

    fn start_task(&mut self, name: &str) {
        for task in self.tasks.iter_mut() {
            if task.name() == name {
                task.start();
            } else {
                task.stop();
            }
        }
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
        tm.stop_task(task_name);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        tm.start_task(task_name);
        assert_eq!(tm.tasks[0].elapsed_time(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(tm.tasks[0].elapsed_time(), 3);
    }

    #[test]
    fn multiple_tasks() {
        let task_1 = "alpha";
        let task_2 = "beta";
        let mut tm = TaskManager::new();
        tm.new_task(task_1);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        tm.new_task(task_2);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 1);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 2);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 3);
        tm.start_task(task_2);
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 3);
        pause();
        assert_eq!(tm.tasks[0].elapsed_time(), 1);
        assert_eq!(tm.tasks[1].elapsed_time(), 4);
        tm.start_task(task_1);
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
}
