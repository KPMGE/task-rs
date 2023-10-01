use crate::domain::entities::Task;

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use sqlx::PgPool;

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct CreateTaskDto {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
}

#[post("/tasks")]
pub async fn create_task(pool: Data<PgPool>, task_json: Json<CreateTaskDto>) -> impl Responder {
    let task = task_json.into_inner();

    let mut transaction = pool
        .into_inner()
        .begin()
        .await
        .expect("could not create transaction");

    sqlx::query!(
        r#"
            INSERT INTO "tasks" (title, description, due_date) 
            VALUES($1, $2, $3)
        "#,
        task.title,
        task.description,
        task.due_date,
    )
    .execute(&mut transaction)
    .await
    .expect("error when creating task");

    transaction
        .commit()
        .await
        .expect("error when commiting transaction");

    HttpResponse::Created().finish()
}
