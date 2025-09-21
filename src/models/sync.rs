use super::{cipher::Cipher, folder::Folder, user::User};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Profile {
    pub name: Option<String>,
    pub email: String,
    pub id: String,
    #[serde(rename = "masterPasswordHint")]
    pub master_password_hint: Option<String>,
    #[serde(rename = "securityStamp")]
    pub security_stamp: String,
    #[serde(rename = "Object")]
    pub object: String,
    #[serde(rename = "premiumFromOrganization")]
    pub premium_from_organization: bool,
    #[serde(rename = "forcePasswordReset")]
    pub force_password_reset: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "twoFactorEnabled")]
    pub two_factor_enabled: bool,
    pub premium: bool,
    #[serde(rename = "usesKeyConnector")]
    pub uses_key_connector: bool,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    #[serde(rename = "privateKey")]
    pub private_key: String,
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    #[serde(rename = "Profile")]
    pub profile: Profile,
    #[serde(rename = "Folders")]
    pub folders: Vec<Folder>,
    #[serde(rename = "ciphers")]
    pub ciphers: Vec<Cipher>,
    #[serde(rename = "Domains")]
    pub domains: Value,
    #[serde(rename = "Object")]
    pub object: String,
}
