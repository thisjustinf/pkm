// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        title -> Text,
        content -> Text,
        tags -> Text,
        created_at -> Text,
        updated_at -> Nullable<Text>,
        encrypted -> Bool,
    }
}
