use crate::errors::ApiError;
use crate::matchmaking::*;
use crate::models::matches::*;
use crate::schema::matches::dsl::*;
use crate::DbConn;

use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

fn get_match_id(
    new_match: &NewMatch,
    conn: &SqliteConnection,
) -> Result<i32, ApiError> {
    match matches
        .filter(team_1.eq(new_match.team_1))
        .filter(team_2.eq(new_match.team_2))
        .select(id)
        .order(id.desc())
        .limit(1)
        .load(conn)
    {
        Ok(match_id) => Ok(*match_id.first().unwrap()),
        Err(_) => Err(ApiError::new("Could not find match", 404)),
    }
}

#[post("/", format = "json", data = "<player_ids>")]
pub fn create(
    conn: DbConn,
    player_ids: Json<[i32; 4]>,
) -> Result<JsonValue, ApiError> {
    let mut req = player_ids.into_inner();
    req.sort();

    let balanced_teams = find_teams(&*conn, &req)?;
    let new_match = NewMatch {
        team_1: balanced_teams.0.id,
        team_2: balanced_teams.1.id,
    };

    match diesel::insert_into(matches)
        .values(&new_match)
        .execute(&*conn)
        .map_err(|_| ApiError::new("Could not create match", 500))
        .and_then(|_| get_match_id(&new_match, &*conn))
    {
        Ok(matchid) => Ok(json!({ "id": matchid })),
        Err(e) => Err(e),
    }
}

#[get("/")]
pub fn all_matches(conn: DbConn) -> JsonValue {
    json!(matches.load::<Match>(&*conn).unwrap())
}
