use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImportCipher {
    #[serde(rename = "type")]
    pub r#type: i32,
    pub folder_id: Option<String>,
    pub organization_id: Option<String>,
    pub name: String,
    pub notes: Option<String>,
    pub favorite: bool,
    pub login: Option<Value>,
    pub card: Option<Value>,
    pub identity: Option<Value>,
    pub secure_note: Option<Value>,
    pub fields: Option<Value>,
    pub password_history: Option<Value>,
    pub reprompt: Option<i32>,
    pub last_known_revision_date: Option<String>,
    pub encrypted_for: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImportFolder {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FolderRelationship {
    pub key: usize,
    pub value: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImportRequest {
    pub ciphers: Vec<ImportCipher>,
    pub folders: Vec<ImportFolder>,
    #[serde(default)]
    pub folder_relationships: Vec<FolderRelationship>,
}
