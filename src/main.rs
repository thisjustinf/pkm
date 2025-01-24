use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pkm::tui::{layout::draw_ui, InputMode, TuiApp};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stderr: io::Stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stderr> = CrosstermBackend::new(stderr);
    let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    let mut app: TuiApp = TuiApp::default();

    loop {
        terminal.draw(|f: &mut ratatui::Frame<'_>| draw_ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => app.input_mode = InputMode::Editing,
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.input_mode = InputMode::Normal,
                    _ => {}
                },
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {}
