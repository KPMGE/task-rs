use actix_web::{HttpServer, App, Responder, get, HttpResponse};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
  HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
    App::new().service(healthcheck)
  )
  .bind(("127.0.0.1", 3333))?
  .run()
  .await
}
