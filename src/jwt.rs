use thiserror::Error;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    pub user_id: Uuid
}

#[derive(Debug, Error)]
pub enum JWTError {
    #[error("JWT creation failed")]
    CreationFailed
}

const JWT_SECRET: &[u8] = b"secret_key_777";

pub fn create_jwt(uid: Uuid) -> Result<String, JWTError> {
    use chrono::prelude::*;

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("Valid timestamp")
        .timestamp();

    let claims = Claims {
        exp: expiration as usize,
        user_id: uid
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| JWTError::CreationFailed)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default()
    )
}
