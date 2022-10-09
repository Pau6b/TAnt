pub mod stateful_list;
pub use stateful_list::StatefulList;

use crate::backend::TaskManager;

pub struct UIContext {
    task_manager : TaskManager,
}