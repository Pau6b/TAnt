use crate::app::ApplicationBackend;
use crate::backend::task::TaskId;
use crate::frontend::{
    controllers::FocusController,
    core::{Logic, Menu, MenuEvent, UIContext},
    widgets::{
        BottomBar, BottomBarAction, Button, FocusState, FocusableWidget, InputWidget,
        MultipleOptionWidget, Widget,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use std::{cell::{RefCell, Ref}, rc::Rc};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub struct CreateTaskMenu {
    logic: Rc<RefCell<Logic>>,
    ui_context: Option<Rc<RefCell<UIContext>>>,
    title_input: Rc<RefCell<InputWidget>>,
    description_input: Rc<RefCell<InputWidget>>,
    state_input: Rc<RefCell<MultipleOptionWidget>>,
    accept_button: Rc<RefCell<Button>>,
    bottom_bar: BottomBar,
    focus_controller: FocusController,
    parent_task: Option<TaskId>
}

impl CreateTaskMenu {
    pub fn new(logic: Rc<RefCell<Logic>>, parent_task: Option<TaskId>) -> CreateTaskMenu {
        let mut bottom_bar = BottomBar::new();
        bottom_bar.add_action(KeyCode::Enter, BottomBarAction::Submit);
        bottom_bar.add_action(KeyCode::Esc, BottomBarAction::Exit);

        let title_input = Rc::new(RefCell::new(InputWidget::create_text_label()));
        let description_input = Rc::new(RefCell::new(InputWidget::create_text_area()));
        let state_input = Rc::new(RefCell::new(MultipleOptionWidget::new(
            logic.borrow_mut().task_manager.get_states(),
            Direction::Horizontal,
        )));
        let accept_button = Rc::new(RefCell::new(Button::new(
            String::from("Accept"),
            Alignment::Center,
        )));

        let mut focusable_widgets: Vec<Rc<RefCell<dyn FocusableWidget>>> = Vec::new();
        focusable_widgets.reserve(4);
        let cloned_title = Rc::clone(&title_input);
        focusable_widgets.push(cloned_title);
        let cloned_title = Rc::clone(&state_input);
        focusable_widgets.push(cloned_title);
        let cloned_title = Rc::clone(&description_input);
        focusable_widgets.push(cloned_title);
        let cloned_title = Rc::clone(&accept_button);
        focusable_widgets.push(cloned_title);
        let focus_controller = FocusController::new(focusable_widgets);

        CreateTaskMenu {
            logic: Rc::clone(&logic),
            ui_context: None,
            title_input,
            description_input,
            state_input,
            accept_button,
            bottom_bar,
            focus_controller,
            parent_task,
        }
    }
}

impl Menu<Option<TaskId>> for CreateTaskMenu {
    fn initialize(&mut self, ui_context: Rc<RefCell<UIContext>>) {
        self.ui_context = Some(Rc::clone(&ui_context));
    }

    fn render(&mut self, frame: &mut Frame<ApplicationBackend>) {
        //let task_manager = &mut self.logic.borrow_mut().task_manager;
        let area = self.bottom_bar.render(frame, frame.size());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Percentage(72),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(area);
        render_input_widget_with_title(
            frame,
            self.title_input.borrow(),
            String::from("Title: "),
            chunks[0],
            13,
        );
        render_input_widget_with_title(
            frame,
            self.state_input.borrow(),
            String::from("State: "),
            chunks[1],
            13,
        );
        render_input_widget_with_title(
            frame,
            self.description_input.borrow(),
            String::from("Description: "),
            chunks[2],
            13,
        );
        self.accept_button.borrow().render(frame, chunks[3]);
    }

    fn on_key_pressed(&mut self, key: KeyEvent) -> Option<MenuEvent<Option<TaskId>>> {

        self.focus_controller.process_input(key.code);

        match key.code {
            KeyCode::Esc => return Some(MenuEvent::Quit(None)),
            KeyCode::Enter => {
                if self.accept_button.borrow_mut().get_focus_state() == FocusState::Focused {
                    let title = self.title_input.borrow_mut().get_current_text();
                    let state = self.state_input.borrow_mut().get_selected_option();
                    let description = self.description_input.borrow_mut().get_current_text();
                    if title.len() > 0 && state != None && description.len() > 0 {
                        let mut logic = self.logic.borrow_mut();
                        let mut _created_task : Option<TaskId> = None;
                        if let Some(parent_task) = self.parent_task {
                            _created_task = logic.task_manager.add_task_with_parent(title, state.unwrap(), description, &parent_task);
                        }
                        else {
                            _created_task = logic.task_manager.add_task(title, state.unwrap(), description);
                        }
                        return Some(MenuEvent::Quit(_created_task));
                    }   
                }
            }
            _ => (),
        };
        None
    }

    fn update(&mut self, elapsed_time: std::time::Duration) {
        self.focus_controller.update(elapsed_time);
    }
}

fn render_input_widget_with_title<T: Widget>(
    frame: &mut Frame<ApplicationBackend>,
    input_widget: Ref<T>,
    title: String,
    area: Rect,
    max_title_width: u16,
) {
    let title_text =
        Paragraph::new(title).style(Style::default().fg(Color::White).bg(Color::Black));
    let line = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
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
