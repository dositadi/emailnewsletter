use std::net::TcpListener;

use actix_web::{ App, HttpServer, dev::Server, web };
use tokio::io;

use crate::handlers::{ health_check_handler, subscribe };

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(||
        App::new()
            .route("/health_check", web::get().to(health_check_handler))
            .route("/subscriptions", web::post().to(subscribe))
    )
        .listen(listener)?
        .run();

    Ok(server)
}
