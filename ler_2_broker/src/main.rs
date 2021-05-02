// Application libraries
mod client;
mod logger;

mod ler_endpoints;
// Third party libraries
#[macro_use]
extern crate log;
extern crate env_logger;

#[actix_web::main]
async fn main() {
    logger::init_logger();
    let endpoints = ler_endpoints::LEREndpoints::default(ler_endpoints::LER_API_PRODUCTION);
    let status: bool = client::punch_through(&endpoints).await;
    debug!("{:?}", status);
    client::secure_punch_through(&endpoints).await;
}
