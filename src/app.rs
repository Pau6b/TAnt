use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    cell::RefCell,
    io::{self, Stdout},
    rc::Rc,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    frontend::{
        core::{Logic, Menu, MenuEvent, UIContext},
        menus::MainMenu,
    },
};

pub type ApplicationBackend = tui::backend::CrosstermBackend<Stdout>;

pub struct Application {
    logic: Rc<RefCell<Logic>>,
}

impl Application {
    pub fn new() -> Result<Application, io::Error> {
        Ok(Application {
            logic: Rc::new(RefCell::new(Logic::new())),
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {

        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let ui_context = Rc::new(RefCell::new(UIContext { terminal }));

        let mut main_menu: Box<dyn Menu<()>> = Box::new(MainMenu::new(Rc::clone(&self.logic)));
        execute_menu(&mut main_menu, Rc::clone(&ui_context))?;

        let mut bui_context = ui_context.borrow_mut();

        // restore terminal
        disable_raw_mode()?;
        execute!(
            bui_context.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        bui_context.terminal.show_cursor()?;
        Ok(())
    }
}

pub fn execute_menu<T>(
    menu: &mut Box<dyn Menu<T>>,
    ui_context: Rc<RefCell<UIContext>>,
) -> Result<T, io::Error> {
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();
    menu.initialize(Rc::clone(&ui_context));
    loop {
        {
            ui_context.borrow_mut().terminal.draw(|frame| {
                menu.render(frame);
            })?;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let result = menu.on_key_pressed(key);
                if let Some(r) = result {
                    match r {
                        MenuEvent::Quit(result) => return Ok(result),
                        MenuEvent::MenuExecutionResult(r) => {
                            r?
                        },
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            menu.update(last_tick.elapsed());
            //update the app here if needed
            last_tick = Instant::now();
        }
    }
}
