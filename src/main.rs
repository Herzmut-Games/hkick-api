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
use self::schema::users::dsl::*;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[get("/")]
fn get(conn: DbConn) -> String {
    users.limit(1).load::<User>(&*conn).expect("WTF");
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![get])
}

fn main() {
    rocket().launch();
}
