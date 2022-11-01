use core::panic;

use crate::frontend::core::ApplicationBackend;
use crossterm::event::KeyCode;
use tui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    Frame, widgets::Paragraph, style::{Style, Color},
};

#[derive(Copy, Clone)]
pub enum BottomBarAction {
    CreateTask,
    Submit,
    Exit,
}

struct BottomBarActionConfig {
    key: KeyCode,
    action: BottomBarAction, //#pau_todo do we really need this?
    description: String,
}

pub struct BottomBar {
    actions: Vec<BottomBarActionConfig>,
}

impl BottomBar {
    pub fn new() -> BottomBar {
        BottomBar {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, key: KeyCode, action: BottomBarAction) {
        self.actions.push(BottomBarActionConfig {
            key,
            action,
            description: format!(
                "[{}] {}",
                key_to_string(key),
                get_bottom_bar_action_description(action)
            ),
        });
    }

    pub fn render(&self, frame: &mut Frame<ApplicationBackend>, area: Rect) -> Rect {
        if self.actions.is_empty() {
            return area;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Max(100), Constraint::Length(3)].as_ref())
            .split(frame.size());
        let full_bottom_bar_string = self.actions.iter().rev().fold("".to_string(), |mut acc, action| {
            if acc.len() > 0 {
                acc.insert_str(0, " | ");
            }
            acc.insert_str(0,&action.description);
            return acc;
        });
        let bottom_bar_paragraph = Paragraph::new(full_bottom_bar_string).style(Style::default().fg(Color::White).bg(Color::Black)).alignment(Alignment::Center);
        frame.render_widget(bottom_bar_paragraph, chunks[1]);
        chunks[0]
    }
}

fn get_bottom_bar_action_description(action: BottomBarAction) -> String {
    match action {
        BottomBarAction::Exit => "Exit",
        BottomBarAction::Submit => "Submit",
        BottomBarAction::CreateTask => "Create task",
    }
    .to_string()
}

fn key_to_string(key_code: KeyCode) -> String {
    if let KeyCode::Char(c) = key_code {
        return c.to_string().to_uppercase();
    }
    match key_code {
        KeyCode::Esc => "Esc",
        KeyCode::Enter => "Enter",
        _ => {
            panic!("Key not handled");
        }
    }
    .to_string()
}
