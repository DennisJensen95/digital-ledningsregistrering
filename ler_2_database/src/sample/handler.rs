use actix_multipart::Multipart;
use actix_web::Error as ACTIX_ERROR;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Datelike, Timelike};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use std::env;

use diesel::result::Error;

// use crate::deprecated_connection::DbConn;
use crate::sample;
use crate::sample::model::Client;
use crate::sample::model::NewClient;

const UPLOAD_PATH: &str = "/opt/ler-2-service/data/";
#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String,
}

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn index() -> impl Responder {
    debug!("Hi");
    HttpResponse::Ok().body("Hello sunshine!")
}

// pub fn all_clients(connection: DbConn) -> Result<Json<Vec<Client>>, Status> {
//     sample::repository::show_clients(&connection)
//         .map(|client| Json(client))
//         .map_err(|error| error_status(error))
// }

pub async fn upload(
    mut payload: Multipart,
    info: web::Path<User>,
) -> Result<HttpResponse, ACTIX_ERROR> {
    debug!("Uploading data file from client");
    // iterate over multipart stream

    let mut filename = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let user_dir = format!("{}{}", UPLOAD_PATH, info.name.to_string());
        fs::create_dir_all(&user_dir)?;
        let timestamp = chrono::offset::Utc::now();
        let timestamp_name = format!(
            "{}_{}_{}_{}:{}",
            timestamp.year(),
            timestamp.month(),
            timestamp.day(),
            timestamp.hour(),
            timestamp.minute()
        );
        filename = format!(
            "{}_{}",
            timestamp_name,
            content_type.get_filename().unwrap(),
        );
        let filepath = format!("{}/{}", &user_dir, sanitize_filename::sanitize(&filename));
        // File::create is blocking operation, use thread pool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use thread pool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().json(&File {
        name: filename,
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        err: "".to_string(),
    }))
}

pub async fn download(info: web::Path<User>) -> HttpResponse {
    debug!("Downloading from server");
    let path = format!("{}/{}", UPLOAD_PATH, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            err: "file does not exists".to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .header(
            "Content-Disposition",
            format!("form-data; filename={}", info.name.to_string()),
        )
        .body(data)
}
