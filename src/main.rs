#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket::http::Status;
use rocket::Rocket;
use rocket_contrib::json::JsonValue;

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::players::dsl::*;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[get("/")]
fn all_players(conn: DbConn) -> JsonValue {
    json!(players.load::<Player>(&*conn).unwrap())
}

#[get("/<player_id>")]
fn single_player(conn: DbConn, player_id: i32) -> Result<JsonValue, Status> {
    let p = players.find(player_id).load::<Player>(&*conn).unwrap();

    match p.len() {
        1 => Ok(json!(p.first())),
        _ => Err(Status::new(404, "Player not found")),
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![])
        .mount("/players", routes![single_player, all_players])
}

fn main() {
    rocket().launch();
}
