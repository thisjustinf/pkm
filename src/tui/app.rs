use std::io;

use ratatui::{DefaultTerminal, Frame};

use crate::{models::note::Note, AppService};

use super::InputMode;

#[derive(Debug, Default)]
pub struct App {
    pub notes: Vec<Note>,
    pub selected_note: Option<Note>,
    pub input_mode: InputMode,
    pub app_service: AppService,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: vec![],
            selected_note: None,
            input_mode: InputMode::Normal,
            app_service: AppService::default(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }
}
