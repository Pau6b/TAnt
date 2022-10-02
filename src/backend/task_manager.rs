use crate::backend::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {

    pub fn new() -> TaskManager {
        TaskManager {
            tasks : Vec::new(),
        }
    }

    pub fn add_task(&mut self, i_task: Task) {
        self.tasks.push(i_task);
    }

    pub fn get_tasks(&self) -> &Vec<Task>
    {
        &self.tasks
    }

}