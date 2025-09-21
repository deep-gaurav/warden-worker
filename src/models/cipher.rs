use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Map, Value};

// This struct represents the data stored in the `data` column of the `ciphers` table.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CipherData {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_note: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_history: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<i32>,
}

// Custom deserialization function for booleans
fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    // A visitor is used to handle different data types
    struct BoolOrIntVisitor;

    impl<'de> de::Visitor<'de> for BoolOrIntVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or an integer 0 or 1")
        }

        // Handles boolean values
        fn visit_bool<E>(self, value: bool) -> Result<bool, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        // Handles integer values (0 or 1)
        fn visit_u64<E>(self, value: u64) -> Result<bool, E>
        where
            E: de::Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(de::Error::invalid_value(
                    de::Unexpected::Unsigned(value),
                    &"0 or 1",
                )),
            }
        }
    }

    deserializer.deserialize_any(BoolOrIntVisitor)
}

// The struct that is stored in the database and used in handlers.
// For serialization to JSON for the client, we implement a custom `Serialize`.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cipher {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: i32,
    pub data: Value,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub favorite: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,

    // Bitwarden specific field for API responses
    #[serde(default = "default_object")]
    pub object: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub organization_use_totp: bool,
    #[serde(default = "default_true")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub edit: bool,
    #[serde(default = "default_true")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub view_password: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CipherDBModel {
    pub id: String,
    pub user_id: String,
    pub organization_id: Option<String>,
    pub r#type: i32,
    pub data: String,
    pub favorite: i32,
    pub folder_id: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Into<Cipher> for CipherDBModel {
    fn into(self) -> Cipher {
        Cipher {
            id: self.id,
            user_id: Some(self.user_id),
            organization_id: self.organization_id,
            r#type: self.r#type,
            data: serde_json::from_str(&self.data).unwrap_or_default(),
            favorite: match self.favorite {
                0 => false,
                _ => true,
            },
            folder_id: self.folder_id,
            deleted_at: self.deleted_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
            object: "default_object".to_string(),
            organization_use_totp: false,
            edit: true,
            view_password: true,
            collection_ids: None,
        }
    }
}

impl Serialize for Cipher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut response_map = Map::new();

        response_map.insert("object".to_string(), json!(self.object));
        response_map.insert("id".to_string(), json!(self.id));
        if self.user_id.is_some() {
            response_map.insert("userId".to_string(), json!(self.user_id));
        }
        response_map.insert("organizationId".to_string(), json!(self.organization_id));
        response_map.insert("folderId".to_string(), json!(self.folder_id));
        response_map.insert("type".to_string(), json!(self.r#type));
        response_map.insert("favorite".to_string(), json!(self.favorite));
        response_map.insert("edit".to_string(), json!(self.edit));
        response_map.insert("viewPassword".to_string(), json!(self.view_password));
        response_map.insert(
            "organizationUseTotp".to_string(),
            json!(self.organization_use_totp),
        );
        response_map.insert("collectionIds".to_string(), json!(self.collection_ids));
        response_map.insert("revisionDate".to_string(), json!(self.updated_at));
        response_map.insert("creationDate".to_string(), json!(self.created_at));
        response_map.insert("deletedDate".to_string(), json!(self.deleted_at));

        log::debug!("Response data is {:?}", self.data);
        if let Some(data_obj) = self.data.as_object() {
            let data_clone = data_obj.clone();

            response_map.insert(
                "name".to_string(),
                data_clone.get("name").cloned().unwrap_or(Value::Null),
            );
            response_map.insert(
                "notes".to_string(),
                data_clone.get("notes").cloned().unwrap_or(Value::Null),
            );
            response_map.insert(
                "fields".to_string(),
                data_clone.get("fields").cloned().unwrap_or(Value::Null),
            );
            response_map.insert(
                "passwordHistory".to_string(),
                data_clone
                    .get("passwordHistory")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
            response_map.insert(
                "reprompt".to_string(),
                data_clone
                    .get("reprompt")
                    .cloned()
                    .unwrap_or(Value::Number(serde_json::Number::from_f64(0.0).unwrap())),
            );

            let mut login = Value::Null;
            let mut secure_note = Value::Null;
            let mut card = Value::Null;
            let mut identity = Value::Null;

            match self.r#type {
                1 => login = data_clone.get("login").cloned().unwrap_or(Value::Null),
                2 => secure_note = data_clone.get("secureNote").cloned().unwrap_or(Value::Null),
                3 => card = data_clone.get("card").cloned().unwrap_or(Value::Null),
                4 => identity = data_clone.get("identity").cloned().unwrap_or(Value::Null),
                _ => {}
            }

            response_map.insert("login".to_string(), login);
            response_map.insert("secureNote".to_string(), secure_note);
            response_map.insert("card".to_string(), card);
            response_map.insert("identity".to_string(), identity);
        } else {
            response_map.insert("name".to_string(), Value::Null);
            response_map.insert("notes".to_string(), Value::Null);
            response_map.insert("fields".to_string(), Value::Null);
            response_map.insert("passwordHistory".to_string(), Value::Null);
            response_map.insert("reprompt".to_string(), Value::Null);
            response_map.insert("login".to_string(), Value::Null);
            response_map.insert("secureNote".to_string(), Value::Null);
            response_map.insert("card".to_string(), Value::Null);
            response_map.insert("identity".to_string(), Value::Null);
        }

        Value::Object(response_map).serialize(serializer)
    }
}

fn default_object() -> String {
    "cipher".to_string()
}

fn default_true() -> bool {
    true
}

// Represents the "Cipher" object within the incoming request payload.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CipherRequestData {
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub favorite: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_note: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_history: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_revision_date: Option<String>,
}

// Represents the full request payload for creating a cipher.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCipherRequest {
    pub cipher: CipherRequestData,
    #[serde(default)]
    pub collection_ids: Vec<String>,
}
