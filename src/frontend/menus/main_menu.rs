use std::{cell::RefCell, rc::Rc};

use crate::app::{execute_menu, ApplicationBackend};
use crate::backend::Task;
use crate::frontend::widgets::BottomBar;
use crate::frontend::widgets::bottom_bar::BottomBarAction;
use crate::frontend::{
    core::{Logic, Menu, MenuEvent, StatefulList, UIContext},
    menus::CreateTaskMenu,
};
use crossterm::event::{KeyCode, KeyEvent};

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub struct MainMenu {
    logic: Rc<RefCell<Logic>>,
    ui_context: Option<Rc<RefCell<UIContext>>>,
    task_list: StatefulList<Task>,
    bottom_bar: BottomBar,
}

impl MainMenu {
    pub fn new(logic: Rc<RefCell<Logic>>) -> MainMenu {
        let tasks = {
            let task_manager = &logic.borrow().task_manager;
            task_manager.get_tasks().to_vec()
        };
        let mut bottom_bar = BottomBar::new();
        bottom_bar.add_action(KeyCode::Char('n'), BottomBarAction::CreateTask);
        bottom_bar.add_action(KeyCode::Esc, BottomBarAction::Exit);

        MainMenu {
            logic: Rc::clone(&logic),
            ui_context: None,
            task_list: StatefulList::with_items(tasks),
            bottom_bar,
        }
    }
}

impl Menu for MainMenu {
    fn initialize(&mut self, ui_context: Rc<RefCell<UIContext>>) {
        self.ui_context = Some(Rc::clone(&ui_context));
    }

    fn render(&mut self, frame: &mut Frame<ApplicationBackend>) {
        let task_manager = &mut self.logic.borrow_mut().task_manager;
        let area = self.bottom_bar.render(frame, frame.size());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(area);
        let tasks: Vec<ListItem> = task_manager
            .get_tasks()
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.title.clone()))]))
            .collect();
        let tasks = List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title("Task List"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        frame.render_stateful_widget(tasks, chunks[0], &mut self.task_list.state);
    }

    fn on_key_pressed(&mut self, key: KeyEvent) -> Option<MenuEvent> {
        match key.code {
            KeyCode::Up => self.task_list.previous(),
            KeyCode::Down => self.task_list.next(),
            KeyCode::Char(c) => {
                if 'n' == c {
                    let mut new_menu: Box<dyn Menu> =
                        Box::new(CreateTaskMenu::new(Rc::clone(&self.logic)));
                    let execution_result = Some(MenuEvent::MenuExecutionResult(execute_menu(
                        &mut new_menu,
                        Rc::clone(self.ui_context.as_ref().unwrap()),
                    )));
                    let logic = self.logic.borrow_mut();
                    if self.task_list.items.len() != logic.task_manager.get_tasks().len() {
                        self.task_list.items.push(logic.task_manager.get_tasks().last().unwrap().clone());
                    }
                    return execution_result;
                }
            }
            KeyCode::Esc => return Some(MenuEvent::Quit),
            _ => (),
        }
        None
    }

    fn update(&mut self, _elapsed_time: std::time::Duration) {}
}
