pub mod stateful_list;
pub use stateful_list::StatefulList;
pub mod menu;

pub use menu::{Menu, MenuEvent};
use crate::backend::TaskManager;
pub use crate::app::ApplicationBackend;
use tui::Terminal;

pub struct Logic {
    pub task_manager : TaskManager,
}

pub struct UIContext {
    pub terminal: Terminal<ApplicationBackend>,
}

impl Logic {
    pub fn new() -> Logic {
        let mut task_manager = TaskManager::new(Some(String::from("state.json")));
        task_manager.initialize();
        
        Logic {
            task_manager,
        }
    }
}