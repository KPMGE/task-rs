use async_trait::async_trait;

use crate::infra::models::UserDb;

#[async_trait]
pub trait GetUserRepository {
    async fn get_user_by_email(&self, email: String) -> Result<UserDb, sqlx::Error>;
}
