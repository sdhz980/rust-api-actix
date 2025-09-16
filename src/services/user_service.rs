use validator::Validate;

use crate::{
    config::database::DbPool,
    errors::app_error::{ AppError, AppResult },
    models::user::{ CreateUserRequest },
    repositories::user_repository::UserRepository,
    utils::{ jwt::create_jwt, password::{ hash_password, verify_password } },
};

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: &DbPool, request: CreateUserRequest) -> AppResult<String> {
        request.validate().map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        if let Some(_) = UserRepository::find_by_email(pool, &request.email).await? {
            return Err(AppError::Validation("Email already exists".to_string()));
        }

        if let Some(_) = UserRepository::find_by_username(pool, &request.username).await? {
            return Err(AppError::Validation("Username already exists".to_string()));
        }

        let password_hash = hash_password(&request.password)?;

        UserRepository::create(pool, request, &password_hash).await?;

        Ok("Success creating user".to_string())
    }

    pub async fn authenticate_user(
        pool: &DbPool,
        email: &str,
        password: &str
    ) -> AppResult<String> {
        let user = UserRepository::find_by_email(pool, email).await?.ok_or_else(||
            AppError::Authentication("Invalid credentials".to_string())
        )?;

        if !verify_password(password, &user.password)? {
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        let token = create_jwt(user.id)?;

        Ok(token)
    }
}
