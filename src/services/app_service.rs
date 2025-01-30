use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::Backend, Frame, Terminal};

use super::NoteService;
use crate::{app::App, AppScreen, Note};

#[derive(Debug)]
pub struct AppService {
    note_service: NoteService,
}

impl AppService {
    pub fn new(note_service: NoteService) -> Self {
        Self { note_service }
    }

    pub fn draw_app(&self, app: &App, frame: &mut Frame) {
        frame.render_widget(app, frame.area());
    }

    pub fn run_app<B: Backend>(&self, terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
        let db_notes: Vec<Note> = self.note_service.get_notes();
        if db_notes.len() > 0 {
            app.notes = db_notes
        }
        while !app.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw_app(app, frame))?;
            self.handle_events(app)?;
        }
        Ok(())
    }

    fn handle_events(&self, app: &mut App) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                AppScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = AppScreen::Exit;
                    }
                    _ => {}
                },
                AppScreen::NoteEditor => match key.code {
                    _ => {}
                },
                AppScreen::NotesList => match key.code {
                    _ => {}
                },
                AppScreen::Exit => match key.code {
                    KeyCode::Char('y') => {
                        app.exit();
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}

impl Default for AppService {
    fn default() -> Self {
        AppService::new(NoteService::default())
    }
}
