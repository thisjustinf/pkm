use crate::database::schema::notes;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub encrypted: bool,
}

impl Note {
    pub fn new(
        id: i32,
        title: String,
        content: String,
        tags: String,
        created_at: String,
        updated_at: Option<String>,
        encrypted: bool,
    ) -> Self {
        Self {
            id,
            title,
            content,
            tags,
            created_at,
            updated_at,
            encrypted,
        }
    }
}

#[derive(Insertable)]
#[table_name = "notes"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub tags: &'a str,
    pub encrypted: bool,
}

impl<'a> NewNote<'a> {
    pub fn new(title: &'a str, content: &'a str, tags: &'a str, encrypted: bool) -> Self {
        Self {
            title,
            content,
            tags,
            encrypted,
        }
    }
}
