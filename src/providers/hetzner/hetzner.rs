
use serde::{Deserialize, Serialize};

use super::creator::{CreatorFn, ServerFn};

const HETZNER_URL: &str = "https://api.hetzner.cloud/v1/servers";

#[allow(unused)]
pub mod server_type {
    pub const CX11: &str = "cx11";
    pub const CXP11: &str = "cpx11";

    pub const CX21: &str = "cx21";
    pub const CPX21: &str = "cpx21";

    pub const CX31: &str = "cx31";
    pub const CPX31: &str = "cpx31";

    pub const CX41: &str = "cx41";
    pub const CPX41: &str = "cpx41";

    pub const CX51: &str = "cx51";
    pub const CPX51: &str = "cpx51";
}
#[allow(unused)]
pub mod datacenters {
    pub const NUREMBERG: &str = "nbg1-dc3";
    pub const HELSINKI: &str = "hel1-dc2";
    pub const FALKENSTEIN: &str = "fsn1-dc14";
}
#[allow(unused)]
pub mod images {
    pub const UBUNTU_16_04: &str = "ubuntu-16.04";
    pub const UBUNTU_18_04: &str = "ubuntu-18.04";
    pub const UBUNTU_20_04: &str = "ubuntu-20.04";

    pub const DEBIAN_9: &str = "debian-9";
    pub const DEBIAN_10: &str = "debian-10";

    pub const CENTOS_7: &str = "centos-7";
    pub const CENTOS_8: &str = "centos-8";

    pub const FEDORA_32: &str = "fedora-32";
    pub const FEDORA_33: &str = "fedora-33";
}

pub struct CreatorMetadata {
    pub datacenter: &'static str,
    pub server_type: &'static str,
    pub image: &'static str,
    pub ssh_keys: Option<Vec<String>>,
}

pub struct Server;
pub struct Creator;

impl <T> CreatorFn<T> for Creator {
    type Server = Server;
    fn new(meta: T) -> Self {
        Creator
    }

    fn create(&self) -> Self::Server {
        Server
    }
}

impl ServerFn for Server {
    fn delete(&self) -> Result<(), String> {
        Ok(())
    }
}


