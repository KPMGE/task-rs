use async_trait::async_trait;

#[async_trait]
pub trait DeleteTaskRepository {
    async fn delete(&self, task_id: i32) -> Result<(), sqlx::Error>;
}
