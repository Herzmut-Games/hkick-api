#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use diesel::prelude::*;
use rocket::Rocket;

pub mod elo;
pub mod errors;
pub mod matchmaking;
pub mod models;
pub mod rating;
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
                routes::players::create,
                routes::players::single_player,
                routes::players::all_players,
            ],
        )
        .mount(
            "/matches",
            routes![
                routes::matches::create,
                routes::matches::get_all,
                routes::matches::get_by_id,
                routes::games::match_get_all,
                routes::games::match_place_game,
            ],
        )
        .mount(
            "/duels",
            routes![
                routes::duels::create,
                routes::duels::get_all,
                routes::duels::get_by_id,
                routes::games::duel_get_all,
                routes::games::duel_place_game,
            ],
        )
}

fn main() {
    rocket().launch();
}
