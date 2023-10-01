use std::sync::Arc;

use crate::{
    data::{dto::CreateTaskDto, repositories::CreateTaskRepository},
    domain::entities::Task,
};

pub async fn create_task_service(
    repo: Arc<impl CreateTaskRepository>,
    task: CreateTaskDto,
) -> Result<Task, sqlx::Error> {
    let created_task_db = repo.create(task).await?;
    Ok(created_task_db.into())
}
