use actix_web::{ App, HttpServer, web };
use handlers::{ greet::greet, health_checker::health_check_handler };

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 127.0.0.1:8000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check_handler))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run().await
}
