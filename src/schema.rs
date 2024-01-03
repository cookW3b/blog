// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        name -> Varchar,
        password -> Text,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
