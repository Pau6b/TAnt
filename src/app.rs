use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    backend::{Task, TaskManager},
    frontend::{main_menu::render_main_menu, UIState},
};

pub fn run() -> Result<(), io::Error> {
    let tick_rate = Duration::from_millis(33);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut last_tick = Instant::now();
    let mut should_quit = false;

    let mut task_manager = TaskManager::new();
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

    let mut ui_state = UIState::new(&task_manager);

    loop {
        terminal.draw(|frame| render_main_menu(frame, &mut task_manager, &mut ui_state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        if c == 'q' {
                            should_quit = true;
                        }
                    },
                    KeyCode::Up => ui_state.on_up(),
                    KeyCode::Down => ui_state.on_down(),
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

    //thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
