use crate::domain::entities::Task;

use actix_web::{get, post, web::Json, HttpResponse, Responder};

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/tasks")]
pub async fn create_task(task: Json<Task>) -> impl Responder {
    println!("task: {:?}", task.into_inner());
    HttpResponse::Ok().finish()
}
