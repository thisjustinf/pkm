use crate::database::{schema::notes::dsl::*, DatabasePool};
use crate::models::note::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    RunQueryDsl, SqliteConnection,
};
use nject::{inject, injectable};

use super::{Repository, RepositoryError};

#[injectable]
#[inject(Self::new(db))]
pub struct NoteRepository {
    db: DatabasePool,
}

impl NoteRepository {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }
}

impl Repository<Note, u32> for NoteRepository {
    fn get_all(&self) -> Result<Vec<Note>, RepositoryError> {
        todo!()
    }

    fn get_by_id(&self, note_id: u32) -> Result<Note, RepositoryError> {
        let conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        let note: Result<Note, RepositoryError> = notes
            .find(note_id)
            .select(Note::as_select())
            .first(&mut conn)
            .map_err(|e| RepositoryError::ResourceNotFound);
        note
    }

    fn create<CreateNoteDTO>(&self, insertable: &CreateNoteDTO) -> Result<Note, RepositoryError> {
        let conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        let note: Result<Note, RepositoryError> = diesel::insert_into(notes)
            .values(insertable)
            .returning(Note::as_returning())
            .get_result(&mut conn)
            .map_err(|e| RepositoryError::DatabaseError(e));
        note
    }

    fn update<BaseNoteDTO>(
        &self,
        note_id: u32,
        dto: &BaseNoteDTO,
    ) -> Result<Note, RepositoryError> {
        let conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        if let Ok(n) = self.get_by_id(note_id) {
            let updated_note = diesel::update(notes.find(note_id))
                .set((
                    title.eq(*dto.title),
                    content.eq(*dto.content),
                    tags.eq(*dto.tags),
                ))
                .returning(Note::as_returning())
                .get_result(&mut conn)?;
            Ok(updated_note)
        } else {
            Err(RepositoryError::ResourceNotFound)
        }
    }

    fn delete(&self, note_id: u32) -> Result<bool, RepositoryError> {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        if let Ok(n) = self.get_by_id(note_id) {
            let deletions: usize = diesel::delete(notes.find(note_id)).execute(&mut conn)?;
            Ok(deletions == 1)
        } else {
            Err(RepositoryError::ResourceNotFound)
        }
    }
}
