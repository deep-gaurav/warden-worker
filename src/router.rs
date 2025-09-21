use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use worker::Env;

use crate::handlers::{accounts, ciphers, config, identity, sync};

pub fn api_router(env: Env) -> Router {
    let app_state = Arc::new(env);

    Router::new()
        // Identity/Auth routes
        .route("/identity/accounts/prelogin", post(accounts::prelogin))
        .route(
            "/identity/accounts/register/finish",
            post(accounts::register),
        )
        .route("/identity/connect/token", post(identity::token))
        .route(
            "/identity/accounts/register/send-verification-email",
            post(accounts::send_verification_email),
        )
        // Main data sync route
        .route("/api/sync", get(sync::get_sync_data))
        // Ciphers CRUD
        .route("/api/ciphers/create", post(ciphers::create_cipher))
        .route("/api/ciphers/{id}", put(ciphers::update_cipher))
        .route("/api/ciphers/{id}/delete", put(ciphers::delete_cipher))
        .route("/api/config", get(config::config))
        .with_state(app_state)
}
