use crate::models::*;
use crate::schema::players::dsl::*;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::JsonValue;

#[get("/")]
pub fn all_players(conn: DbConn) -> JsonValue {
    json!(players.load::<Player>(&*conn).unwrap())
}

#[get("/<player_id>")]
pub fn single_player(conn: DbConn, player_id: i32) -> Result<JsonValue, Status> {
    let p = players.find(player_id).load::<Player>(&*conn).unwrap();

    match p.len() {
        1 => Ok(json!(p.first())),
        _ => Err(Status::new(404, "Player not found")),
    }
}
