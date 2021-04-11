// Third party imports
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};

// This application imports
use crate::sample::handler;

const PORT: &str = "8000";

#[actix_rt::main]
pub async fn create_routes() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::scope("/clients")
                    .route("/hi", web::get().to(handler::index))
                    .route("/files/upload/{email}", web::post().to(handler::upload))
                    .route("/files/download/{email}", web::get().to(handler::download))
                    .route("", web::get().to(handler::all_clients))
                    .route("/new", web::post().to(handler::create_client))
                    .route("/{email}", web::get().to(handler::get_client_data))
                    .route("/update/{id}", web::post().to(handler::update_client))
                    .route("/delete/{id}", web::post().to(handler::delete_client))
                    .route("/auth", web::post().to(handler::auth_client)),
            )
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
