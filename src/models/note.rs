use std::io::Write;

use crate::database::schema::notes;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sqlite::{Sqlite, SqliteValue},
    Insertable, Queryable,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = notes)]
#[diesel(check_for_backend(Sqlite))]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>, // JSON encoded string
    pub created_at: String,
    pub updated_at: Option<String>,
    pub encrypted: bool,
}

impl Note {
    pub fn new(
        id: i32,
        title: String,
        content: String,
        tags: Vec<String>,
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
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub tags: &'a str, // JSON encoded string
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

// Diesel-compatible custom type for tags
#[derive(Serialize, Deserialize, Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct JsonTags(pub Vec<String>);

impl ToSql<diesel::sql_types::Text, Sqlite> for JsonTags {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let json =
            serde_json::to_string(&self.0).map_err(|_| serialize::Error::SerializationError)?;
        out.write_all(json.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for JsonTags {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let bytes: &[u8] = bytes.ok_or_else(|| deserialize::Error::UnexpectedNull)?;
        let tags: Vec<String> =
            serde_json::from_slice(bytes).map_err(|_| deserialize::Error::DeserializationError)?;
        Ok(JsonTags(tags))
    }
}
