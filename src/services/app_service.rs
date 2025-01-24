use super::NoteService;

#[derive(Debug)]
pub struct AppService {
    note_service: NoteService,
}

impl AppService {
    pub fn new(note_service: NoteService) -> Self {
        Self { note_service }
    }
}

impl Default for AppService {
    fn default() -> Self {
        AppService::new(NoteService::default())
    }
}
