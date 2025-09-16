use bcrypt::{ hash, verify, DEFAULT_COST };

use crate::errors::app_error::{ AppError, AppResult };

pub fn hash_password(password: &str) -> AppResult<String> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::InternalServer)
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    verify(password, hash).map_err(|_| AppError::InternalServer)
}
