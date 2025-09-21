use axum::Json;
use serde_json::{json, Value};

#[worker::send]
pub async fn config() -> Json<Value> {
    // let domain = crate::CONFIG.domain();
    // Official available feature flags can be found here:
    // Server (v2025.6.2): https://github.com/bitwarden/server/blob/d094be3267f2030bd0dc62106bc6871cf82682f5/src/Core/Constants.cs#L103
    // Client (web-v2025.6.1): https://github.com/bitwarden/clients/blob/747c2fd6a1c348a57a76e4a7de8128466ffd3c01/libs/common/src/enums/feature-flag.enum.ts#L12
    // Android (v2025.6.0): https://github.com/bitwarden/android/blob/b5b022caaad33390c31b3021b2c1205925b0e1a2/app/src/main/kotlin/com/x8bit/bitwarden/data/platform/manager/model/FlagKey.kt#L22
    // iOS (v2025.6.0): https://github.com/bitwarden/ios/blob/ff06d9c6cc8da89f78f37f376495800201d7261a/BitwardenShared/Core/Platform/Models/Enum/FeatureFlag.swift#L7
    // let mut feature_states =
    //     parse_experimental_client_feature_flags(&crate::CONFIG.experimental_client_feature_flags());
    // feature_states.insert("duo-redirect".to_string(), true);
    // feature_states.insert("email-verification".to_string(), true);
    // feature_states.insert("unauth-ui-refresh".to_string(), true);
    // feature_states.insert("enable-pm-flight-recorder".to_string(), true);
    // feature_states.insert("mobile-error-reporting".to_string(), true);

    let domain = "https://warden-worker.deepgauravraj.workers.dev";
    Json(json!({
        // Note: The clients use this version to handle backwards compatibility concerns
        // This means they expect a version that closely matches the Bitwarden server version
        // We should make sure that we keep this updated when we support the new server features
        // Version history:
        // - Individual cipher key encryption: 2024.2.0
        "version": "2024.7.0",
        "gitHash": "25cf6119-dirty",
        "server": {
          "name": "Vaultwarden",
          "url": "https://github.com/dani-garcia/vaultwarden"
        },
        "settings": {
            "disableUserRegistration": true,
        },
        "environment": {
          "vault": domain,
          "api": format!("{domain}/api"),
          "identity": format!("{domain}/identity"),
          "notifications": format!("{domain}/notifications"),
          "sso": format!("{domain}/sso"),
          "cloudRegion": null,
        },
        // Bitwarden uses this for the self-hosted servers to indicate the default push technology
        "push": {
          "pushTechnology": 0,
          "vapidPublicKey": null
        },
        "featureStates": {
            // "duo-redirect": true,
            // "flexible-collections-v-1": false
        },
        "object": "config",
    }))
}
