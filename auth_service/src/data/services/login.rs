use std::error::Error;
use std::sync::Arc;

use crate::{
    data::{dto::LoginUserDto, repositories::GetUserRepository},
    domain::entities::{Claims, Token},
};

pub async fn login_service(
    repo: Arc<impl GetUserRepository>,
    user_dto: LoginUserDto,
) -> Result<Token, Box<dyn Error>> {
    let found_user = repo.get_user_by_email(user_dto.email).await.map_err(|e| {
        eprintln!("ERROR: {e}");
        e
    })?;

    let now = chrono::Local::now();
    let iat = now.timestamp();
    let expiration_time = chrono::Duration::hours(1);
    let exp = (now + expiration_time).timestamp();

    let claims = Claims {
        sub: found_user.id.to_string(),
        name: found_user.name,
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
