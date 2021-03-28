use crate::sample::handler::{download, index, upload};
use actix_web::{web, App, HttpServer};

const PORT: &str = "8000";

#[actix_rt::main]
pub async fn create_routes() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/clients")
                .route("/hi", web::get().to(index))
                .route("/files/upload/{name}", web::post().to(upload))
                .route("/files/download/{name}", web::get().to(download)),
        )
    })
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
