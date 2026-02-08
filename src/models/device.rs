use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceID(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub id: String,
    pub user_id: Option<String>,
    pub identifier: String,
    pub push_token: Option<String>,
    pub r#type: i32,
    pub name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeviceType {
    Android = 0,
    iOS = 1,
    ChromeExtension = 2,
    FirefoxExtension = 3,
    OperaExtension = 4,
    EdgeExtension = 5,
    WindowsDesktop = 6,
    MacOsDesktop = 7,
    LinuxDesktop = 8,
    Web = 9,
    VivaldiExtension = 10,
    SafariExtension = 11,
    SDK = 12,
    Server = 13,
    WindowsStoreDesktop = 14,
    BraveExtension = 15,
    TorExtension = 16,
}
