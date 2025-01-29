use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pkm::{
    tui::{App, InputMode},
    AppService,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stderr: io::Stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let app_service: AppService = AppService::default();
    let backend: CrosstermBackend<io::Stderr> = CrosstermBackend::new(stderr);
    let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    let mut app: App = App::default();
    let res: Result<(), io::Error> = app_service.run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
