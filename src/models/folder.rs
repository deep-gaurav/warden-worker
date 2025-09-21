use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    pub user_id: String,
    // The name is encrypted client-side
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
