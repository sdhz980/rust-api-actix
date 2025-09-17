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
        request.validate().map_err(|e| AppError::BadRequest(format!("\n{}", e)))?;

        if UserRepository::find_by_email(pool, &request.email).await?.is_some() {
            return Err(AppError::UserAlreadyExists("Email already exist".to_string()));
        }

        if UserRepository::find_by_username(pool, &request.username).await?.is_some() {
            return Err(AppError::UserAlreadyExists("Username already exist".to_string()));
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
            AppError::InvalidUserData("Invalid credentials".to_string())
        )?;

        if !verify_password(password, &user.password)? {
            return Err(AppError::InvalidUserData("Invalid credentials".to_string()));
        }

        let token = create_jwt(user.id)?;

        Ok(token)
    }
}
