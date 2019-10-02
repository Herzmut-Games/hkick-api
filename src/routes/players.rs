use crate::errors::ApiError;
use crate::models::players::*;
use crate::schema::players::dsl::*;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn all_players(conn: DbConn) -> JsonValue {
    json!(players.load::<Player>(&*conn).unwrap())
}

#[get("/<player_id>")]
pub fn single_player(conn: DbConn, player_id: i32) -> Result<JsonValue, ApiError> {
    let p = players.find(player_id).load::<Player>(&*conn).unwrap();

    match p.len() {
        1 => Ok(json!(p.first())),
        _ => Err(ApiError::new("Player not found", 404)),
    }
}

#[post("/", format = "json", data = "<player_data>")]
pub fn create(conn: DbConn, player_data: Json<NewPlayer>) -> Result<Status, ApiError> {
    match diesel::insert_into(players)
        .values(&player_data.into_inner())
        .execute(&*conn)
    {
        Ok(_) => Ok(Status::new(200, "User created")),
        Err(_) => Err(ApiError::new("Could not create player", 500)),
    }
}
