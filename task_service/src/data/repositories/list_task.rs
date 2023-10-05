use async_trait::async_trait;

use crate::infra::models::task::TaskDb;

#[async_trait]
pub trait ListTaskRepository {
    async fn list(&self) -> Result<Vec<TaskDb>, sqlx::Error>;
}
