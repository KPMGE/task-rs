use crate::data::repositories::DeleteTaskRepository;
use std::sync::Arc;

pub async fn delete_task_service(
    repo: Arc<impl DeleteTaskRepository>,
    task_id: i32,
) -> Result<(), sqlx::Error> {
    repo.delete(task_id).await
}
