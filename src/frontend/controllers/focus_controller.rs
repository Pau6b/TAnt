use crate::frontend::widgets::{FocusState, FocusableWidget};
use crossterm::event::KeyCode;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct FocusController {
    focusable_widgets: Vec<Rc<RefCell<dyn FocusableWidget>>>,
    selected_widget: usize,
}

impl FocusController {
    pub fn new(focusable_widgets: Vec<Rc<RefCell<dyn FocusableWidget>>>) -> FocusController {
        if focusable_widgets.len() > 0 {
            focusable_widgets[0]
                .borrow_mut()
                .focus_state_changed(FocusState::Focused);
        }
        FocusController {
            focusable_widgets,
            selected_widget: 0,
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.focusable_widgets
            .iter()
            .for_each(|focusable_widget| focusable_widget.borrow_mut().update(duration));
    }

    pub fn process_input(&mut self, key_code: KeyCode) {
        if self.focusable_widgets.len() == 0 {
            return;
        }
        match key_code {
            KeyCode::Up => {
                if self.selected_widget > 0 {
                    if (self.selected_widget as usize) < self.focusable_widgets.len() {
                        self.focusable_widgets[self.selected_widget]
                            .borrow_mut()
                            .focus_state_changed(FocusState::NotFocused);
                    }
                    self.selected_widget -= 1;
                    self.focusable_widgets[self.selected_widget as usize]
                        .borrow_mut()
                        .focus_state_changed(FocusState::Focused);
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                if self.selected_widget < self.focusable_widgets.len() - 1 {
                    self.focusable_widgets[self.selected_widget]
                        .borrow_mut()
                        .focus_state_changed(FocusState::NotFocused);
                    self.selected_widget += 1;
                    if (self.selected_widget as usize) < self.focusable_widgets.len() {
                        self.focusable_widgets[self.selected_widget as usize]
                            .borrow_mut()
                            .focus_state_changed(FocusState::Focused);
                    }
                }
            }
            _ => {
                let mut focused_wiget = self.focusable_widgets[self.selected_widget].borrow_mut();
                focused_wiget.process_input(key_code);
            }
        }
    }
}
