table! {
    events (id) {
        id -> Integer,
        sport -> Text,
        title -> Text,
        country -> Text,
        location -> Text,
        track -> Text,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        name -> Text,
        time -> Nullable<Timestamp>,
        event_id -> Integer,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Text,
        hashed_password -> Text,
    }
}

joinable!(sessions -> events (event_id));

allow_tables_to_appear_in_same_query!(
    events, sessions,
);
