use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Server {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub user_data: Option<serde_json::Value>,
    pub private_networking: Option<serde_json::Value>,
    pub volumes: Option<serde_json::Value>,
    pub tags: Vec<String>,
}
