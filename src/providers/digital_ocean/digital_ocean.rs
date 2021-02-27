

use async_trait::async_trait;

use crate::{CreatorFn, ServerFn};


// All providers must have the following structs:
//      Server              -> implement ServerFn
//      Creator             -> implement CreatorFn
//      CreatorMetadata
// To see an example implementation take a look at the fake provider

// All providers implement the CreatorFn and ServerFn traits.
// These traits provide an easy-to-use interface.
// The inner workings of the api are not strict, but they should look a bit like this file
// Adding new traits and functions to the standard structures


#[allow(unused)]
const URL: &str = "https://api.digitalocean.com";



// The server struct is obligatory.
// This server struct implements the ServerFn trait, which provides a simple interface
// for interacting with the underlying server.

// The server struct represents a server to be created by the creator,
// and stores information about it, such as the ip, region, etc.
use super::digital_ocean_response::Server;

// The creator struct stores information needed to create new servers with the underlying provider.
// The creator struct implements the ServerFn trait, which provides a simple interface for creating new servers.
#[derive(Debug)]
pub struct Creator(pub &'static str);

use super::digital_ocean_request::Server as Request;
use crate::Preset;

impl Preset for Request {
    fn preset(region: &str, size: &str, image: &str, ssh_keys: Option<Vec<String>>) -> Self {
        let mut req = Request::default();
        req.region = region.into();
        req.size = size.into();
        req.image = image.into();
        req.ssh_keys = ssh_keys;
        req
    }

    fn with_name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    fn with_prefix(mut self, prefix: &str) -> Self {
        self.name = format!("{}{}", prefix, rand_str());
        self
    }
}

use crate::StandardServer;

use std::{borrow::Borrow, iter};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn rand_str() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(8)
        .collect()
}

pub mod datacenters {
    pub const NY1: &str = "nyc1";
    pub const NY2: &str = "nyc2";
    pub const NY3: &str = "nyc3";
    pub const AMSTERDAM1: &str = "ams1";
    pub const AMSTERDAM2: &str = "ams2";
    pub const AMSTERDAM3: &str = "ams3";
    pub const SF1: &str = "sfo1";
    pub const SF2: &str = "sfo2";
    pub const SF3: &str = "sfo3";
    pub const SINGAPORE: &str = "sgp1";
    pub const LONDON: &str = "lon1";
    pub const FRANKFURT: &str = "fra1";
    pub const TORONTO: &str = "tor1";
    pub const BANGALORE: &str = "blr1";
}

pub mod droplets {
    pub const S1GB_1CPU: &str = "s-1vcpu-1gb";
    pub const S2GB_1CPU: &str = "s-1vcpu-2gb";
    pub const S2GB_2CPU: &str = "s-2vcpu-2gb";
    pub const S2GB_4CPU: &str = "s-2vcpu-4gb";
    pub const S4GB_8CPU: &str = "s-4vcpu-8gb";
}

pub mod images {
    pub const UBUNTU_16_04: &str = "ubuntu-16-04-x64";
    pub const UBUNTU_18_04: &str = "ubuntu-18-04-x64";
    pub const UBUNTU_20_04: &str = "ubuntu-20-04-x64";
}

#[async_trait]
impl CreatorFn for Creator {
    type Server = Server;
    type Metadata = &'static str;
    type ServerRequest = Request;

    async fn new(meta: Self::Metadata) -> Self {
        Creator(meta)
    }

    async fn create(&self, rq: &Self::ServerRequest) -> Result<Self::Server, anyhow::Error> {
        let res = reqwest::Client::new()
            .post(&format!("{}/v2/droplets", URL))
            .header("Authorization", &format!("Bearer {}", self.0))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&rq)?)
            .send()
            .await?
            .text()
            .await?;
        
        let mut s: Server = serde_json::from_str(&res)?;
        s.auth = Some(&self.0);
        Ok(s)
    }
}

#[async_trait]
impl ServerFn for Server {
    type DeleteResult = ();
    async fn delete(&self) -> Result<(), anyhow::Error> {
        let res = reqwest::Client::new()
            .delete(&format!("{}/v2/droplets/{}", URL, self.droplet.as_ref().unwrap().id.unwrap()))
            .header("Authorization", &format!("Bearer {}", self.auth.as_ref().unwrap()))
            .header("Content-Type", "application/json")
            .send()
            .await?
            .text()
            .await?;
        
        println!("{}", res);
        Ok(())
    }

    async fn update(&mut self) -> Result<(), anyhow::Error> {
        let text = reqwest::Client::new()
            .get(&format!("{}/v2/droplets/{}", URL, self.droplet.as_ref().unwrap().id.unwrap()))
            .header("Authorization", &format!("Bearer {}", self.auth.unwrap()))
            .send()
            .await?
            .text()
            .await?;
        
        let server = serde_json::from_str::<Server>(&text)?;
        self.droplet = server.droplet;
        self.links = server.links;
        Ok(())
    }

    async fn as_standard_server(&self) -> Result<StandardServer, anyhow::Error> {

        let ip = match self.droplet.as_ref() {
            Some(s) => {
                match s.networks.as_ref() {
                    Some(s) => {
                        match s.v4.as_ref() {
                            Some(s) => {
                                match s.first() {
                                    Some(s) => match s {
                                        Some(s) => match s.ip_address.borrow() {
                                            Some(s) => {
                                                s.clone()
                                            },
                                            None => return Err(anyhow::anyhow!("No ip address found"))
                                        }
                                        None => return Err(anyhow::anyhow!("No ipv4 networks found"))
                                    }
                                    None => return Err(anyhow::anyhow!("No networks found"))
                                }
                            },
                            None => return Err(anyhow::anyhow!("No networks found"))
                        }
                    },
                    None => return Err(anyhow::anyhow!("No networks found"))
                }
            },
            None => return Err(anyhow::anyhow!("No droplet found"))
        };

        let id = match self.droplet.as_ref().unwrap().id {
            Some(id) => id.to_string(),
            None => return Err(anyhow::anyhow!("No id found"))
        };

        Ok(StandardServer {
            ip: ip,
            id: id,
            password: None
        })
    }

    fn needs_update() -> bool { true }
}

