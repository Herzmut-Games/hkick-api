#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use diesel::prelude::*;
use rocket::http::Method;
use rocket::Rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

pub mod elo;
pub mod errors;
pub mod matchmaking;
pub mod models;
pub mod rating;
pub mod routes;
pub mod schema;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn prepare_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // 8100 is the default port of hkick-admin
        "http://localhost:8100",
        "http://localhost:8080",
        "http://localhost:8000",
        "https://hkick-admin.marv.sexy",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error building CORS")
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(prepare_cors())
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
