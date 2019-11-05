use crate::errors::ApiError;
use crate::models::{games::Game, players::Player};
use crate::schema::duels as duels_table;
use crate::schema::games::dsl::{duel_id, games as table_games};
use crate::schema::players::dsl::players as table_players;

use diesel::{prelude::*, SqliteConnection};

#[derive(
    serde_derive::Serialize, serde_derive::Deserialize, Clone, Queryable,
)]
pub struct Duel {
    pub id: i32,
    pub player_1: i32,
    pub player_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

impl Duel {
    pub fn get_player_1(
        &self,
        conn: &SqliteConnection,
    ) -> Result<Player, ApiError> {
        table_players
            .find(self.player_1)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find player_1 of match", 404))
    }

    pub fn get_player_2(
        &self,
        conn: &SqliteConnection,
    ) -> Result<Player, ApiError> {
        table_players
            .find(self.player_2)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find player_2 of match", 404))
    }

    pub fn get_games(
        &self,
        conn: &SqliteConnection,
    ) -> Result<Vec<Game>, ApiError> {
        table_games
            .filter(duel_id.eq(self.id))
            .get_results(conn)
            .map_err(|_| ApiError::new("Could not get games for match", 404))
    }
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "duels_table"]
pub struct NewDuel {
    pub player_1: i32,
    pub player_2: i32,
}
