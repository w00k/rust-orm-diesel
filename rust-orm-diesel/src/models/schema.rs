// @generated automatically by Diesel CLI.

diesel::table! {
    countries (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
        capital -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        code_country -> Varchar,
        number -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    countries,
    users,
);

