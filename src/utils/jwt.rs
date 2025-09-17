use std::env;

use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };

use crate::errors::app_error::{ AppError, AppResult };

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_jwt(user_id: i64) -> AppResult<String> {
    let secret = env::var("JWT_SECRET").map_err(|_| AppError::InternalServer)?;

    let now = Utc::now();
    let expire = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expire.timestamp(),
        iat: now.timestamp(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(
        |_| AppError::InternalServer
    )
}

pub fn verify_jwt(token: &str) -> AppResult<Claims> {
    let secret = env::var("JWT_SECRET").map_err(|_| AppError::InternalServer)?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256)
    )
        .map(|data| data.claims)
        .map_err(|_| AppError::Authorization("Invalid token".to_string()))
}
