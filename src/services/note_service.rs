use super::Service;
use nject::{inject, injectable};

#[injectable]
#[inject(Self::new(repo))]
pub struct NoteService {
    repo: NoteRepository,
}

impl NoteService {
    fn new(repo: NoteRepository) -> Self {
        Self { repo }
    }
}

impl Service<Note> for NoteService {}
