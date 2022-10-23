use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::frontend::{core::ApplicationBackend, widgets::Widget};

use super::{FocusState, FocusableWidget};

pub struct MultipleOptionWidget {
    selected_option: Option<u32>,
    options: Vec<String>,
    direction: tui::layout::Direction,
    focus_state: FocusState,
}

impl MultipleOptionWidget {
    pub fn new(options: &Vec<String>, direction: tui::layout::Direction) -> MultipleOptionWidget {
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
    fn render(&self, frame: &mut Frame<ApplicationBackend>, area: tui::layout::Rect) {
        let constraints: Vec<Constraint> = self
            .options
            .iter()
            .map(|option| Constraint::Length(option.len() as u16 + 2))
            .collect();
        let chunks = Layout::default()
            .direction(self.direction.clone())
            .margin(1)
            .constraints(constraints)
            .split(area);
        for i in 0..self.options.len() {
            let mut style = Style::default().fg(Color::White).bg(Color::Black);
            if let Some(selected_input) = self.selected_option.clone() {
                if selected_input == i as u32 {
                    style = style.add_modifier(Modifier::UNDERLINED);
                }
            }
            let text = Paragraph::new(self.options[i].clone())
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
}
