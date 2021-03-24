#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate multipart;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;
#[macro_use]
extern crate log;

use dotenv::dotenv;

mod connection;
mod logger;
mod sample;
mod schema;

fn main() {
    logger::init_logger();
    dotenv().ok();
    sample::router::create_routes();
}
