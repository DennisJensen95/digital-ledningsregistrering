// Third party imports
use actix_web::{web, App, HttpServer};

// This application imports
use crate::sample::handler;

const PORT: &str = "8000";

#[actix_rt::main]
pub async fn create_routes() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/clients")
                .route("/hi", web::get().to(handler::index))
                .route("/files/upload/{name}", web::post().to(handler::upload))
                .route("/files/download/{name}", web::get().to(handler::download))
                .route("", web::get().to(handler::all_clients))
                .route("/new", web::post().to(handler::create_client))
                .route("/update/{id}", web::post().to(handler::update_client))
                .route("/delete/{id}", web::post().to(handler::delete_client)),
        )
    })
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
