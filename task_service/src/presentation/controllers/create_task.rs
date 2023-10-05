use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    data::{dto::CreateTaskDto, services::create_task_service},
    domain::errors::CreateTaskError,
    infra::repositories::task::TaskRepository,
};

#[post("/tasks")]
pub async fn create_task_controller(
    repo: Data<TaskRepository>,
    task_json: Json<CreateTaskDto>,
) -> impl Responder {
    let task = task_json.into_inner();

    match create_task_service(repo.into_inner(), task).await {
        Ok(created_task) => HttpResponse::Created().json(created_task),
        Err(e) => match e {
            CreateTaskError::MissingFieldsError(msg) => HttpResponse::BadRequest().json(msg),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
