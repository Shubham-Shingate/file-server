table! {
    accounts (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

table! {
    accounts_file_mapping (mapping_id) {
        mapping_id -> Int4,
        user_id -> Int4,
        file_id -> Int4,
        permissions -> Varchar,
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
    accounts_file_mapping,
    fileentity,
);
