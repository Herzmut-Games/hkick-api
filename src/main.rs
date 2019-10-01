#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket::Rocket;

pub mod matchmaking;
pub mod models;
pub mod routes;
pub mod schema;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![])
        .mount(
            "/players",
            routes![
                routes::players::single_player,
                routes::players::all_players,
                routes::players::create
            ],
        )
        .mount(
            "/matches",
            routes![routes::matches::all_matches, routes::matches::create],
        )
}

fn main() {
    rocket().launch();
}
