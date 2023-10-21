use std::sync::Arc;

use crate::{
    data::{dto::CreateUserDto, repositories::CreateUserRepository},
    domain::entities::{Claims, Token},
};

pub async fn signup_service(
    repo: Arc<impl CreateUserRepository>,
    user: CreateUserDto,
) -> Result<Token, sqlx::Error> {
    let created_user = repo.create_user(user).await?;
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
    let secret_str = "super secret".as_bytes();
    let secret_key = jsonwebtoken::EncodingKey::from_secret(secret_str);

    let token = jsonwebtoken::encode(&header, &claims, &secret_key)
        .expect("error while encoding user info");

    Ok(Token { token })
}
