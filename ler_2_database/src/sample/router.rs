use crate::connection;
use crate::sample;
use rocket;
pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/clients",
            routes![
                sample::handler::all_clients,
                sample::handler::create_client,
                sample::handler::get_client,
                sample::handler::update_client,
                sample::handler::delete_client
            ],
        )
        .launch();
}
