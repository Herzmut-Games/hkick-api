#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::players::dsl::*;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[get("/players")]
fn all_players(conn: DbConn) -> JsonValue {
    json!(players.load::<Player>(&*conn).unwrap())
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![all_players])
}

fn main() {
    rocket().launch();
}
