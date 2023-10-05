use std::sync::Arc;

use crate::{data::repositories::ListTaskRepository, domain::entities::Task};

pub async fn list_task_service(
    repo: Arc<impl ListTaskRepository>,
) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = repo
        .list()
        .await?
        .iter()
        .map(|task_db| Task {
            id: task_db.id,
            title: task_db.title.clone(),
            description: task_db.description.clone(),
            due_date: task_db.due_date,
        })
        .collect();

    Ok(tasks)
}
