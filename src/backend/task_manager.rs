use std::{fs::File, io::Write};
use serde::{Serialize, Deserialize};

use crate::backend::Task;

#[derive(Serialize, Deserialize)]
struct TasksState {
    tasks: Vec<Task>,
}

pub struct TaskManager {
    tasks_state: TasksState,
}

impl TaskManager {

    pub fn new() -> TaskManager {
        TaskManager {
            tasks_state : TasksState{tasks: Vec::new()},
        }
    }

    pub fn initialize(&mut self) {
        let open_file_result = File::open("state.json"); 
        if let Ok(file) = open_file_result {
            let tasks_state_from_json = serde_json::from_reader(file);
            if let Ok(tasks_state) = tasks_state_from_json {
                self.tasks_state = tasks_state;
            }
        }
    }

    pub fn save(&self) {
        let create_file_result = File::create("state.json");
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

    pub fn add_task(&mut self, i_task: Task) {
        self.tasks_state.tasks.push(i_task);
        self.save();
    }

    pub fn get_tasks(&self) -> &Vec<Task>
    {
        &self.tasks_state.tasks
    }

}