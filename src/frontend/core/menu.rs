use crate::app::ApplicationBackend;
use crate::frontend::core::UIContext;
use crossterm::event::KeyEvent;
use std::{cell::RefCell, io, rc::Rc, time::Duration};
use tui::Frame;

pub enum MenuEvent<T> {
    Quit(T),
    MenuExecutionResult(Result<(), io::Error>),
}

pub trait Menu<T> {
    fn initialize(&mut self, ui_context: Rc<RefCell<UIContext>>);
    fn render(&mut self, frame: &mut Frame<ApplicationBackend>);
    fn update(&mut self, elapsed_time: Duration);
    fn on_key_pressed(&mut self, key: KeyEvent) -> Option<MenuEvent<T>>;
}
