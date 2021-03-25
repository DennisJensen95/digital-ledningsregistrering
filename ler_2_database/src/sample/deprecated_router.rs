use crate::connection;
use crate::sample;
use rocket;
pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/clients",
            routes![
                sample::deprecated_handler::all_clients,
                sample::deprecated_handler::create_client,
                sample::deprecated_handler::get_client,
                sample::deprecated_handler::update_client,
                sample::deprecated_handler::delete_client,
                sample::deprecated_handler::client_upload
            ],
        )
        .launch();
}
