use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use infra::repositories::task::TaskRepository;
use sqlx::postgres::PgPoolOptions;
use std::{env, time::Duration};
use presentation::controllers::{
    create_task_controller,
    healthcheck_controller, 
    list_task_controller,
    delete_task_controller
};

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
        .connect_timeout(Duration::from_secs(20))
        .connect_lazy(database_url.as_str())
        .expect("could not connect to the database!");

    let task_repository = Data::new(TaskRepository::new(pool.clone()));

    println!("Server running on http://localhost:{}", port);
    HttpServer::new(move || {
        App::new()
            .service(healthcheck_controller)
            .service(create_task_controller)
            .service(list_task_controller)
            .service(delete_task_controller)
            .app_data(task_repository.clone())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
