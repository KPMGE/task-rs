use std::sync::Arc;

use crate::{
    data::{dto::CreateTaskDto, repositories::CreateTaskRepository},
    domain::{entities::Task, errors::CreateTaskError},
};

pub async fn create_task_service(
    repo: Arc<impl CreateTaskRepository>,
    task: CreateTaskDto,
) -> Result<Task, CreateTaskError> {
    if task.title.is_empty() {
        return Err(CreateTaskError::MissingFieldsError(
            "missing title!".to_string(),
        ));
    }

    let created_task_db = repo.create(task).await.map_err(|e| {
        eprintln!("ERROR (create_task_service): {:?}", e);
        CreateTaskError::RepoError
    })?;

    Ok(created_task_db.into())
}
