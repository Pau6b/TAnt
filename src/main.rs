mod app;
mod backend;
mod frontend;


use crate::app::{Application};
use std::{io, env, rc::Rc, cell::RefCell};

fn main() -> Result<(), io::Error> {
    env::set_var("RUST_BACKTRACE", "full");
    // setup terminal
    let app = Rc::new(RefCell::new(Application::new()?));
    app.borrow_mut().run()?;
    Ok(())
}