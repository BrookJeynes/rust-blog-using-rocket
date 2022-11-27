// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        genre -> Varchar,
        published -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_name -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
