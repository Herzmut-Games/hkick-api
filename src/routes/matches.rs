use crate::matchmaking::*;
use crate::schema::matches;
use crate::schema::matches::dsl::*;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1: i32,
    pub team_2: i32,
}

fn get_match_id(new_match: &NewMatch, conn: &SqliteConnection) -> Result<i32, MatchmakingError> {
    match matches
        .filter(team_1.eq(new_match.team_1))
        .filter(team_2.eq(new_match.team_2))
        .select(id)
        .order(id.desc())
        .limit(1)
        .load(conn)
    {
        Ok(match_id) => Ok(*match_id.first().unwrap()),
        Err(_) => Err(MatchmakingError::Fetch),
    }
}

#[post("/", format = "json", data = "<player_ids>")]
pub fn create(conn: DbConn, player_ids: Json<[i32; 4]>) -> Result<JsonValue, Status> {
    let mut req = player_ids.into_inner();
    req.sort();

    let balanced_teams = match find_teams(&*conn, &req) {
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
                MatchmakingError::Fetch => {
                    panic!("Fetch error after find teams -> this should not be happening")
                }
            }
        }
    };

    let new_match = NewMatch {
        team_1: balanced_teams.0.id,
        team_2: balanced_teams.1.id,
    };

    match diesel::insert_into(matches)
        .values(&new_match)
        .execute(&*conn)
        .map_err(|_| MatchmakingError::Create)
        .and_then(|_| get_match_id(&new_match, &*conn))
    {
        Ok(matchid) => Ok(json!(matchid)),
        Err(_) => Err(Status::new(500, "Error creating match")),
    }
}
