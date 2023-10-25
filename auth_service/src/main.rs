use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::infra::repositories::user::UserRepository;
use crate::presentation::controllers::{login_controller, signup_controller};

mod data;
mod domain;
mod infra;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("API_PORT")
        .expect("API_PORT environment variable must be set!")
        .parse::<u16>()
        .expect("API_PORT must be a number");
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set!");

    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(30))
        .connect_lazy(database_url.as_str())
        .expect("could not connect to the databse!");

    let user_repo = Data::new(UserRepository::new(pool));

    println!("server listening on: http://127.0.0.1:{}", port);

    HttpServer::new(move || {
        App::new()
            .service(login_controller)
            .service(signup_controller)
            .app_data(user_repo.clone())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
