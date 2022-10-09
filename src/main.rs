mod app;
mod backend;
mod frontend;

use crate::app::{Application};
use std::io;
use std::env;

fn main() -> Result<(), io::Error> {
    env::set_var("RUST_BACKTRACE", "full");
    // setup terminal
    let mut app = Application::new()?;
    app.run()
}