use crate::app::ApplicationBackend;
use crate::backend::Task;
use crate::frontend::core::{Logic, Menu, MenuEvent, UIContext};
use crate::frontend::widgets::input_widget::InputWidgetFocusState;
use crate::frontend::widgets::InputWidget;
use crossterm::event::{KeyCode, KeyEvent};
use std::{cell::RefCell, rc::Rc};
use tui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Paragraph},
    Frame,
};

pub struct CreateTaskMenu {
    logic: Rc<RefCell<Logic>>,
    ui_context: Option<Rc<RefCell<UIContext>>>,
    inputs: [InputWidget; 3],
    selected_input: u8,
}

impl CreateTaskMenu {
    pub fn new(logic: Rc<RefCell<Logic>>) -> CreateTaskMenu {
        CreateTaskMenu {
            logic: Rc::clone(&logic),
            ui_context: None,
            inputs: [InputWidget::new(), InputWidget::new(), InputWidget::new()],
            selected_input: 0,
        }
    }
}

impl Menu for CreateTaskMenu {
    fn initialize(&mut self, ui_context: Rc<RefCell<UIContext>>) {
        self.ui_context = Some(Rc::clone(&ui_context));
        self.inputs[0].set_focus_state(InputWidgetFocusState::Focused);
    }

    fn render(&mut self, frame: &mut Frame<ApplicationBackend>) {
        //let task_manager = &mut self.logic.borrow_mut().task_manager;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(5), Constraint::Percentage(13), Constraint::Percentage(72), Constraint::Percentage(10)].as_ref())
            .split(frame.size());
        render_input_widget_with_title(frame, &self.inputs[0], String::from("Title: "), chunks[0], 13);
        render_input_widget_with_title(frame, &self.inputs[1], String::from("State: "), chunks[1], 13);
        render_input_widget_with_title(frame, &self.inputs[2], String::from("Description: "), chunks[2], 13);
        let mut title_text_str = String::from("Accept");
        if (self.selected_input as usize) == self.inputs.len() {
            title_text_str.insert(0, '>');
        }
        let title_text = Paragraph::new(title_text_str).style(Style::default().fg(Color::White).bg(Color::Black)).alignment(Alignment::Center);
        frame.render_widget(title_text, chunks[3])
    }

    fn on_key_pressed(&mut self, key: KeyEvent) -> Option<MenuEvent> {
        let selected_input = self.selected_input as usize;
        if selected_input < self.inputs.len() {
            self.inputs[selected_input].process_input(key.code);
        }
        match key.code {
            KeyCode::Esc => return Some(MenuEvent::Quit),
            KeyCode::Up => {
                if self.selected_input > 0 {
                    if (self.selected_input as usize) < self.inputs.len() {
                        self.inputs[selected_input].set_focus_state(InputWidgetFocusState::NotFocused);
                    }
                    self.selected_input -= 1;
                    self.inputs[self.selected_input as usize].set_focus_state(InputWidgetFocusState::Focused);
                }
            },
            KeyCode::Down | KeyCode::Tab => {
                if selected_input < self.inputs.len() {
                    self.inputs[selected_input].set_focus_state(InputWidgetFocusState::NotFocused);
                    self.selected_input += 1;
                    if (self.selected_input as usize) < self.inputs.len() {
                        self.inputs[self.selected_input as usize].set_focus_state(InputWidgetFocusState::Focused);
                    }
                }
            },
            KeyCode::Enter => {
                if selected_input == self.inputs.len() {
                    if self.inputs.iter().any(|input| input.get_current_text().len() == 0) {
                        return None;
                    }
                    let new_task = Task {
                        title: self.inputs[0].get_current_text(),
                        state: self.inputs[1].get_current_text(),
                        description: self.inputs[2].get_current_text(),
                    };
                    let mut logic = self.logic.borrow_mut();
                    logic.task_manager.add_task(new_task);
                    return Some(MenuEvent::Quit);
                }
            }
            _ => (),
        };
        None
    }

    fn update(&mut self, elapsed_time: std::time::Duration) {
        let selected_input = self.selected_input as usize;
        if selected_input < self.inputs.len() {
            self.inputs[selected_input].update(elapsed_time);
        }
    }
}


fn render_input_widget_with_title(frame: &mut Frame<ApplicationBackend>, input_widget: &InputWidget, title: String, area: Rect, max_title_width: u16) {
    let title_text = Paragraph::new(title)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    let line = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Length(max_title_width),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(area);
    let title_with_margin = Layout::default()
        .vertical_margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(line[0]);
    frame.render_widget(title_text, title_with_margin[0]);
    input_widget.render(frame, line[1]);
}