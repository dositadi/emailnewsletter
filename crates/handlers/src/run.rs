use actix_web::{ App, HttpServer, web };
use tokio::io;

use crate::{ greet::greet, health_checker::health_check_handler };

pub async fn run() -> Result<(), io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check_handler))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run().await
}
