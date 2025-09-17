use sqlx::{ query, query_as };

use crate::{
    config::database::DbPool,
    errors::app_error::AppResult,
    models::todo::{ CreateTodoRequest, TodoResponse, UpdateTodoRequest },
};

pub struct TodoRepository;

impl TodoRepository {
    pub async fn create(pool: &DbPool, request: CreateTodoRequest, user_id: i64) -> AppResult<()> {
        query(
            "
        INSERT INTO todos 
        (title , descriptions , user_id) 
        VALUES ( ? , ? , ? )
        "
        )
            .bind(&request.title)
            .bind(&request.descriptions)
            .bind(user_id)
            .execute(pool).await?;

        Ok(())
    }

    pub async fn get(pool: &DbPool, todo_id: i64, user_id: i64) -> AppResult<TodoResponse> {
        let todo = query_as::<_, TodoResponse>(
            "
        SELECT * id , title , descriptions , is_done , finished_at
        FROM todos 
        WHERE user_id = ? AND id = ?
        "
        )
            .bind(user_id)
            .bind(todo_id)
            .fetch_one(pool).await?;

        Ok(todo)
    }
    pub async fn list(pool: &DbPool, user_id: i64) -> AppResult<Vec<TodoResponse>> {
        let todos = query_as::<_, TodoResponse>(
            "
        SELECT * 
        id , title , descriptions , is_done , finished_at
        FROM todos 
        WHERE user_id = ?
        "
        )
            .bind(user_id)
            .fetch_all(pool).await?;

        Ok(todos)
    }
    pub async fn update(pool: &DbPool, request: UpdateTodoRequest, user_id: i64) -> AppResult<()> {
        query(
            "
        UPDATE todos 
        SET 
        title = COALESCE( ? , title ),
        descriptions = COALESCE( ? , descriptions ),
        is_done = COALESCE( ? , is_done ),
        finished_at = COALESCE( ? , finished_at )
        WHERE id = ? AND user_id = ?
        "
        )
            .bind(&request.title)
            .bind(&request.descriptions)
            .bind(&request.is_done)
            .bind(&request.finished_at)
            .bind(request.id)
            .bind(user_id)
            .execute(pool).await?;

        Ok(())
    }
    pub async fn delete(pool: &DbPool, user_id: i64, todo_id: i64) -> AppResult<()> {
        query("
        DELETE FROM todos
        WHERE id ? AND user_id ?
        ")
            .bind(todo_id)
            .bind(user_id)
            .execute(pool).await?;

        todo!()
    }
}
