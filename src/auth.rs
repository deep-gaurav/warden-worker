use axum::{
    extract::{FromRequestParts, State},
    http::{header, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use worker::Env;

use crate::error::AppError;

// Secret key for signing JWTs. In a real application, this should be a strong,
// securely stored secret from the Worker's environment variables.
const JWT_SECRET: &str = "a-very-secure-secret-key-that-should-be-in-env";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time
    pub nbf: usize,  // Not before time

    pub premium: bool,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub amr: Vec<String>,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_value| {
                if auth_value.starts_with("Bearer ") {
                    Some(auth_value[7..].to_owned())
                } else {
                    None
                }
            })
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid token".to_string()))?;

        // Decode and validate the token
        let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_ref());
        let token_data = decode::<Claims>(&token, &decoding_key, &Validation::default())
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        Ok(token_data.claims)
    }
}
