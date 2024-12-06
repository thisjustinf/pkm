use crate::database::establish_connection;
use crate::models::note::{NewNote, Note};
use diesel::sqlite::SqliteConnection;

trait NoteRepository {
    fn fetch_notes(limit: u8, offset: u8) -> Result;
    fn create_note(note: NewNote);
    fn delete_note(note_id: i32);
}

pub struct NoteRepositoryImpl {
    connection: SqliteConnection,
}

pub fn new() -> Self {
    Self {
        connection: establish_connection(),
    }
}

impl NoteRepository for NoteRepositoryImpl {}
