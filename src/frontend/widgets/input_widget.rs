use crate::frontend::core::ApplicationBackend;
use crossterm::event::KeyCode;
use std::time::{Duration};
use tui::{
    layout::{Rect},
    style::{Color, Style},
    widgets::Borders,
    widgets::{Block, Paragraph},
    Frame,
};

struct CursorState {
    tick_rate: Duration,
    tick_rate_milliseconds_left: i32,
    is_cursor_showing: bool,
}

#[derive(PartialEq)]
pub enum InputWidgetFocusState {
    Focused,
    NotFocused,
}

pub struct InputWidget {
    text: String,
    focus_state: InputWidgetFocusState,
    cursor_state: Option<CursorState>,
    allow_new_lines: bool,
}

impl InputWidget {
    fn new(allow_new_lines: bool) -> InputWidget {
        InputWidget {
            text: String::new(),
            focus_state: InputWidgetFocusState::NotFocused,
            cursor_state: None,
            allow_new_lines,
        }
    }

    pub fn create_text_area() -> InputWidget {
        Self::new(true)
    }

    pub fn create_text_label() -> InputWidget {
        Self::new(false)
    }

    pub fn get_current_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_focus_state(&mut self, focus_state: InputWidgetFocusState) {
        if self.focus_state == focus_state {
            return;
        }
        self.focus_state = focus_state;
        if self.focus_state == InputWidgetFocusState::Focused {
            self.cursor_state = Some(CursorState {
                tick_rate: Duration::from_millis(500),
                tick_rate_milliseconds_left: 500,
                is_cursor_showing: true,
            });
        } else {
            self.cursor_state = None;
        }
    }

    pub fn process_input(&mut self, key_code: KeyCode) {
        if self.focus_state == InputWidgetFocusState::NotFocused {
            return ();
        }
        let mut modified_text = false;
        match key_code {
            KeyCode::Char(c) => {
                self.text.push(c);
                modified_text = true;
            },
            KeyCode::Backspace => {
                self.text.pop();
                modified_text = true;
            }
            KeyCode::Enter => {
                if self.allow_new_lines {
                    self.text.push('\n');
                }
            }
            _ => (),
        };
        if modified_text {
            if let Some(cursor_state) = &mut self.cursor_state {
                cursor_state.tick_rate_milliseconds_left = cursor_state.tick_rate.as_millis() as i32;
                cursor_state.is_cursor_showing = true;
            }
        }
    }

    pub fn render(&self, frame: &mut Frame<ApplicationBackend>, area: Rect) {
        let mut text_to_show = self.text.clone();
        if let Some(cursor_state) = &self.cursor_state {
            if cursor_state.is_cursor_showing {
                text_to_show.push('|')
            }
        }
        let text = Paragraph::new(text_to_show)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black));
        frame.render_widget(text, area)
    }

    pub fn update(&mut self, duration: Duration) {
        if let Some(cursor_state) = &mut self.cursor_state {
            cursor_state.tick_rate_milliseconds_left -= duration.as_millis() as i32;
            if cursor_state.tick_rate_milliseconds_left <= 0 {
                cursor_state.tick_rate_milliseconds_left += cursor_state.tick_rate.as_millis() as i32;
                cursor_state.is_cursor_showing = !cursor_state.is_cursor_showing;
            }
        }
    }
}
