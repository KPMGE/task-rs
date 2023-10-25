use std::env;
use std::sync::Arc;
use validator::Validate;

use crate::{
    data::{dto::CreateUserDto, repositories::CreateUserRepository},
    domain::{
        entities::{Claims, Token},
        errors::SignupError,
    },
};

pub async fn signup_service(
    repo: Arc<impl CreateUserRepository>,
    user: CreateUserDto,
) -> Result<Token, SignupError> {
    user.validate()
        .map_err(|e| SignupError::InvalidUserError(e))?;

    let created_user = repo
        .create_user(user)
        .await
        .map_err(|e| SignupError::DatabaseError(e))?;
    let now = chrono::Local::now();
    let iat = now.timestamp();
    let expiration_time = chrono::Duration::hours(1);
    let exp = (now + expiration_time).timestamp();

    let claims = Claims {
        sub: created_user.id.to_string(),
        name: created_user.name,
        iat,
        exp,
    };

    let header = jsonwebtoken::Header::default();
    let secret_str = env::var("JWT_SECRET").unwrap().clone();
    let secret_key = jsonwebtoken::EncodingKey::from_secret(secret_str.as_bytes());

    let token = jsonwebtoken::encode(&header, &claims, &secret_key)
        .map_err(|e| SignupError::TokenError(e))?;

    Ok(Token { token })
}
