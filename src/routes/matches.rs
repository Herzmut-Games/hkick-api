use crate::matchmaking::*;
use crate::schema::matches;
use crate::schema::matches::dsl::*;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1: i32,
    pub team_2: i32,
}

#[post("/", format = "json", data = "<player_ids>")]
pub fn create(conn: DbConn, player_ids: Json<[i32; 4]>) -> Result<Status, Status> {
    let mut req = player_ids.into_inner();
    req.sort();

    let t = match find_teams(&*conn, &req) {
        Ok(ts) => ts,
        Err(e) => {
            return match e {
                MatchmakingError::Create => Err(Status::new(500, "Error creating team")),
                MatchmakingError::GetPlayers => {
                    Err(Status::new(500, "Could not get players from database"))
                }
                MatchmakingError::WrongPlayerAmount => {
                    Err(Status::new(500, "Error loading players"))
                }
            }
        }
    };

    let new_match = NewMatch {
        team_1: t.0.id,
        team_2: t.1.id,
    };

    match diesel::insert_into(matches)
        .values(&new_match)
        .execute(&*conn)
    {
        Ok(_) => Ok(Status::new(200, "Match created")),
        Err(_) => Err(Status::new(500, "Error creating match")),
    }
}
