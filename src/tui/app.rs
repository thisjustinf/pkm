use diesel::deserialize::Result;

use crate::models::note::Note;

pub struct TuiApp {
    pub notes: Vec<Note>,
    pub selected_note: Option<usize>,
    pub input_mode: InputMode,
}

impl TuiApp {
    pub fn new(notes: Vec<Note>, selected_note: Option<usize>, input_mode: InputMode) -> Self {
        Self {
            notes,
            selected_note,
            input_mode,
        }
    }

    pub fn add_note(title: &str, content: &str, tags: Vec<&str>, encrypted: bool) -> Result<Note> {}
}

pub enum InputMode {
    Normal,
    Editing,
}
