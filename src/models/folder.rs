use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub user_id: String,
    // The name is encrypted client-side
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderResponse {
    pub id: String,
    pub name: String,
    pub revision_date: String,
    #[serde(default = "default_object")]
    pub object: String,
}

fn default_object() -> String {
    "folder".to_string()
}

impl From<Folder> for FolderResponse {
    fn from(folder: Folder) -> Self {
        FolderResponse {
            id: folder.id,
            name: folder.name,
            revision_date: folder.updated_at,
            object: "folder".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
}
