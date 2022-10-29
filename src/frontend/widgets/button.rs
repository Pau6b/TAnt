use std::time::Duration;

use crate::frontend::{
    core::ApplicationBackend,
    widgets::{FocusState, FocusableWidget, Widget},
};
use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub struct Button {
    text: String,
    focus_state: FocusState,
    text_alignment: Alignment,
}

impl Button {
    pub fn new(text: String, text_alignment: Alignment) -> Button {
        Button {
            text,
            focus_state: FocusState::NotFocused,
            text_alignment,
        }
    }
}

impl Widget for Button {
    fn render(&self, frame: &mut Frame<ApplicationBackend>, area: Rect) {
        let mut button_text_str = self.text.clone();
        if self.focus_state == FocusState::Focused {
            button_text_str.insert(0, '>');
        }
        let title_text = Paragraph::new(button_text_str)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(self.text_alignment);
        frame.render_widget(title_text, area)
    }
}

impl FocusableWidget for Button {
    fn focus_state_changed(&mut self, focus_state: FocusState) {
        self.focus_state = focus_state;
    }

    fn get_focus_state(&self) -> FocusState {
        self.focus_state.clone()
    }

    fn process_input(&mut self, _key_code: KeyCode) {

    }

    fn update(&mut self, _duration: Duration) {

    }
}
