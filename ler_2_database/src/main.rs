// Application imports
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate actix_web;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;
#[macro_use]
extern crate log;

// File imports
use dotenv::dotenv;

// Application libraries
mod db;
mod logger;
mod sample;
mod schema;

fn main() {
    logger::init_logger();
    dotenv().ok();

    // Initializing database connection
    db::init();
    sample::router::create_routes().expect("Could not initialize API endpoints");
}
