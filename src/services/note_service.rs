use crate::{
    note::{BaseNoteDTO, Note},
    note_repository::NoteRepository,
    repository::Repository,
};

use super::ServiceError;

#[derive(Debug)]
pub struct NoteService {
    repo: NoteRepository,
}

impl NoteService {
    pub fn new(repo: NoteRepository) -> Self {
        Self { repo }
    }

    fn get_notes(&self) -> Vec<Note> {
        self.repo.get_all(10).unwrap_or_default()
    }

    fn create_note(&self, note_dto: BaseNoteDTO) -> Result<Note, ServiceError> {
        match self.repo.create(&note_dto) {
            Ok(n) => Ok(n),
            Err(_) => Err(ServiceError::UnexpectedError),
        }
    }

    fn update_note(&self, id: i32, note_dto: BaseNoteDTO) -> Result<Note, ServiceError> {
        match self.repo.update(id, &note_dto) {
            Ok(n) => Ok(n),
            Err(_) => Err(ServiceError::UnexpectedError),
        }
    }

    fn delete_note(&self, id: i32) -> Result<bool, ServiceError> {
        match self.repo.delete(id) {
            Ok(n) => Ok(n),
            Err(_) => Err(ServiceError::UnexpectedError),
        }
    }
}

impl Default for NoteService {
    fn default() -> Self {
        let repo: NoteRepository = NoteRepository::default();
        NoteService::new(repo)
    }
}
