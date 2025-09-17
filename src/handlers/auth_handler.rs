use actix_web::{ web, HttpResponse };
use serde_json::json;

use crate::{
    config::database::DbPool,
    errors::app_error::AppResult,
    models::user::{ CreateUserRequest, LoginRequest },
    services::user_service::UserService,
};

pub async fn register(
    pool: web::Data<DbPool>,
    request: web::Json<CreateUserRequest>
) -> AppResult<HttpResponse> {
    let user = UserService::create_user(&pool, request.into_inner()).await?;

    Ok(
        HttpResponse::Created().json(
            json!({
            "status" : "success",
            "message" : user
        })
        )
    )
}

pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>
) -> AppResult<HttpResponse> {
    let user = UserService::authenticate_user(&pool, &request.email, &request.password).await?;

    Ok(
        HttpResponse::Created().json(
            json!({
            "status" : "success",
            "data" : user
        })
        )
    )
}
