use crate::{models::note::Note, AppService};

use super::{AppScreen, InputMode};

#[derive(Debug, Default)]
pub struct App {
    pub notes: Vec<Note>,
    pub current_screen: AppScreen,
    pub selected_note: Option<Note>,
    pub input_mode: InputMode,
    pub app_service: AppService,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: vec![],
            current_screen: AppScreen::Main,
            selected_note: None,
            input_mode: InputMode::Normal,
            app_service: AppService::default(),
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true
    }
}
