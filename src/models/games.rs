use crate::schema::games as schema_games;

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Queryable)]
pub struct Game {
    pub id: i32,
    pub match_id: Option<i32>,
    pub duel_id: Option<i32>,
    pub score_1: i32,
    pub score_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(serde_derive::Deserialize)]
pub struct GameResult {
    pub score_1: i32,
    pub score_2: i32,
}

#[derive(Insertable)]
#[table_name = "schema_games"]
pub struct NewGame {
    pub match_id: Option<i32>,
    pub duel_id: Option<i32>,
    pub score_1: i32,
    pub score_2: i32,
}

impl NewGame {
    pub fn new(
        result: GameResult,
        match_id: Option<i32>,
        duel_id: Option<i32>,
    ) -> NewGame {
        NewGame {
            match_id,
            duel_id,
            score_1: result.score_1,
            score_2: result.score_2,
        }
    }
}
