use crate::domain::errors::{ApiError, ApiErrorType};
use std::env;
use std::sync::Arc;
use validator::{Validate, ValidationErrors};

use crate::{
    data::{dto::CreateUserDto, repositories::CreateUserRepository},
    domain::entities::{Claims, Token},
};

impl Into<ApiError> for ValidationErrors {
    fn into(self) -> ApiError {
        ApiError {
            cause: Some(self.to_string()),
            message: Some("Invalid request".to_string()),
            error_type: ApiErrorType::ValidationError,
        }
    }
}

impl Into<ApiError> for sqlx::Error {
    fn into(self) -> ApiError {
        ApiError {
            cause: Some(self.to_string()),
            message: Some("Internal server  error".to_string()),
            error_type: ApiErrorType::DatabaseError,
        }
    }
}

impl Into<ApiError> for jsonwebtoken::errors::Error {
    fn into(self) -> ApiError {
        ApiError {
            cause: Some(self.to_string()),
            message: Some("Internal server error".to_string()),
            error_type: ApiErrorType::UnexpectedError,
        }
    }
}

pub async fn signup_service(
    repo: Arc<impl CreateUserRepository>,
    user: CreateUserDto,
) -> Result<Token, ApiError> {
    user.validate().map_err(|e| e.into())?;

    let created_user = repo.create_user(user).await.map_err(|e| e.into())?;
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

    let token = jsonwebtoken::encode(&header, &claims, &secret_key).map_err(|e| e.into())?;

    Ok(Token { token })
}
