use crate::data::{dto::CreateUserDto, repositories::CreateUserRepository};
use crate::infra::models::UserDb;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CreateUserRepository for UserRepository {
    async fn create_user(&self, user: CreateUserDto) -> Result<UserDb, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        let created_user = sqlx::query_as!(
            UserDb,
            r#"
                INSERT INTO "users" (name, email)
                VALUES ($1, $2)
                RETURNING id, name, email, created_at, updated_at
            "#,
            user.name,
            user.email
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(created_user)
    }
}
