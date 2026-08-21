use actix_web::{ App, HttpServer, dev::Server, web };
use tokio::io;

use crate::{ health_checker::health_check_handler };

pub fn run() -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check_handler))
    })
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
