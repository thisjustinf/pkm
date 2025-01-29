use std::io;

use ratatui::{prelude::Backend, Frame, Terminal};

use super::NoteService;
use crate::app::App;

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
        while !app.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw_app(app, frame))?;
            self.handle_events(app)?;
        }
        Ok(())
    }

    fn handle_events(&self, app: &App) -> io::Result<()> {
        Ok(())
    }
}

impl Default for AppService {
    fn default() -> Self {
        AppService::new(NoteService::default())
    }
}
