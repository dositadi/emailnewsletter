use actix_web::HttpResponse;

pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}
