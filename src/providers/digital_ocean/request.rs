
use serde::Serialize;

use super::response::{DropletSize, Images, Regions};

#[derive(Serialize, Clone)]
pub struct Request {
    name: String,
    region: Regions,
    size: DropletSize,
    image: Images,

    ssh_keys: Option<Vec<u32>>,
    backups: Option<bool>,
    ipv6: Option<bool>,
    monitoring: Option<bool>,
    tags: Option<bool>,
    user_data: Option<String>,
    vpc_uuid: Option<String>,
}

macro_rules! delg {
    ($s: ident : $t: ty) => {
        pub fn $s(mut self, $s: $t) -> Self {
            self.$s = Some($s);
            self
        }
    };
    ($($s: ident : $t: ty),*) => {
        $(delg!($s : $t);)*
    };
}

impl Request {
    delg! {
        ssh_keys: Vec<u32>,
        backups: bool,
        ipv6: bool,
        monitoring: bool,
        tags: bool,
        user_data: String,
        vpc_uuid: String
    }
    
    pub fn add_ssh_key(&mut self, key: u32) {
        match &mut self.ssh_keys {
            Some(ssh_keys) => ssh_keys.push(key),
            None => self.ssh_keys = Some(vec![key]),
        }
    }

    pub fn new(name: String, region: Regions, size: DropletSize, image: Images) -> Self {
        Self {
            name, region, size, image,
            ssh_keys: None,
            backups: None,
            ipv6: None,
            monitoring: None,
            tags: None,
            user_data: None,
            vpc_uuid: None
        }
    }
}
