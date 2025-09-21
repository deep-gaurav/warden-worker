use axum::{extract::State, Form, Json};
use chrono::{Duration, Utc};
use constant_time_eq::constant_time_eq;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use worker::Env;

use crate::{auth::Claims, db, error::AppError, models::user::User};

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    grant_type: String,
    username: String,
    password: Option<String>, // This is the masterPasswordHash
                              // Other fields like scope, client_id, device info are ignored for this basic implementation
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TokenResponse {
    #[serde(rename = "access_token")]
    access_token: String,
    #[serde(rename = "expires_in")]
    expires_in: i64,
    #[serde(rename = "token_type")]
    token_type: String,
    #[serde(rename = "refresh_token")]
    refresh_token: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "PrivateKey")]
    private_key: String,
    #[serde(rename = "Kdf")]
    kdf: i32,
    #[serde(rename = "ResetMasterPassword")]
    reset_master_password: bool,
    #[serde(rename = "ForcePasswordReset")]
    force_password_reset: bool,
    #[serde(rename = "UserDecryptionOptions")]
    user_decryption_options: UserDecryptionOptions,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserDecryptionOptions {
    pub has_master_password: bool,
    pub object: String,
}

#[worker::send]
pub async fn token(
    State(env): State<Arc<Env>>,
    Form(payload): Form<TokenRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    if payload.grant_type != "password" {
        return Err(AppError::BadRequest("Unsupported grant_type".to_string()));
    }
    let password_hash = payload
        .password
        .ok_or_else(|| AppError::BadRequest("Missing password".to_string()))?;

    let db = db::get_db(&env)?;

    let user: Value = db
        .prepare("SELECT * FROM users WHERE email = ?1")
        .bind(&[payload.username.to_lowercase().into()])?
        .first(None)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;
    let user: User = serde_json::from_value(user).map_err(|_| AppError::Internal)?;
    // Securely compare the provided hash with the stored hash
    if !constant_time_eq(
        user.master_password_hash.as_bytes(),
        password_hash.as_bytes(),
    ) {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Create JWT claims
    let now = Utc::now();
    let expires_in = Duration::hours(1);
    let exp = (now + expires_in).timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        exp,
        nbf: now.timestamp() as usize,
        premium: true,
        name: user.name.unwrap_or("User".to_string()),
        email: user.email,
        email_verified: true,
        amr: vec!["Application".into()],
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("a-very-secure-secret-key-that-should-be-in-env".as_ref()),
    )?;

    // In a real implementation, the refresh token would be securely generated and stored
    let refresh_token = "static-refresh-token-for-this-example".to_string();

    Ok(Json(TokenResponse {
        access_token: token,
        expires_in: expires_in.num_seconds(),
        token_type: "Bearer".to_string(),
        refresh_token,
        key: user.key,
        private_key: user.private_key,
        kdf: user.kdf_type,
        force_password_reset: false,
        reset_master_password: false,
        user_decryption_options: UserDecryptionOptions {
            has_master_password: true,
            object: "userDecryptionOptions".to_string(),
        },
    }))
}
