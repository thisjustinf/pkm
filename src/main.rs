use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pkm::tui::{layout::draw_ui, InputMode, TuiApp};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), io::Error> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let mut app: TuiApp = TuiApp::new(vec![], None, InputMode::Normal);
    // let mut app: TuiApp = TuiApp {
    //     notes: vec![], // Load notes from the database
    //     selected_note: None,
    //     input_mode: InputMode::Normal,
    // };

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
