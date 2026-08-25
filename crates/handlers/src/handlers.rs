use actix_web::{ HttpResponse, web };

pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    if _form.email.is_empty() || _form.name.is_empty() {
        HttpResponse::BadRequest().finish()
    } else {
        HttpResponse::Ok().finish()
    }
    
}
