#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hex;
extern crate ring;
extern crate byteorder;
extern crate rand;
extern crate num_traits;

pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;
pub mod errors;
pub mod druuid;

#[database("platform_merchant_identity")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::create_token,
                routes::list_users
            ],
        )
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing)
        .launch();
}

