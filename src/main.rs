use std::io::{self, stdout};

pub mod timer;
pub mod app;
pub mod user_interfaces;
pub mod todo;
pub mod events;

use events::handle_events;
use app::{App, State::{Startup, Display, Exit}};
use ratatui::prelude::*;
use ratatui::crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use user_interfaces::{startup_ui, main_ui, leave};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	let mut app = App::new();
    while !app.should_quit {
		terminal.draw(|f| ui(f, &mut app))?;
        app.should_quit = handle_events(&mut app)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
	match app.state {
		Startup => startup_ui(frame),
        Display => main_ui(frame, app),
		Exit => leave(),
    }
}