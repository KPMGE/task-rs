use actix_web::{App, HttpServer};
use presentation::controllers::{create_task, healthcheck};

mod domain;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck).service(create_task))
        .bind(("127.0.0.1", 3333))?
        .run()
        .await
}
