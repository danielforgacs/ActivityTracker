use super::task::Task;

#[derive(Debug, PartialEq)]
struct TaskManager {
    tasks: Vec<Task>,
}
 impl TaskManager {
    fn new() -> Self {
        Self { tasks: Vec::new() }
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
 }
