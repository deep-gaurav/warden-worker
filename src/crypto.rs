use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CryptoKey, SubtleCrypto, WorkerGlobalScope};
use worker::js_sys;

use crate::error::AppError;

pub fn worker_global() -> Option<WorkerGlobalScope> {
    use wasm_bindgen::JsCast;

    js_sys::global().dyn_into::<WorkerGlobalScope>().ok()
}

/// Gets the SubtleCrypto interface from the global scope.
fn subtle_crypto() -> Result<SubtleCrypto, AppError> {
    Ok(worker_global()
        .ok_or_else(|| AppError::Crypto("Could not get worker global scope".to_string()))?
        .crypto()
        .map_err(|e| AppError::Crypto(format!("Failed to get crypto: {:?}", e)))?
        .subtle())
}

/// Derives a key using PBKDF2-SHA256.
pub async fn pbkdf2_sha256(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
    key_length_bits: u32,
) -> Result<Vec<u8>, AppError> {
    let subtle = subtle_crypto()?;

    // Import the password as a raw key material
    let password_array = Uint8Array::new_from_slice(password);
    let password_obj = password_array.as_ref();
    let key_material = JsFuture::from(
        subtle
            .import_key_with_str(
                "raw",
                password_obj,
                "PBKDF2",
                false,
                &js_sys::Array::of1(&JsValue::from_str("deriveBits")),
            )
            .map_err(|e| AppError::Crypto(format!("PBKDF2 import_key failed: {:?}", e)))?,
    )
    .await
    .map_err(|e| AppError::Crypto(format!("PBKDF2 import_key await failed: {:?}", e)))?;

    let salt_array = Uint8Array::new_from_slice(salt);
    // Define PBKDF2 parameters
    let mut params = web_sys::Pbkdf2Params::new(
        "PBKDF2",
        JsValue::from_str("SHA-256").as_ref(),
        iterations,
        salt_array.as_ref(),
    );

    // Derive the bits
    let derived_bits = JsFuture::from(
        subtle
            .derive_bits_with_object(
                params.as_ref(),
                &CryptoKey::from(key_material),
                key_length_bits,
            )
            .map_err(|e| AppError::Crypto(format!("PBKDF2 derive_bits failed: {:?}", e)))?,
    )
    .await
    .map_err(|e| AppError::Crypto(format!("PBKDF2 derive_bits await failed: {:?}", e)))?;

    Ok(js_sys::Uint8Array::new(&derived_bits).to_vec())
}

/// Generates a hash of the master key for password verification.
pub async fn hash_master_key(
    master_key: &[u8],
    master_password: &[u8],
) -> Result<Vec<u8>, AppError> {
    pbkdf2_sha256(master_key, master_password, 1, 256).await
}
