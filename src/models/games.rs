use crate::schema::games;

#[derive(serde_derive::Deserialize)]
pub struct Game {
    pub id: i32,
    pub match_id: i32,
    pub score_team_1: i32,
    pub score_team_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(serde_derive::Deserialize)]
pub struct GameResult {
    pub score_team_1: i32,
    pub score_team_2: i32,
}

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub match_id: i32,
    pub score_team_1: i32,
    pub score_team_2: i32,
}

impl NewGame {
    pub fn new(result: GameResult, match_id: i32) -> NewGame {
        NewGame {
            match_id,
            score_team_1: result.score_team_1,
            score_team_2: result.score_team_2,
        }
    }
}
