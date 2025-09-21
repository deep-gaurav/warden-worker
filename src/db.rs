use crate::error::AppError;
use std::sync::Arc;
use worker::{D1Database, Env};

pub fn get_db(env: &Arc<Env>) -> Result<D1Database, AppError> {
    env.d1("vault1").map_err(AppError::Worker)
}
