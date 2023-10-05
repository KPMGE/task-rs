use actix_web::{get, HttpResponse, Responder};

#[get("/healthcheck")]
pub async fn healthcheck_controller() -> impl Responder {
    HttpResponse::Ok().finish()
}
