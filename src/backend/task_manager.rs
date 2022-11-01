use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, collections::HashMap};

use crate::backend::Task;

use super::task::TaskId;

#[derive(Serialize, Deserialize)]
struct TasksState {
    tasks: HashMap<TaskId, Task>,
    valid_states: Vec<String>,
    next_valid_id: u64,
}

pub struct TaskManager {
    tasks_state: TasksState,
    file_path_opt: Option<String>,
}

impl TaskManager {
    pub fn new(file_path_opt: Option<String>) -> TaskManager {
        TaskManager {
            tasks_state: TasksState {
                tasks: HashMap::new(),
                valid_states: Vec::new(),
                next_valid_id: 0,
            },
            file_path_opt
        }
    }

    pub fn initialize(&mut self) {
        if let Some(ref file_path) = self.file_path_opt {
            let open_file_result = File::open(file_path);
            if let Ok(file) = open_file_result {
                let tasks_state_from_json = serde_json::from_reader(file);
                if let Ok(tasks_state) = tasks_state_from_json {
                    self.tasks_state = tasks_state;
                    return;
                }
            }
        }
        //We could not initialize, we are going to add some default states
        self.add_state("Open".to_string());
        self.add_state("Selected for development".to_string());
        self.add_state("In progress".to_string());
        self.add_state("Done".to_string());
    }

    pub fn save(&self) {
        if let Some(ref file_path) = self.file_path_opt {
            let create_file_result = File::create(file_path);
            if let Ok(mut created_file) = create_file_result {
                let serialized_state_result = serde_json::to_string(&self.tasks_state);
                if let Ok(serialized_state) = serialized_state_result {
                    let write_result = created_file.write(serialized_state.as_bytes());
                    if let Err(error) = write_result {
                        panic!("{}", error);
                    }
                }
            }
        } 
    }

    pub fn add_task(&mut self, title: String, state: String, description: String) -> Option<TaskId> {
        let new_task = Task {
            id: TaskId(self.tasks_state.next_valid_id),
            title,
            state,
            description,
            parent_task: None,
            child_tasks: Vec::new(),
        };

        let task_id = new_task.id;
        self.tasks_state.next_valid_id += 1;

        if !self.is_task_valid(&new_task) {
            return None;
        }
        self.tasks_state.tasks.insert(task_id, new_task);
        self.save();
        Some(task_id)
    }

    pub fn get_tasks(&self) -> Vec<&Task> {
        self.tasks_state.tasks.iter().map(|kv| kv.1).collect()
    }

    pub fn find_task(&self, task_id: TaskId) -> Option<&Task> {
        self.tasks_state.tasks.get(&task_id)
    }

    pub fn add_state(&mut self, state: String) {
        if self.tasks_state.valid_states.contains(&state) {
            return;
        }
        self.tasks_state.valid_states.push(state);
    }

    pub fn get_states(&self) -> &Vec<String> {
        &self.tasks_state.valid_states
    }


    fn is_task_valid(&self, task: &Task) -> bool {
        self.tasks_state.valid_states.contains(&task.state)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::task;

    use super::*;

    fn create_task_manager() -> TaskManager {
        let mut task_manager = TaskManager::new(None);
        task_manager.initialize();
        task_manager
    }

    #[test]
    fn add_task() {
        let mut task_manager = create_task_manager();
        let task = task_manager.add_task(String::from("Title"), String::from("Open"), String::from("Description"));
        assert_eq!(task.is_some(), true);
    }

    #[test]
    fn add_task_wrong_state() {
        let mut task_manager = create_task_manager();
        let task = task_manager.add_task(String::from("Title"), String::from("invalid"), String::from("Description"));
        assert_eq!(task.is_none(), true);
    }

    #[test]
    fn find_task_not_found_task() {
        let task_manager = create_task_manager();
        let task = task_manager.find_task(TaskId(1));
        assert_eq!(task.is_none(), true);
    }

    #[test]
    fn find_task() {
        let mut task_manager = create_task_manager();
        let added_task = task_manager.add_task(String::from("Title"), String::from("Open"), String::from("Description"));
        let found_task = task_manager.find_task(added_task.unwrap());
        assert_eq!(found_task.is_some(), true);
    }

    #[test]
    fn add_state() {
        let mut task_manager = create_task_manager();
        task_manager.add_state(String::from("Foo"));
        let added_task = task_manager.add_task(String::from("Title"), String::from("Foo"), String::from("Description"));
        assert_eq!(added_task.is_some(), true);
    }
}