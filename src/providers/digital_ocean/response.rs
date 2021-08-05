
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Response {
    pub droplet: Droplet,
    pub links: Links,
}

#[derive(Deserialize)]
pub struct Droplet {
    pub id: i64,
    pub name: String,
    pub memory: i64,
    pub vcpus: i64,
    pub disk: i64,
    pub locked: bool,
    pub status: Status,
    pub created_at: String,
    pub features: Vec<String>,
    pub backup_ids: Vec<u32>,
    pub snapshot_ids: Vec<u32>,
    pub image: Image,
    pub volume_ids: Vec<String>,
    pub size: Size,
    pub size_slug: String,
    pub networks: Networks,
    pub region: Region,
    pub tags: Vec<String>,
    
    pub next_backup_window: Option<NextBackupWindow>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DropletSize {
    #[serde(rename = "s-1vcpu-1gb")]
    Basic1C1G,
    #[serde(rename = "s-1vcpu-1gb-amd")]
    BasicAMD1C1G,
    #[serde(rename = "s-1vcpu-1gb-intel")]
    BasicIntel1C1G,
    #[serde(rename = "s-1vcpu-2gb")]
    Basic1C2G,
    #[serde(rename = "s-1vcpu-2gb-amd")]
    BasicAMD1C2G,
    #[serde(rename = "s-1vcpu-2gb-intel")]
    BasicIntel1C2G,
    #[serde(rename = "s-2vcpu-2gb")]
    LegacyBasic2C2G,
    #[serde(rename = "s-2vcpu-2gb-amd")]
    BasicAMD2C2G,
    #[serde(rename = "s-2vcpu-2gb-intel")]
    BasicIntel2C2G,
    #[serde(rename = "s-2vcpu-4gb")]
    Basic2C4G,
    #[serde(rename = "s-2vcpu-4gb-amd")]
    BasicAMD2C4G,
    #[serde(rename = "s-2vcpu-4gb-intel")]
    BasicIntel2C4G,
    #[serde(rename = "s-4vcpu-8gb")]
    Basic4C8G,
    #[serde(rename = "c-2")]
    CPUOptimizedC2,
    #[serde(rename = "c2-2vcpu-4gb")]
    CPUOptimized2xSSD2C4G,
    #[serde(rename = "s-4vcpu-8gb-amd")]
    BasicAMD4C8G,
    #[serde(rename = "s-4vcpu-8gb-intel")]
    BasicIntel4C8G,
    #[serde(rename = "g-2vcpu-8gb")]
    GeneralPurpose2C8G,
    #[serde(rename = "gd-2vcpu-8gb")]
    GeneralPurpose2xSSD2C8G,
    #[serde(rename = "s-8vcpu-16gb")]
    Basic8C16G,
    #[serde(rename = "m-2vcpu-16gb")]
    MemoryOptimized2C16G,
    #[serde(rename = "c-4")]
    CPUOptimizedC4,
    #[serde(rename = "c2-4vcpu-8gb")]
    CPUOptimized2xSSD4C8G,
    #[serde(rename = "s-8vcpu-16gb-amd")]
    BasicAMD8C16G,
    #[serde(rename = "s-8vcpu-16gb-intel")]
    BasicIntel8C16G,
    #[serde(rename = "m3-2vcpu-16gb")]
    MemoryOptimized3xSSD2C16G,
    #[serde(rename = "g-4vcpu-16gb")]
    GeneralPurpose4C16G,
    #[serde(rename = "so-2vcpu-16gb")]
    StorageOptimized2C16G,
    #[serde(rename = "m6-2vcpu-16gb")]
    MemoryOptimized6xSSD2C16G,
    #[serde(rename = "gd-4vcpu-16gb")]
    GeneralPurpose2xSSD4C16G,
    #[serde(rename = "so1_5-2vcpu-16gb")]
    StorageOptimizedOnePointFiveXSSD2C16G,
    #[serde(rename = "m-4vcpu-32gb")]
    MemoryOptimized4C32G,
    #[serde(rename = "c-8")]
    CPUOptimizedC8,
    #[serde(rename = "c2-8vcpu-16gb")]
    CPUOptimized2xSSD8C16G,
    #[serde(rename = "m3-4vcpu-32gb")]
    MemoryOptimized3xSSD4C32G,
    #[serde(rename = "g-8vcpu-32gb")]
    GeneralPurpose8C32G,
    #[serde(rename = "so-4vcpu-32gb")]
    StorageOptimized4C32G,
    #[serde(rename = "m6-4vcpu-32gb")]
    MemoryOptimized6xSSD4C32G,
    #[serde(rename = "gd-8vcpu-32gb")]
    GeneralPurpose2xSSD8C32G,
    #[serde(rename = "so1_5-4vcpu-32gb")]
    StorageOptimizedOnePointFiveXSSD4C32G,
    #[serde(rename = "m-8vcpu-64gb")]
    MemoryOptimized8C64G,
    #[serde(rename = "c-16")]
    LegacyCPUOptimizedC16,
    #[serde(rename = "c2-16vcpu-32gb")]
    CPUOptimized2xSSD16C32G,
    #[serde(rename = "m3-8vcpu-64gb")]
    MemoryOptimized3xSSD8C64G,
    #[serde(rename = "g-16vcpu-64gb")]
    GeneralPurpose16C64G,
    #[serde(rename = "so-8vcpu-64gb")]
    StorageOptimized8C64G,
    #[serde(rename = "m6-8vcpu-64gb")]
    MemoryOptimized6xSSD8C64G,
    #[serde(rename = "gd-16vcpu-64gb")]
    GeneralPurpose2xSSD16C64G,
    #[serde(rename = "so1_5-8vcpu-64gb")]
    StorageOptimizedOnePointFiveXSSD8C64G,
    #[serde(rename = "m-16vcpu-128gb")]
    MemoryOptimized16C128G,
    #[serde(rename = "c-32")]
    CPUOptimizedC32,
    #[serde(rename = "c2-32vcpu-64gb")]
    CPUOptimized2xSSD32C64G,
    #[serde(rename = "m3-16vcpu-128gb")]
    MemoryOptimized3xSSD16C128G,
    #[serde(rename = "m-24vcpu-192gb")]
    MemoryOptimized24C192G,
    #[serde(rename = "g-32vcpu-128gb")]
    GeneralPurpose32C128G,
    #[serde(rename = "so-16vcpu-128gb")]
    StorageOptimized16C128G,
    #[serde(rename = "m6-16vcpu-128gb")]
    MemoryOptimized6xSSD16C128G,
    #[serde(rename = "gd-32vcpu-128gb")]
    GeneralPurpose2xSSD32C128G,
    #[serde(rename = "m3-24vcpu-192gb")]
    MemoryOptimized3xSSD24C192G,
    #[serde(rename = "g-40vcpu-160gb")]
    GeneralPurpose40C160G,
    #[serde(rename = "so1_5-16vcpu-128gb")]
    StorageOptimizedOnePointFiveXSSD16C128G,
    #[serde(rename = "m-32vcpu-256gb")]
    MemoryOptimized32C256G,
    #[serde(rename = "gd-40vcpu-160gb")]
    GeneralPurpose2xSSD40C160G,
    #[serde(rename = "so-24vcpu-192gb")]
    StorageOptimized24C192G,
    #[serde(rename = "m6-24vcpu-192gb")]
    MemoryOptimized6xSSD24C192G,
    #[serde(rename = "m3-32vcpu-256gb")]
    MemoryOptimized3xSSD32C256G,
    #[serde(rename = "so1_5-24vcpu-192gb")]
    StorageOptimizedOnePointFiveXSSD24C192G,
    #[serde(rename = "so-32vcpu-256gb")]
    StorageOptimized32C256G,
    #[serde(rename = "m6-32vcpu-256gb")]
    MemoryOptimized6xSSD32C256G,
    #[serde(rename = "so1_5-32vcpu-256gb")]
    StorageOptimizedOnePointFiveXSSD32C256G,
}

#[derive(Deserialize)]
pub enum Status {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "archive")]
    Archive
}

