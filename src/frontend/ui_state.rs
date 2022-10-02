use crate::backend::{Task, TaskManager};
use tui::widgets::{ListState, List};

pub struct UIState {
    pub task_list: StatefulList<Task>,
}

impl UIState {
    pub fn new(task_manager: &TaskManager) -> UIState {
        UIState {
            task_list: StatefulList::with_items(task_manager.get_tasks().to_vec()),
        }
    }

    pub fn on_up(&mut self) {
        self.task_list.previous();
    }

    pub fn on_down(&mut self) {
        self.task_list.next();
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0));

        StatefulList {
            state,
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
