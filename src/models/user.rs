use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use sqlx::prelude::FromRow;
use validator::{ Validate, ValidationError };

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

fn is_password_valid(s: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in s.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }

    if !(!has_whitespace && has_upper && has_lower && has_digit && s.len() >= 8) {
        return Err(ValidationError::new("Password invalid"));
    }

    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 5, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "is_password_valid"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
