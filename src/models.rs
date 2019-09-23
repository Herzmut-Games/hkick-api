#[derive(Clone, Copy, Queryable, serde_derive::Serialize)]
pub struct Team {
    pub id: i32,
    pub player_1: i32,
    pub player_2: i32,
    pub rating: f32,
}

#[derive(Clone, Queryable, serde_derive::Serialize)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
    pub rating: f32,
}

pub struct Match {
    pub id: i32,
    pub team_1: i32,
    pub team_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

pub struct Game {
    pub id: i32,
    pub match_id: i32,
    pub score_team_1: i16,
    pub score_team_2: i16,
    pub timestamp: chrono::NaiveDateTime,
}
