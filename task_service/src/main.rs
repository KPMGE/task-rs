mod domain;

use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/tasks")]
async fn create_task(task: Json<domain::entities::Task>) -> impl Responder {
    println!("task: {:?}", task.into_inner());
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck).service(create_task))
        .bind(("127.0.0.1", 3333))?
        .run()
        .await
}
