use crate::app::ApplicationBackend;
use tui::Frame;
use crossterm::event::KeyEvent;
use std::{rc::Rc, cell::RefCell, io};
use crate::frontend::core::UIContext;

pub enum MenuEvent {
    Quit,
    MenuExecutionResult(Result<(), io::Error>),
}


pub trait Menu {
    fn initialize(&mut self, ui_context: Rc<RefCell<UIContext>>);
    fn render(&mut self, frame: &mut Frame<ApplicationBackend>);
    fn on_key_pressed(&mut self, key: KeyEvent) -> Option<MenuEvent>;
}