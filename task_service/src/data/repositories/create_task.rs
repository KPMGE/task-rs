use async_trait::async_trait;

use crate::{data::dto::CreateTaskDto, infra::models::task::TaskDb};

#[async_trait]
pub trait CreateTaskRepository {
    async fn create(&self, task: CreateTaskDto) -> Result<TaskDb, sqlx::Error>;
}
