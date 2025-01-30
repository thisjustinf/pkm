#[derive(Debug)]
pub enum AppScreen {
    Main,
    NoteEditor,
    NotesList,
    Exit,
}

impl Default for AppScreen {
    fn default() -> Self {
        AppScreen::Main
    }
}
