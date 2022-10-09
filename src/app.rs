use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
    rc::Rc,
    cell::RefCell,
};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};

use crate::{
    backend::{TaskManager, Task},
    frontend::{menus::{Menu, MainMenu}},
};

pub type ApplicationBackend = tui::backend::CrosstermBackend<Stdout>;

pub struct Application {
    task_manager : Rc<RefCell<TaskManager>>,
    terminal : Terminal<ApplicationBackend>,
}

impl Application {
    pub fn new() -> Result<Application, io::Error> {
        enable_raw_mode()?;   
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Application {
            task_manager : Rc::new(RefCell::new(TaskManager::new())),
            terminal,
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {    
        {
            let mut task_manager = self.task_manager.borrow_mut();
            task_manager.add_task(Task {
                title: "Task 1".to_string(),
                state: "In process".to_string(),
            });
            task_manager.add_task(Task {
                title: "Task 2".to_string(),
                state: "Backlog".to_string(),
            });
        
            task_manager.add_task(Task {
                title: "Task 3".to_string(),
                state: "Done".to_string(),
            });
        
            task_manager.add_task(Task {
                title: "Task 4".to_string(),
                state: "Done".to_string(),
            });
        
            task_manager.add_task(Task {
                title: "Task 5".to_string(),
                state: "InProcess".to_string(),
            });
        }

        self.execute_menu(Box::new(MainMenu::new(Rc::clone(&self.task_manager))))?;

        //thread::sleep(Duration::from_millis(5000));

        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn execute_menu(&mut self, mut menu : Box<dyn Menu>) -> Result<(), io::Error> {
        let tick_rate = Duration::from_millis(33);
        let mut last_tick = Instant::now();
        loop {
            let mut should_quit = false;
            self.terminal.draw(|frame| {
                menu.render(frame);
            })?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    menu.on_key_pressed(key);
                    match key.code {
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                should_quit = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                //update the app here if needed
                last_tick = Instant::now();
            }
            if should_quit {
                break;
            }
        }
        Ok(())
    }
}
