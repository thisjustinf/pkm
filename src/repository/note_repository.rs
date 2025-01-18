use crate::database::{schema::notes, DatabasePool};
use crate::models::note::{NewNote, Note};
use diesel::{
    query_dsl::methods::FindDsl,
    r2d2::{ConnectionManager, PooledConnection},
    RunQueryDsl, SqliteConnection,
};
// use diesel::RunQueryDsl;
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

impl Repository<Note, i32> for NoteRepository {
    fn get_all(&self) -> Result<Note, RepositoryError> {
        todo!()
    }

    fn get_by_id(&self, id: i32) -> Result<Note, RepositoryError> {
        let conn: PooledConnection<ConnectionManager<SqliteConnection>> =
            self.db.get_connection().unwrap();
        let note = notes::table
            .find(id)
            .first(&mut conn)
            .map_err(|e| -> RepositoryError {
                match e {
                    diesel::result::Error::NotFound => RepositoryError::ResourceNotFound,
                    e => RepositoryError::Database(e),
                }
            });
    }

    fn create<NewNote>(&self, insertable: &NewNote) -> Result<Note, RepositoryError> {
        let conn: PooledConnection<ConnectionManager<SqliteConnection>> =
            self.db.get_connection().unwrap();
        let note: Result<usize, RepositoryError> = diesel::insert_into(notes::table)
            .values(insertable)
            .execute(&mut conn);
    }

    fn update(&self, id: i32) -> Result<Note, RepositoryError> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<Note, RepositoryError> {
        todo!()
    }
}
