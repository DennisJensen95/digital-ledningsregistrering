#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
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
    sample::router::create_routes().expect("Test");
}
