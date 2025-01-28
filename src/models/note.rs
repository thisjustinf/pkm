use crate::database::schema::notes;
use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::{self, Text},
    sqlite::{Sqlite, SqliteValue},
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, PartialEq, Debug)]
#[diesel(table_name = notes)]
#[diesel(check_for_backend(Sqlite))]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: JsonTags, // JSON encoded string
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Note {
    pub fn new(
        id: i32,
        title: String,
        content: String,
        tags: JsonTags,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            title,
            content,
            tags,
            created_at,
            updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
#[diesel(check_for_backend(Sqlite))]
pub struct BaseNoteDTO<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub tags: &'a str, // JSON encoded string
}

impl<'a> BaseNoteDTO<'a> {
    pub fn new(title: &'a str, content: &'a str, tags: &'a str) -> Self {
        Self {
            title,
            content,
            tags,
        }
    }
}

// Diesel-compatible custom type for tags
#[derive(Serialize, Deserialize, Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = sql_types::Text)]
pub struct JsonTags(pub Vec<String>);

impl PartialEq for JsonTags {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToSql<Text, Sqlite> for JsonTags {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json: String = serde_json::to_string(&self.0)?;
        out.set_value(json);
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for JsonTags {
    fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
        let sql_str: String = <String as deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let tags: Vec<String> = serde_json::from_str(&sql_str)?;
        Ok(JsonTags(tags))
    }
}
