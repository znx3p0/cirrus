

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Server {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub droplet: Option<Droplet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip)]
    pub auth: Option<&'static str>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Droplet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<Kernel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_ids: Option<Vec<Option<serde_json::Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<Vec<Option<serde_json::Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_ids: Option<Vec<Option<serde_json::Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kernel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}


