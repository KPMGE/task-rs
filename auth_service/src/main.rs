use chrono::{Duration, Local};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::domain::entities::Claims;
use crate::infra::repositories::user::UserRepository;
use crate::presentation::controllers::signup_controller;

mod data;
mod domain;
mod infra;
mod presentation;

#[get("/login")]
async fn login() -> impl Responder {
    let now = Local::now();
    let iat = now.timestamp();
    let expiration_time = Duration::hours(1);
    let exp = (now + expiration_time).timestamp();

    let my_claims = Claims {
        sub: "1".to_string(),
        name: "test name".to_string(),
        iat,
        exp,
    };

    let jwt_secret = "super secret";

    let token = match jsonwebtoken::encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|e| {
        eprintln!("ERROR: {}", e);
        e
    }) {
        Ok(token) => token,
        Err(_e) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(token)
}

#[derive(Debug, Serialize, Deserialize)]
struct Params {}

#[get("/test_token")]
async fn test_token(auth: BearerAuth) -> impl Responder {
    let token = auth.token();

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret("super secret".as_ref()),
        &Validation::default(),
    )
    .unwrap();

    println!("claims: {:?}", token_data.claims);

    HttpResponse::Ok().finish()
}

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
            .service(login)
            .service(test_token)
            .service(signup_controller)
            .app_data(user_repo.clone())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
