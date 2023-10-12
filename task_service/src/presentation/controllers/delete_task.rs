use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{data::services::delete_task_service, infra::repositories::task::TaskRepository};

#[derive(Deserialize)]
struct DeleteTaskParams {
    task_id: i32,
}

#[delete("/tasks/{task_id}")]
pub async fn delete_task_controller(
    repo: Data<TaskRepository>,
    params: Path<DeleteTaskParams>,
) -> impl Responder {
    let task_id = params.into_inner().task_id;

    match delete_task_service(repo.into_inner(), task_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
