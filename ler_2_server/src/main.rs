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
    let endpoints = ler_endpoints::LEREndpoints::default(ler_endpoints::LER_API_TEST);
    client::punch_through(&endpoints).await;
    client::secure_punch_through(&endpoints).await;
}
