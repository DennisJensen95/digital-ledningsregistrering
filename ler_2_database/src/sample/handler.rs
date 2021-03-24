use std::env;

use diesel::result::Error;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::model::Client;
use crate::sample::model::NewClient;

// File upload
use crate::multipart;
use multipart::mock::StdoutTee;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use multipart::server::Multipart;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::response::Stream;
use rocket::Data;
use std::fs::File;
use std::io::{self, Cursor, Write};

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

#[post("/upload", data = "<data>")]
pub fn client_upload(
    cont_type: &ContentType,
    data: Data,
) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into(),
        ));
    }

    let (_, boundary) = cont_type
        .params()
        .find(|&(k, _)| k == "boundary")
        .ok_or_else(|| {
            Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into(),
            )
        })?;

    match process_upload(boundary, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out)?,
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)?
        }
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, mut out: &mut Vec<u8>) -> io::Result<()> {
    {
        let stdout = io::stdout();
        let tee = StdoutTee::new(&mut out, &stdout);

        entries.write_debug(tee)?;
        debug!("{:?}", entries);
        // entries.fields.get("data");
        // let mut data = entries.fields.get("data").unwrap();
        // io::copy(&mut , &mut tee)?;
    }
    let mut buffer = File::create("test.json")?;
    buffer.write(out)?;
    Ok(())
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
