use crate::{
    errors::ApiError,
    models::players::*,
    schema::{players::dsl::*, teams},
};

use diesel::{prelude::*, SqliteConnection};

#[derive(Clone, Copy, Queryable, serde_derive::Serialize)]
pub struct Team {
    pub id: i32,
    pub player_1: i32,
    pub player_2: i32,
    pub rating: i32,
}

impl Team {
    pub fn get_player_1(&self, conn: &SqliteConnection) -> Result<Player, ApiError> {
        players
            .find(self.player_1)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find player_1 of team", 404))
    }

    pub fn get_player_2(&self, conn: &SqliteConnection) -> Result<Player, ApiError> {
        players
            .find(self.player_2)
            .first(conn)
            .map_err(|_| ApiError::new("Could not find player_2 of team", 404))
    }
}

#[derive(serde_derive::Deserialize, Insertable, Debug, PartialEq)]
#[table_name = "teams"]
pub struct NewTeam {
    pub player_1: i32,
    pub player_2: i32,
}

impl NewTeam {
    pub fn new(p_1: i32, p_2: i32) -> NewTeam {
        // By convention the id of player_1 has to be lower than the id of
        // player_2 to ensure that we do not have duplicate teams in the
        // database
        if p_1 < p_2 {
            NewTeam {
                player_1: p_1,
                player_2: p_2,
            }
        } else {
            NewTeam {
                player_1: p_2,
                player_2: p_1,
            }
        }
    }
}
