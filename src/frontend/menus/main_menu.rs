use std::{rc::Rc, cell::RefCell};

use crate::app::{ApplicationBackend};
use crate::backend::{Task, TaskManager, task};
use crate::frontend::core::StatefulList;
use crate::frontend::menus::{Menu};
use crossterm::event::{KeyEvent, KeyCode};

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub struct MainMenu {
    task_manager: Rc<RefCell<TaskManager>>,
    task_list: StatefulList<Task>,
}

impl MainMenu {
    pub fn new(task_manager: Rc<RefCell<TaskManager>>) -> MainMenu {
        let tasks = task_manager.borrow().get_tasks().to_vec();
        MainMenu {
            task_manager,
            task_list : StatefulList::with_items(tasks),
        }
    }
}

impl Menu for MainMenu {
    fn render(&mut self, frame: &mut Frame<ApplicationBackend>) {
        let task_manager = self.task_manager.borrow_mut();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(frame.size());
        let tasks: Vec<ListItem> = task_manager
            .get_tasks()
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.title.clone()))]))
            .collect();
        let tasks = List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        frame.render_stateful_widget(tasks, chunks[0], &mut self.task_list.state);
    }

    fn on_key_pressed(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.task_list.previous(),
            KeyCode::Down => self.task_list.next(),
            KeyCode::Char(_c) => {
                //if 'n' == c {
                //    let new_menu = Box::new(MainMenu::new());
                //    self.execute_menu_fn.unwrap()(new_menu).unwrap();
                //}
            } 
            _ => (),
        }
    }
}
