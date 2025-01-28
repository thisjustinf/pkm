use crate::database::{schema::notes::dsl::*, DatabasePool};
use crate::models::note::{BaseNoteDTO, Note};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    RunQueryDsl, SqliteConnection,
};

use super::{Repository, RepositoryError};

#[derive(Debug)]
pub struct NoteRepository {
    db: DatabasePool,
}

impl NoteRepository {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }
}

impl Default for NoteRepository {
    fn default() -> Self {
        let db_pool: DatabasePool = DatabasePool::create_pool();
        NoteRepository::new(db_pool)
    }
}

impl Repository<Note, i32, BaseNoteDTO<'_>> for NoteRepository {
    fn get_all(&self, limit: i64) -> Result<Vec<Note>, RepositoryError> {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        notes
            .limit(limit)
            .select(Note::as_select())
            .load(&mut conn)
            .map_err(|_| RepositoryError::ResourceNotFound)
    }

    fn get_by_id(&self, note_id: i32) -> Result<Note, RepositoryError> {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        notes
            .find(note_id)
            .select(Note::as_select())
            .first(&mut conn)
            .map_err(|_| RepositoryError::ResourceNotFound)
    }

    fn create(&self, insertable: &BaseNoteDTO) -> Result<Note, RepositoryError> {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        diesel::insert_into(notes)
            .values(insertable)
            .returning(Note::as_returning())
            .get_result(&mut conn)
            .map_err(|_| RepositoryError::InsertError)
    }

    fn update(&self, note_id: i32, dto: &BaseNoteDTO) -> Result<Note, RepositoryError> {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        if let Ok(_) = self.get_by_id(note_id) {
            let updated_note: Note = diesel::update(notes.find(note_id))
                .set((
                    title.eq(dto.title),
                    content.eq(dto.content),
                    tags.eq(dto.tags),
                ))
                .returning(Note::as_returning())
                .get_result(&mut conn)
                .map_err(|e| RepositoryError::DieselError(e))?;
            Ok(updated_note)
        } else {
            Err(RepositoryError::ResourceNotFound)
        }
    }

    fn delete(&self, note_id: i32) -> Result<bool, RepositoryError> {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .db
            .get_connection()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        if let Ok(_) = self.get_by_id(note_id) {
            let deletions: usize = diesel::delete(notes.find(note_id)).execute(&mut conn)?;
            Ok(deletions == 1)
        } else {
            Err(RepositoryError::ResourceNotFound)
        }
    }
}
