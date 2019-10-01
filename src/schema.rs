table! {
    games (id) {
        id -> Integer,
        match_id -> Integer,
        score_team_1 -> Integer,
        score_team_2 -> Integer,
        timestamp -> Timestamp,
    }
}

table! {
    matches (id) {
        id -> Integer,
        team_1 -> Integer,
        team_2 -> Integer,
        timestamp -> Timestamp,
    }
}

table! {
    players (id) {
        id -> Integer,
        first_name -> Text,
        surname -> Text,
        nickname -> Text,
        rating -> Integer,
    }
}

table! {
    teams (id) {
        id -> Integer,
        player_1 -> Integer,
        player_2 -> Integer,
        rating -> Integer,
    }
}

joinable!(games -> matches (match_id));

allow_tables_to_appear_in_same_query!(
    games,
    matches,
    players,
    teams,
);
