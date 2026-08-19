use actix_web::{App, HttpServer, web};
use handlers::greet::greet;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 127.0.0.1:8000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
