use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    data::services::{create_task_service, list_task_service},
    infra::repositories::task::TaskRepository,
};

#[get("/healthcheck")]
pub async fn healthcheck_controller() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/tasks")]
pub async fn create_task_controller(
    repo: Data<TaskRepository>,
    task_json: Json<crate::data::dto::CreateTaskDto>,
) -> impl Responder {
    let task = task_json.into_inner();
    match create_task_service(repo.into_inner(), task).await {
        Ok(created_task) => HttpResponse::Created().json(created_task),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/tasks")]
pub async fn list_task_controller(repo: Data<TaskRepository>) -> impl Responder {
    match list_task_service(repo.into_inner()).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
