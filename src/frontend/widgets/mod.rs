
pub mod input_widget;
pub use input_widget::{InputWidget};

pub mod bottom_bar;
pub use bottom_bar::{BottomBar, BottomBarAction};

pub mod multiple_option_widget;
pub use multiple_option_widget::MultipleOptionWidget;

pub mod button;
pub use button::Button;

use tui::{Frame, layout::Rect};
use super::core::ApplicationBackend;

pub trait Widget {
    fn render(&self, frame: &mut Frame<ApplicationBackend>, area: Rect);
}

#[derive(PartialEq, Clone, Copy)]
pub enum FocusState {
    Focused,
    NotFocused,
}

pub trait FocusableWidget {
    fn focus_state_changed(&mut self, focus_state: FocusState);
    fn get_focus_state(&self) -> FocusState;
}