#[derive(Deserialize)]
pub enum ImageType {
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Deserialize)]
pub struct NextBackupWindow {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize)]
pub enum Distribution {
    #[serde(rename = "Arch Linux")]
    ArchLinux,
    CentOS,
    CoreOS,
    Debian,
    Fedora,
    #[serde(rename = "Fedora Atomic")]
    FedoraAtomic,
    FreeBSD,
    Gentoo,
    #[serde(rename = "openSUSE")]
    OpenSUSE,
    RancherOS,
    Ubuntu,
    Unknown,
}

#[derive(Deserialize)]
pub struct Image {
    pub id: i64,
    pub name: String,
    pub distribution: Distribution,
    pub slug: Option<String>,
    pub public: bool,
    pub regions: Vec<Regions>,
    pub created_at: String,
    #[serde(rename = "type")]
    pub image_type: ImageType,
    pub description: String,
    pub tags: Option<Vec<String>>,
    pub status: String,

    pub min_disk_size: Option<i64>,
    pub size_gigabytes: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub enum Images {
    #[serde(rename = "85341905")]
    CentOSStream8x64,
    #[serde(rename = "85722003")]
    CentOS7x64,
    #[serde(rename = "85779954")]
    CentOS8x64,
    #[serde(rename = "85779974")]
    Debian9x64,
    #[serde(rename = "86718194")]
    Debian10x64,
    #[serde(rename = "84780898")]
    Fedora34x64,
    #[serde(rename = "85780245")]
    Fedora33x64,
    #[serde(rename = "69452245")]
    FreeBSD11x64zfs,
    #[serde(rename = "69500386")]
    FreeBSD11x64ufs,
    #[serde(rename = "77558491")]
    FreeBSD12x64ufs,
    #[serde(rename = "77558552")]
    FreeBSD12x64zfs,
    #[serde(rename = "78547182")]
    RancherOS,
    #[serde(rename = "89052313")]
    RockyLinux8x64,
    #[serde(rename = "84780478")]
    Ubuntu2004x64,
    #[serde(rename = "85779928")]
    Ubuntu1804x64,
    #[serde(rename = "85781040")]
    Ubuntu2104x64,
    #[serde(rename = "85785636")]
    Ubuntu2010x64,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Regions {
    #[serde(rename = "ams1")]
    Ams1,
    #[serde(rename = "ams2")]
    Ams2,
    #[serde(rename = "ams3")]
    Ams3,
    #[serde(rename = "blr1")]
    Blr1,
    #[serde(rename = "fra1")]
    Fra1,
    #[serde(rename = "lon1")]
    Lon1,
    #[serde(rename = "nyc1")]
    Nyc1,
    #[serde(rename = "nyc2")]
    Nyc2,
    #[serde(rename = "nyc3")]
    Nyc3,
    #[serde(rename = "sfo1")]
    Sfo1,
    #[serde(rename = "sfo2")]
    Sfo2,
    #[serde(rename = "sfo3")]
    Sfo3,
    #[serde(rename = "sgp1")]
    Sgp1,
    #[serde(rename = "tor1")]
    Tor1,
}

#[derive(Deserialize)]
pub struct Networks {
    pub v4: Vec<Vec<V4>>,
    pub v6: Vec<Vec<V6>>,
}

#[derive(Deserialize)]
pub struct Region {
    pub name: String,
    pub slug: String,
    pub features: Vec<String>,
    pub available: bool,
    pub sizes: Vec<String>,
}

#[derive(Deserialize)]
pub struct V4 {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,

    #[serde(rename = "type")]
    pub network_type: String,
}

#[derive(Deserialize)]
pub struct V6 {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,

    #[serde(rename = "type")]
    pub network_type: String,
}

#[derive(Deserialize)]
pub struct Size {
    pub slug: String,
    pub memory: i64,
    pub vcpus: i64,
    pub disk: i64,
    pub transfer: f64,
    pub price_monthly: f64,
    pub price_hourly: f64,
    pub regions: Vec<String>,
    pub available: bool,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Links {
    pub actions: Vec<Action>,
}

#[derive(Deserialize)]
pub struct Action {
    pub id: i64,
    pub rel: String,
    pub href: String,
}
