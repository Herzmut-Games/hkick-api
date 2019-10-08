use crate::errors::ApiError;
use crate::schema::players;
use crate::schema::players::dsl::*;

use diesel::prelude::*;
use diesel::SqliteConnection;

#[derive(Clone, Queryable, serde_derive::Serialize)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
    pub rating: i32,
}

impl Player {
    pub fn update_in_db(
        &self,
        conn: &SqliteConnection,
    ) -> Result<usize, ApiError> {
        diesel::update(players.filter(id.eq(self.id)))
            .set(rating.eq(self.rating))
            .execute(conn)
            .map_err(|_| ApiError::new("Could not update player rating", 500))
    }
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
}
