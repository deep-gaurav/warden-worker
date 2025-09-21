use axum::{extract::State, Json};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;
use worker::Env;

use crate::{auth::Claims, db, error::AppError, models::cipher::Cipher};

#[worker::send]
pub async fn create_cipher(
    claims: Claims,
    State(env): State<Arc<Env>>,
    Json(mut payload): Json<Cipher>,
) -> Result<Json<Cipher>, AppError> {
    let db = db::get_db(&env)?;
    let now = Utc::now().to_rfc3339();

    payload.id = Uuid::new_v4().to_string();
    payload.user_id = Some(claims.sub);
    payload.created_at = now.clone();
    payload.updated_at = now;

    db.prepare(
        "INSERT INTO ciphers (id, user_id, type, data, favorite, folder_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )
    .bind(&[
        payload.id.clone().into(),
        payload.user_id.clone().into(),
        payload.r#type.into(),
        serde_json::to_string(&payload.data).unwrap().into(),
        payload.favorite.into(),
        payload.folder_id.clone().into(),
        payload.created_at.clone().into(),
        payload.updated_at.clone().into(),
    ])?
    .run()
    .await?;

    Ok(Json(payload))
}
