use crate::app::ApplicationBackend;
use crate::backend::TaskManager;
use crossterm::event::KeyEvent;
use tui::Frame;
use std::io;


pub mod main_menu;
pub use main_menu::MainMenu;
use crate::app::Application;


pub trait Menu {
    fn render(&mut self, frame: &mut Frame<ApplicationBackend>);
    fn on_key_pressed(&mut self, key: KeyEvent);
}
