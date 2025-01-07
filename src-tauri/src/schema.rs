// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
