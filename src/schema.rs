table! {
    accounts (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

table! {
    fileentity (file_id) {
        file_id -> Int4,
        filepath -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    fileentity,
);
