use axum::{extract::State, Json};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;
use worker::{query, Env};

use crate::auth::Claims;
use crate::db;
use crate::error::AppError;
use crate::models::folder::{CreateFolderRequest, Folder, FolderResponse};
use axum::extract::Path;

#[worker::send]
pub async fn create_folder(
    claims: Claims,
    State(env): State<Arc<Env>>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<FolderResponse>, AppError> {
    let db = db::get_db(&env)?;
    let now = Utc::now();
    let now = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    let folder = Folder {
        id: Uuid::new_v4().to_string(),
        user_id: claims.sub.clone(),
        name: payload.name,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    query!(
        &db,
        "INSERT INTO folders (id, user_id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        folder.id,
        folder.user_id,
        folder.name,
        folder.created_at,
        folder.updated_at
    )
    .map_err(|_| AppError::Database)?
    .run()
    .await?;

    let response = FolderResponse {
        id: folder.id,
        name: folder.name,
        revision_date: folder.updated_at,
        object: "folder".to_string(),
    };

    Ok(Json(response))
}

#[worker::send]
pub async fn delete_folder(
    claims: Claims,
    State(env): State<Arc<Env>>,
    Path(id): Path<String>,
) -> Result<Json<()>, AppError> {
    let db = db::get_db(&env)?;

    query!(
        &db,
        "DELETE FROM folders WHERE id = ?1 AND user_id = ?2",
        id,
        claims.sub
    )
    .map_err(|_| AppError::Database)?
    .run()
    .await?;

    Ok(Json(()))
}
#[worker::send]
pub async fn update_folder(
    claims: Claims,
    State(env): State<Arc<Env>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<FolderResponse>, AppError> {
    let db = db::get_db(&env)?;
    let now = Utc::now();
    let now = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    let existing_folder: Folder = query!(
        &db,
        "SELECT * FROM folders WHERE id = ?1 AND user_id = ?2",
        id,
        claims.sub
    )
    .map_err(|_| AppError::Database)?
    .first(None)
    .await?
    .ok_or(AppError::NotFound("Folder not found".to_string()))?;

    let folder = Folder {
        id: id.clone(),
        user_id: existing_folder.user_id,
        name: payload.name,
        created_at: existing_folder.created_at,
        updated_at: now.clone(),
    };

    query!(
        &db,
        "UPDATE folders SET name = ?1, updated_at = ?2 WHERE id = ?3 AND user_id = ?4",
        folder.name,
        folder.updated_at,
        folder.id,
        folder.user_id
    )
    .map_err(|_| AppError::Database)?
    .run()
    .await?;

    let response = FolderResponse {
        id: folder.id,
        name: folder.name,
        revision_date: folder.updated_at,
        object: "folder".to_string(),
    };

    Ok(Json(response))
}
