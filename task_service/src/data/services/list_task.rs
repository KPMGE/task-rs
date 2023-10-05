use std::sync::Arc;

use crate::{data::repositories::ListTaskRepository, domain::entities::Task};

pub async fn list_task_service(
    repo: Arc<impl ListTaskRepository>,
) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = repo
        .list()
        .await?
        .iter()
        .map(|task_db| task_db.into())
        .collect();

    Ok(tasks)
}
