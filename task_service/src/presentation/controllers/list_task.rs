use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{data::services::list_task_service, infra::repositories::task::TaskRepository};

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
