use crate::note_repository::NoteRepository;

#[derive(Debug)]
pub struct NoteService {
    repo: NoteRepository,
}

impl NoteService {
    fn new(repo: NoteRepository) -> Self {
        Self { repo }
    }
}

impl Default for NoteService {
    fn default() -> Self {
        let repo: NoteRepository = NoteRepository::default();
        NoteService::new(repo)
    }
}
