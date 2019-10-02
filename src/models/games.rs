use crate::{
    errors::ApiError,
    models::matches::*,
    schema::{games, matches::dsl::*},
};

use diesel::{prelude::*, SqliteConnection};

#[derive(serde_derive::Deserialize)]
pub struct Game {
    pub id: i32,
    pub match_id: i32,
    pub score_team_1: i32,
    pub score_team_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

impl Game {
    pub fn get_match(&self, conn: &SqliteConnection) -> Result<Match, ApiError> {
        matches
            .find(self.match_id)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find match of game", 404))
    }
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
