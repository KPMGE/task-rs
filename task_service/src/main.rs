use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use presentation::controllers::{create_task, healthcheck};
use sqlx::postgres::PgPoolOptions;
use std::{env, time::Duration};

mod domain;
mod presentation;
mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set!");

    let pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(20))
        .connect_lazy(database_url.as_str())
        .expect("could not connect to the database!");

    HttpServer::new(move || {
        App::new()
            .service(healthcheck)
            .service(create_task)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
