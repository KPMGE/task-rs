use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::domain::entities::{Claims, User};

mod domain;
mod data;
mod infra;
mod presentation;

#[post("/signup")]
async fn signup(user_data: Json<User>) -> impl Responder {
    println!("got user: {:?}", user_data.into_inner());
    HttpResponse::Ok().finish()
}

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
    let port = 3334;

    println!("server listening on: http://127.0.0.1:{}", port);

    HttpServer::new(move || {
        App::new()
            .service(login)
            .service(test_token)
            .service(signup)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
