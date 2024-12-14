use crate::database::establish_connection;
use crate::models::note::{NewNote, Note};
use diesel::sqlite::SqliteConnection;
use nject::{injectable, provider};


#[injectable]
pub struct NoteRepository {
    connection: SqliteConnection,
}

pub fn new() -> Self {
    Self {
        connection: establish_connection(),
    }
}

impl Repository<Note, i32> for NoteRepository {
    fn create_note(note: NewNote) -> Result {

    }
}
