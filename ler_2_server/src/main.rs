// Application libraries
mod client;
mod logger;

mod ler_endpoints;
// Third party libraries
#[macro_use]
extern crate log;
extern crate env_logger;

#[tokio::main]
async fn main() {
    logger::init_logger();
    debug!("Succesfully initalized logger");
    client::punch_through().await;
}
