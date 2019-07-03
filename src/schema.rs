table! {
    stagings (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    users_stagings (id) {
        id -> Int4,
        user_id -> Int4,
        staging_id -> Int4,
    }
}

joinable!(users_stagings -> stagings (staging_id));
joinable!(users_stagings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    stagings,
    users,
    users_stagings,
);
