use async_trait::async_trait;
use crate::{data::dto::CreateUserDto, infra::models::UserDb};

#[async_trait]
pub trait CreateUserRepository {
    async fn create_user(&self, user: CreateUserDto) -> Result<UserDb, sqlx::Error>;
}
