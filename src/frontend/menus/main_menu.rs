use std::collections::HashSet;
use std::{cell::RefCell, rc::Rc};

use crate::app::{execute_menu, ApplicationBackend};
use crate::backend::task::{Task, TaskId};
use crate::backend::TaskManager;
use crate::frontend::widgets::bottom_bar::BottomBarAction;
use crate::frontend::widgets::BottomBar;
use crate::frontend::{
    core::{Logic, Menu, MenuEvent, StatefulList, UIContext},
    menus::CreateTaskMenu,
};
use crossterm::event::{KeyCode, KeyEvent};

use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

struct TaskUIView {
    task: Task,
    depth: usize,
}

pub struct MainMenu {
    logic: Rc<RefCell<Logic>>,
    ui_context: Option<Rc<RefCell<UIContext>>>,
    task_list: StatefulList<TaskUIView>,
    bottom_bar: BottomBar,
}

impl MainMenu {
    pub fn new(logic: Rc<RefCell<Logic>>) -> MainMenu {
        let mut bottom_bar = BottomBar::new();
        bottom_bar.add_action(KeyCode::Char('n'), BottomBarAction::CreateTask);
        bottom_bar.add_action(KeyCode::Char('s'), BottomBarAction::CreateTaskWithParent);
        bottom_bar.add_action(KeyCode::Esc, BottomBarAction::Exit);

        MainMenu {
            logic: Rc::clone(&logic),
            ui_context: None,
            task_list: StatefulList::new(),
            bottom_bar,
        }
    }

    fn refresh_tasks(&mut self) {
        let task_manager = &self.logic.borrow_mut().task_manager;
        let tasks = task_manager.get_tasks();
        let mut viewed_tasks = HashSet::<TaskId>::new();
        let mut task_ui_views = Vec::<TaskUIView>::new();
        tasks.iter().for_each(|task| {
            if !viewed_tasks.contains(&task.id) {
                add_subtasks(
                    &task_manager,
                    task.id,
                    &mut viewed_tasks,
                    &mut task_ui_views,
                    0,
                );
            }
        });
        self.task_list = StatefulList::with_items(task_ui_views);
    }

    fn render_tasks(&self, frame: &mut Frame<ApplicationBackend>, rect: Rect) {
        let constraints: Vec<Constraint> = self
            .task_list
            .items
            .iter()
            .map(|_| Constraint::Length(1))
            .collect();

        let block = Block::default().borders(Borders::ALL).title("Task List");
        let render_rect = block.inner(rect);
        frame.render_widget(block, rect);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(render_rect);

        for i in 0..self.task_list.items.len() {
            let ui_task = &self.task_list.items[i];
            let offset = ui_task.depth * 4;
            let line_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(offset as u16),
                    Constraint::Percentage(1),
                ])
                .split(chunks[i]);
            {
                let mut text_str = String::from("  ");
                text_str.push_str(&ui_task.task.title);
                if i == self.task_list.state.selected().unwrap() {
                    text_str.replace_range(1..2, ">");
                }
                let text = Paragraph::new(text_str).block(Block::default());
                frame.render_widget(text, line_chunks[1]);
            }
            if offset > 0 {
                let mut text_str = String::from(" ".repeat(offset-2));
                text_str.push_str("└─");
                let text = Paragraph::new(text_str).block(Block::default());
                frame.render_widget(text, line_chunks[0]);
            }
        }
    }
}

impl Menu<()> for MainMenu {
    fn initialize(&mut self, ui_context: Rc<RefCell<UIContext>>) {
        self.ui_context = Some(Rc::clone(&ui_context));
        self.refresh_tasks();
    }

    fn render(&mut self, frame: &mut Frame<ApplicationBackend>) {
        let area = self.bottom_bar.render(frame, frame.size());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(area);
        //let tasks: Vec<ListItem> = task_manager
        //    .get_tasks()
        //    .iter()
        //    .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.title.clone()))]))
        //    .collect();
        //let tasks = List::new(tasks)
        //    .block(Block::default().borders(Borders::ALL).title("Task List"))
        //    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        //    .highlight_symbol("> ");
        //frame.render_stateful_widget(tasks, chunks[0], &mut self.task_list.state);
        self.render_tasks(frame, chunks[0]);
    }

    fn on_key_pressed(&mut self, key: KeyEvent) -> Option<MenuEvent<()>> {
        match key.code {
            KeyCode::Up => self.task_list.previous(),
            KeyCode::Down => self.task_list.next(),
            KeyCode::Char(c) => {
                let pressed_char = c.to_ascii_lowercase();
                if 'n' == pressed_char || 's' == pressed_char {
                    let mut parent_task: Option<TaskId> = None;
                    if 's' == pressed_char {
                        let selected_index = self.task_list.state.selected().unwrap();
                        parent_task = Some(self.task_list.items[selected_index].task.id);
                    }
                    let mut new_menu: Box<dyn Menu<Option<TaskId>>> =
                        Box::new(CreateTaskMenu::new(Rc::clone(&self.logic), parent_task));
                    let menu_execution_result =
                        execute_menu(&mut new_menu, Rc::clone(self.ui_context.as_ref().unwrap()));
                    match menu_execution_result {
                        Ok(created_task_id_opt) => {
                            if let Some(_) = created_task_id_opt {
                                self.refresh_tasks();
                                return Some(MenuEvent::MenuExecutionResult(Ok(())));
                            }
                        }
                        Err(e) => return Some(MenuEvent::MenuExecutionResult(Err(e))),
                    }
                }
            }
            KeyCode::Esc => return Some(MenuEvent::Quit(())),
            _ => (),
        }
        None
    }

    fn update(&mut self, _elapsed_time: std::time::Duration) {}
}

fn add_subtasks(
    task_manager: &TaskManager,
    task_id: TaskId,
    viewed_tasks: &mut HashSet<TaskId>,
    ui_tasks: &mut Vec<TaskUIView>,
    depth: usize,
) {
    let task = task_manager.find_task(task_id).unwrap();
    ui_tasks.push(TaskUIView {
        task: task.clone(),
        depth,
    });
    viewed_tasks.insert(task_id);
    task.child_tasks
        .iter()
        .for_each(|t| add_subtasks(task_manager, *t, viewed_tasks, ui_tasks, depth + 1));
}
