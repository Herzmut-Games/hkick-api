use crate::errors::ApiError;
use crate::models::matches::Match;
use crate::models::teams::Team;
use crate::schema::games;
use crate::schema::matches::dsl::*;

use diesel::{prelude::*, SqliteConnection};

#[derive(serde_derive::Deserialize, Queryable)]
pub struct Game {
    pub id: i32,
    pub match_id: i32,
    pub score_team_1: i32,
    pub score_team_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

impl Game {
    fn get_match(&self, conn: &SqliteConnection) -> Result<Match, ApiError> {
        matches
            .find(self.match_id)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find match of game", 404))
    }

    pub fn get_winner_and_loser(
        &self,
        conn: &SqliteConnection,
    ) -> Result<(Team, Team), ApiError> {
        let parent_match: Match = self.get_match(&*conn)?;

        if self.score_team_1 > self.score_team_2 {
            Ok((
                parent_match.get_team_1(&*conn)?,
                parent_match.get_team_2(&*conn)?,
            ))
        } else {
            Ok((
                parent_match.get_team_2(&*conn)?,
                parent_match.get_team_1(&*conn)?,
            ))
        }
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
