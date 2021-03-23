use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::model::Client;
use crate::sample::model::NewClient;

#[get("/")]
pub fn all_clients(connection: DbConn) -> Result<Json<Vec<Client>>, Status> {
    sample::repository::show_clients(&connection)
        .map(|client| Json(client))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_client>")]
pub fn create_client(
    new_client: Json<NewClient>,
    connection: DbConn,
) -> Result<status::Created<Json<Client>>, Status> {
    println!("here 0 {}", &new_client.user_name);
    sample::repository::create_client(new_client.into_inner(), &connection)
        .map(|client| client_created(client))
        .map_err(|error| error_status(error))
}

// #[post("/data"), data="<data>"]

#[get("/<id>")]
pub fn get_client(id: i32, connection: DbConn) -> Result<Json<Client>, Status> {
    sample::repository::get_client(id, &connection)
        .map(|client| Json(client))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<client>")]
pub fn update_client(
    id: i32,
    client: Json<Client>,
    connection: DbConn,
) -> Result<Json<Client>, Status> {
    sample::repository::update_client(id, client.into_inner(), &connection)
        .map(|client| Json(client))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_client(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sample::repository::delete_client(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn client_created(client: Client) -> status::Created<Json<Client>> {
    println!("here final");
    status::Created(
        format!(
            "{host}:{port}/client/{id}",
            host = host(),
            port = port(),
            id = client.id
        )
        .to_string(),
        Some(Json(client)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
