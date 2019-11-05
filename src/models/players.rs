use crate::errors::ApiError;
use crate::schema::players as schema_players;
use crate::schema::players::dsl::{
    id as col_id, players as table_players, solo_rating as col_solo_rating,
    team_rating as col_team_rating,
};

use diesel::prelude::*;
use diesel::SqliteConnection;

#[derive(Clone, Queryable, serde_derive::Serialize)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
    pub team_rating: i32,
    pub solo_rating: i32,
}

impl Player {
    pub fn update_solo_rating(
        &self,
        conn: &SqliteConnection,
    ) -> Result<usize, ApiError> {
        diesel::update(table_players.filter(col_id.eq(self.id)))
            .set(col_solo_rating.eq(self.solo_rating))
            .execute(conn)
            .map_err(|_| {
                ApiError::new("Could not update player solo_rating", 500)
            })
    }

    pub fn update_team_rating(
        &self,
        conn: &SqliteConnection,
    ) -> Result<usize, ApiError> {
        diesel::update(table_players.filter(col_id.eq(self.id)))
            .set(col_team_rating.eq(self.team_rating))
            .execute(conn)
            .map_err(|_| {
                ApiError::new("Could not update player team_rating", 500)
            })
    }
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "schema_players"]
pub struct NewPlayer {
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
}
