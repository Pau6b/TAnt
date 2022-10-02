mod app;
mod backend;
mod frontend;

use crate::app::run;
use std::io;

fn main() -> Result<(), io::Error> {
    // setup terminal
    run()
}