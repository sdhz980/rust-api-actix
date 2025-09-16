use sqlx::{ query, query_as };

use crate::{
    config::database::DbPool,
    errors::app_error::AppResult,
    models::user::{ CreateUserRequest, User },
};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &DbPool,
        request: CreateUserRequest,
        password_hash: &str
    ) -> AppResult<()> {
        query(
            r#"
                INSERT INTO users (username,email,password,created_at,updated_at)
                VALUES ( ? , ? , ? , CURRENT_TIMESTAMP , CURRENT_TIMESTAMP )
            "#
        )
            .bind(&request.username)
            .bind(&request.email)
            .bind(password_hash)
            .execute(pool).await?;

        Ok(())
    }

    pub async fn find_by_email(pool: &DbPool, email: &str) -> AppResult<Option<User>> {
        let user = query_as::<_, User>(
            r#"
                SELECT * FROM users where email = ?
            "#
        )
            .bind(email)
            .fetch_optional(pool).await?;

        Ok(user)
    }

    pub async fn find_by_username(pool: &DbPool, username: &str) -> AppResult<Option<User>> {
        let user = query_as::<_, User>(
            r#"
                SELECT * FROM users where username = ?
            "#
        )
            .bind(username)
            .fetch_optional(pool).await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &DbPool, id: u64) -> AppResult<Option<User>> {
        let user = query_as::<_, User>(
            r#"
                SELECT * FROM users where id = ?
            "#
        )
            .bind(id)
            .fetch_optional(pool).await?;

        Ok(user)
    }

    pub async fn list(pool: &DbPool, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let user = query_as::<_, User>(
            r#"
                SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?
            "#
        )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;

        Ok(user)
    }
}
