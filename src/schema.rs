table! {
    teams (player1, player2) {
        player1 -> Integer,
        player2 -> Integer,
        rating -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        surname -> Text,
        rating -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
