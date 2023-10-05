use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    data::{
        dto::CreateTaskDto,
        repositories::{CreateTaskRepository, ListTaskRepository},
    },
    infra::models::task::TaskDb,
};

pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CreateTaskRepository for TaskRepository {
    async fn create(&self, task: CreateTaskDto) -> Result<TaskDb, sqlx::Error> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .expect("could not create transaction");

        let created_task_db = sqlx::query_as!(
            TaskDb,
            r#"
                INSERT INTO "tasks" (title, description, due_date) 
                VALUES($1, $2, $3)
                RETURNING id, title, description, due_date, created_at, updated_at
            "#,
            task.title,
            task.description,
            task.due_date,
        )
        .fetch_one(&mut transaction)
        .await
        .expect("error when creating task");

        transaction
            .commit()
            .await
            .expect("error when commiting transaction");

        Ok(created_task_db)
    }
}

#[async_trait]
impl ListTaskRepository for TaskRepository {
    async fn list(&self) -> Result<Vec<TaskDb>, sqlx::Error> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .expect("could not create transaction");

        let tasks = sqlx::query_as!(
            TaskDb,
            r#"
                SELECT id, title, description, due_date, created_at, updated_at 
                FROM "tasks"
            "#
        )
        .fetch_all(&mut transaction)
        .await
        .expect("error when fetching tasks");

        transaction
            .commit()
            .await
            .expect("error when commiting transaction");

        Ok(tasks)
    }
}
