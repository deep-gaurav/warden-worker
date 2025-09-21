use axum::{extract::State, Json};
use serde_json::Value;
use std::sync::Arc;
use worker::Env;

use crate::{
    auth::Claims,
    db,
    error::AppError,
    models::{
        cipher::{Cipher, CipherDBModel},
        folder::{Folder, FolderResponse},
        sync::{Profile, SyncResponse},
        user::User,
    },
};

#[worker::send]
pub async fn get_sync_data(
    claims: Claims,
    State(env): State<Arc<Env>>,
) -> Result<Json<SyncResponse>, AppError> {
    let user_id = claims.sub;
    let db = db::get_db(&env)?;

    // Fetch profile
    let user: User = db
        .prepare("SELECT * FROM users WHERE id = ?1")
        .bind(&[user_id.clone().into()])?
        .first(None)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Fetch folders
    let folders_db: Vec<Folder> = db
        .prepare("SELECT * FROM folders WHERE user_id = ?1")
        .bind(&[user_id.clone().into()])?
        .all()
        .await?
        .results()?;

    let folders: Vec<FolderResponse> = folders_db.into_iter().map(|f| f.into()).collect();

    // Fetch ciphers
    let ciphers: Vec<Value> = db
        .prepare("SELECT * FROM ciphers WHERE user_id = ?1")
        .bind(&[user_id.clone().into()])?
        .all()
        .await?
        .results()?;

    let ciphers = ciphers
        .into_iter()
        .filter_map(
            |cipher| match serde_json::from_value::<CipherDBModel>(cipher.clone()) {
                Ok(cipher) => Some(cipher),
                Err(err) => {
                    log::warn!("Cannot parse {err:?} {cipher:?}");
                    None
                }
            },
        )
        .map(|cipher| cipher.into())
        .collect::<Vec<Cipher>>();

    let time = chrono::DateTime::parse_from_rfc3339(&user.created_at)
        .map_err(|_| AppError::Internal)?
        .to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
    let profile = Profile {
        id: user.id,
        name: user.name,
        email: user.email,
        master_password_hint: user.master_password_hint,
        security_stamp: user.security_stamp,
        object: "profile".to_string(),
        premium: true,
        premium_from_organization: false,
        email_verified: true,
        force_password_reset: false,
        two_factor_enabled: false,
        uses_key_connector: false,
        creation_date: time,
        key: user.key,
        private_key: user.private_key,
    };

    let response = SyncResponse {
        profile,
        folders,
        ciphers,
        domains: serde_json::Value::Null, // Ignored for basic implementation
        object: "sync".to_string(),
    };

    Ok(Json(response))
}