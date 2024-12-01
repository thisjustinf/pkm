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
}

pub enum InputMode {
    Normal,
    Editing,
}
