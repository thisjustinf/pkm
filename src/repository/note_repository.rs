use crate::models::note::{NewNote, Note};

trait NoteRepository {
    fn fetch_notes(limit: u8, offset: u8) -> Result;
    fn create_note(note: NewNote);
    fn delete_note(note_id: i32);
}

pub struct NoteRepositoryImpl{
    conn: SqliteConnection
}
