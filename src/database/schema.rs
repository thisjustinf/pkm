// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        tags -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
