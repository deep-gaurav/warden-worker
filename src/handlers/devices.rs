use axum::{extract::State, Json};
use axum_extra::extract::WithRejection;
use axum::http::HeaderMap;
use chrono::Utc;
use std::sync::Arc;
use worker::{query, Env};
use axum::extract::Path;

use crate::db;
use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PushTokenRequest {
    #[serde(rename = "pushToken")]
    pub push_token: String,
}

#[derive(Deserialize)]
struct CountResult {
    count: i32,
}

#[worker::send]
pub async fn get_known_device(
    State(env): State<Arc<Env>>,
    headers: HeaderMap,
) -> Result<Json<bool>, AppError> {
    let email = headers
        .get("X-Request-Email")
        .and_then(|h| h.to_str().ok())
        .map(|s| {
             // Basic base64 decode if it looks encoded, otherwise use as is
             // The Android app base64 encodes it, but we should handle both or check usage
             // For now assuming raw or standard bas64. 
             // Android code: emailAddress.base64UrlEncode()
             // So we should decode it.
             use base64::{engine::general_purpose, Engine as _};
             let decoded = general_purpose::URL_SAFE_NO_PAD.decode(s).unwrap_or_else(|_| s.as_bytes().to_vec());
             String::from_utf8(decoded).unwrap_or_else(|_| s.to_string())
        });
    
    let device_identifier = headers
        .get("X-Device-Identifier")
        .and_then(|h| h.to_str().ok());

    if let (Some(email), Some(device_identifier)) = (email, device_identifier) {
         let db = db::get_db(&env)?;
         // Check if device exists for this user (by email lookup)
         // First get user id from email
         let user_id_res = query!(
             &db,
             "SELECT id FROM users WHERE email = ?1",
             email
         )
         .map_err(|_| AppError::Database)?
         .first::<crate::models::user::User>(None) 
         .await; // This projection might fail if we don't select all fields or use a partial struct. 
         // Let's use raw Value or a partial struct. 
         
         // Actually, let's just count
         // But we need to join or subquery. 
         // "SELECT count(*) FROM devices JOIN users ON devices.user_id = users.id WHERE users.email = ? AND devices.identifier = ?"
         
         let result = query!(
             &db,
             "SELECT COUNT(*) as count FROM devices 
              JOIN users ON devices.user_id = users.id 
              WHERE users.email = ?1 AND devices.identifier = ?2",
             email,
             device_identifier
         )
         .map_err(|_| AppError::Database)?
         .first::<CountResult>(None) 
         .await?;
         
         if let Some(row) = result {
             return Ok(Json(row.count > 0));
         }
    }

    Ok(Json(false))
}

#[worker::send]
pub async fn put_token(
    State(env): State<Arc<Env>>,
    Path(id): Path<String>,
    Json(payload): Json<PushTokenRequest>,
) -> Result<Json<()>, AppError> {
    let db = db::get_db(&env)?;
    let now = Utc::now();
    let now_str = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    // Update the push token for the device with the given identifier
    // Note: The `id` in the path is the device identifier, not the primary key ID.
    // The android app sends: /devices/identifier/{appId}/token
    
    let _ = query!(
        &db,
        "UPDATE devices SET push_token = ?1, updated_at = ?2 WHERE identifier = ?3",
        payload.push_token,
        now_str,
        id
    )
    .map_err(|_| AppError::Database)?
    .run()
    .await?;

    Ok(Json(()))
}
