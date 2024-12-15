
use super::Service;

pub struct NoteService {
    repo: NoteRepository
}

impl Service<Note> for NoteService {}
