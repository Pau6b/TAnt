use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

use crate::backend::Task;

#[derive(Serialize, Deserialize)]
struct TasksState {
    tasks: Vec<Task>,
    valid_states: Vec<String>,
}

pub struct TaskManager {
    tasks_state: TasksState,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks_state: TasksState {
                tasks: Vec::new(),
                valid_states: Vec::new(),
            },
        }
    }

    pub fn initialize(&mut self) {
        let open_file_result = File::open("state.json");
        if let Ok(file) = open_file_result {
            let tasks_state_from_json = serde_json::from_reader(file);
            if let Ok(tasks_state) = tasks_state_from_json {
                self.tasks_state = tasks_state;
                return;
            }
        }
        //We could not initialize, we are going to add some default states
        self.add_state("Open".to_string());
        self.add_state("Selected for development".to_string());
        self.add_state("In progress".to_string());
        self.add_state("Done".to_string());
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
        if !self.is_task_valid(&i_task) {
            return;
        }
        self.tasks_state.tasks.push(i_task);
        self.save();
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks_state.tasks
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

    pub fn is_task_valid(&self, task: &Task) -> bool {
        self.tasks_state.valid_states.contains(&task.state)
    }
}
