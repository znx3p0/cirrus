// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub server: Server,
    pub action: Action,
    pub next_actions: Vec<Action>,
    pub root_password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub id: i64,
    pub command: String,
    pub status: String,
    pub progress: f64,
    pub started: String,
    pub finished: Option<serde_json::Value>,
    pub resources: Vec<Resource>,
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: i64,
    #[serde(rename = "type")]
    pub resource_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub created: String,
    pub public_net: PublicNet,
    pub private_net: Vec<Option<serde_json::Value>>,
    pub server_type: ServerType,
    pub datacenter: Datacenter,
    pub image: Image,
    pub iso: Option<serde_json::Value>,
    pub rescue_enabled: bool,
    pub locked: bool,
    pub backup_window: Option<serde_json::Value>,
    pub outgoing_traffic: Option<serde_json::Value>,
    pub ingoing_traffic: Option<serde_json::Value>,
    pub included_traffic: i64,
    pub protection: ServerProtection,
    pub labels: Labels,
    pub volumes: Vec<Option<serde_json::Value>>,
    pub load_balancers: Vec<Option<serde_json::Value>>,
    pub primary_disk_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datacenter {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub location: Location,
    pub server_types: ServerTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub network_zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTypes {
    pub supported: Vec<i64>,
    pub available: Vec<i64>,
    pub available_for_migration: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: i64,
    #[serde(rename = "type")]
    pub image_type: String,
    pub status: String,
    pub name: String,
    pub description: String,
    pub image_size: Option<serde_json::Value>,
    pub disk_size: i64,
    pub created: String,
    pub created_from: Option<serde_json::Value>,
    pub bound_to: Option<serde_json::Value>,
    pub os_flavor: String,
    pub os_version: String,
    pub rapid_deploy: bool,
    pub protection: ImageProtection,
    pub deprecated: Option<serde_json::Value>,
    pub labels: Labels,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Labels {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageProtection {
    pub delete: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerProtection {
    pub delete: bool,
    pub rebuild: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicNet {
    pub ipv4: Ipv4,
    pub ipv6: Ipv6,
    pub floating_ips: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ipv4 {
    pub ip: String,
    pub blocked: bool,
    pub dns_ptr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ipv6 {
    pub ip: String,
    pub blocked: bool,
    pub dns_ptr: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerType {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub cores: i64,
    pub memory: f64,
    pub disk: i64,
    pub deprecated: Option<serde_json::Value>,
    pub prices: Vec<Price>,
    pub storage_type: String,
    pub cpu_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub location: String,
    pub price_hourly: PriceHourlyClass,
    pub price_monthly: PriceHourlyClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceHourlyClass {
    pub net: String,
    pub gross: String,
}
