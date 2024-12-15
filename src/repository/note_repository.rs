use crate::database::Database;
use crate::models::note::Note;
use nject::{inject, injectable};

use super::Repository;

#[injectable]
pub struct NoteRepository {
    db: Database,
}

impl NoteRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

// impl Repository<Note, i32> for NoteRepository {
//     fn get_all() {
//         todo!()
//     }

//     fn get_by_id<U>(id: i32) -> Result<Note, E> {
//         todo!()
//     }

//     fn create() {
//         todo!()
//     }

//     fn update<U>(id: i32) {
//         todo!()
//     }

//     fn delete<U>(id: i32) {
//         todo!()
//     }
// }
