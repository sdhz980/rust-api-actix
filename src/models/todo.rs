use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use sqlx::prelude::FromRow;
use validator::{ Validate };

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub descriptions: Option<String>,
    pub is_done: bool,
    pub finished_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TodoResponse {
    pub id: i64,
    pub title: String,
    pub descriptions: Option<String>,
    pub is_done: bool,
    pub finished_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 5, max = 256, message = "Title require min 5 and max 5 characters."))]
    pub title: String,

    #[validate(
        length(min = 5, max = 1021, message = "Descriptions require min 5 and max 1021 characters.")
    )]
    pub descriptions: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(range(min = 0, message = "Todo id required."))]
    pub id: i64,

    #[validate(length(min = 5, max = 256, message = "Title require min 5 and max 5 characters."))]
    pub title: String,

    #[validate(
        length(min = 5, max = 1021, message = "Descriptions require min 5 and max 1021 characters.")
    )]
    pub descriptions: Option<String>,

    #[validate(required)]
    pub is_done: Option<bool>,

    #[validate(required)]
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DeleteTodoRequest {}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            descriptions: todo.descriptions,
            is_done: todo.is_done,
            finished_at: todo.finished_at,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}
