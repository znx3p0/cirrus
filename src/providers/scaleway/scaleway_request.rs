
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_ip_required: Option<String>,
    pub commercial_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Volumes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootscript: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
}

impl ServerRequest {
    pub fn new(name: &str) -> Self {
        let mut server = Self::default();
        server.name = name.into();
        server
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volumes {
    #[serde(rename = "<volumeKey>")]
    pub volume_key: Option<VolumeKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeKey {
    pub id: Option<String>,
    pub name: Option<String>,
    pub size: Option<i64>,
    pub volume_type: Option<String>,
    pub organization: Option<String>,
    pub project: Option<String>,
}
