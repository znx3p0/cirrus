



use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commercial_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_ip_required: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<PublicIp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Ipv6>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootscript: Option<Bootscript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Volumes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<SecurityGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenances: Option<Vec<Maintenance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<PlacementGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_nics: Option<Vec<PrivateNic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bootscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootcmdargs: Option<String>,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootscript_default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bootscript: Option<Bootscript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_volumes: Option<ExtraVolumes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume: Option<RootVolume>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraVolumes {
    #[serde(rename = "<extra_volumeKey>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_volume_key: Option<VolumeKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<SecurityGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootVolume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ipv6 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_respected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateNic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_network_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicIp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volumes {
    #[serde(rename = "<volumeKey>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_key: Option<VolumeKey>,
}
