use crate::backend::TaskManager;
use crate::frontend::UIState;

use tui::{
    backend::Backend,
    layout::{Direction, Layout, Constraint},
    widgets::{Block, Borders, List, ListItem},
    style::{Modifier, Style},
    text::{Span, Spans},
    Frame,
};

pub fn render_main_menu<B: Backend>(frame: &mut Frame<B>, task_manager: &mut TaskManager, ui_state: &mut UIState) {
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
    frame.render_stateful_widget(tasks, chunks[0], &mut ui_state.task_list.state);
}
