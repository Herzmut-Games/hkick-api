use crate::errors::ApiError;
use crate::models::teams::*;
use crate::schema::{matches, teams::dsl::*};

use diesel::{prelude::*, SqliteConnection};

#[derive(serde_derive::Deserialize, Clone, Queryable, serde_derive::Serialize)]
pub struct Match {
    pub id: i32,
    pub team_1: i32,
    pub team_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

impl Match {
    pub fn get_team_1(&self, conn: &SqliteConnection) -> Result<Team, ApiError> {
        teams
            .find(self.team_1)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find team_1 of match", 404))
    }

    pub fn get_team_2(&self, conn: &SqliteConnection) -> Result<Team, ApiError> {
        teams
            .find(self.team_2)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find team_2 of match", 404))
    }
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1: i32,
    pub team_2: i32,
}
