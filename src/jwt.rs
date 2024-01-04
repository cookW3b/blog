use thiserror::Error;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user_id: Uuid
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

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| JWTError::CreationFailed)
}
