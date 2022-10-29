use std::time::Duration;

use crossterm::event::KeyCode;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::frontend::{core::ApplicationBackend, widgets::Widget};

use super::{FocusState, FocusableWidget};

pub struct MultipleOptionWidget {
    selected_option: Option<u32>,
    options: Vec<String>,
    direction: Direction,
    focus_state: FocusState,
}

impl MultipleOptionWidget {
    pub fn new(options: &Vec<String>, direction: Direction) -> MultipleOptionWidget {
        let selected_option = if options.len() > 0 { Some(0) } else { None };
        MultipleOptionWidget {
            selected_option,
            options: options.clone(),
            direction,
            focus_state: FocusState::NotFocused,
        }
    }

    pub fn get_selected_option(&self) -> Option<String> {
        if let Some(selected_option) = self.selected_option {
            return Some(self.options[selected_option as usize].clone());
        }
        return None;
    }
}

impl Widget for MultipleOptionWidget {
    fn render(&self, frame: &mut Frame<ApplicationBackend>, area: Rect) {
        let mut constraints: Vec<Constraint> = Vec::new();
        self
            .options
            .iter()
            .for_each(|option| {
                constraints.push(Constraint::Length(option.len() as u16));
                constraints.push(Constraint::Length(2));
            });
        let chunks = Layout::default()
            .direction(self.direction.clone())
            .margin(1)
            .constraints(constraints)
            .split(area);
        for i in 0..chunks.len() {
            if (i % 2) == 1 {
                continue;
            }
            let elem = i/2;
            let mut style = Style::default().fg(Color::White).bg(Color::Black);
            if let Some(selected_input) = self.selected_option.clone() {
                if selected_input == elem as u32 {
                    style = style.add_modifier(Modifier::UNDERLINED);
                }
            }
            let text = Paragraph::new(self.options[elem].clone())
                .block(Block::default())
                .style(style);
            frame.render_widget(text, chunks[i])
        }
    }
}

impl FocusableWidget for MultipleOptionWidget {
    fn focus_state_changed(&mut self, focus_state: FocusState) {
        self.focus_state = focus_state;
    }

    fn get_focus_state(&self) -> FocusState {
        self.focus_state
    }

    fn process_input(&mut self, key_code: KeyCode) {
        if self.options.len() == 0 {
            return;
        }
        match key_code {
            KeyCode::Left => {
                if let Some(ref mut selected_option) = self.selected_option {
                    if *selected_option > 0 {
                        *selected_option -= 1;
                    }
                } else {
                    self.selected_option = Some(0);
                }
            }
            KeyCode::Right => {
                if let Some(ref mut selected_option) = self.selected_option {
                    if *selected_option < self.options.len() as u32 - 1 {
                        *selected_option += 1;
                    }
                } else {
                    self.selected_option = Some(0);
                }
            }
            _ => (),
        }
    }

    fn update(&mut self, _duration: Duration) {}
}
