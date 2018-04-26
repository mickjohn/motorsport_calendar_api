// table! {
//     datetest (id) {
//         id -> BigInt,
//         time -> Nullable<Timestamp>,
//     }
// }

table! {
    events (id) {
        id -> Integer,
        sport -> Text,
        round -> Integer,
        country -> Text,
        location -> Text,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        name -> Text,
        date -> Nullable<Timestamp>,
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
    // datetest,
    events,
    sessions,
);
