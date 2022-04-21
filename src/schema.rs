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