#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket::Rocket;
use rocket_contrib::databases::diesel::SqliteConnection;

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::teams::dsl::*;
use self::schema::users::dsl::*;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[get("/user")]
fn user(conn: DbConn) -> String {
    let u: User = users.find(1).first::<User>(&*conn).unwrap();

    u.surname
}

#[get("/team")]
fn team(conn: DbConn) -> String {
    let t: Team = teams.find((1, 2)).first::<Team>(&*conn).unwrap();

    t.rating.to_string()
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![user, team])
}

fn main() {
    rocket().launch();
}
