use async_trait::async_trait;
use serde_json::to_string;

use crate::prelude::*;

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
use super::digital_ocean_response::LoadableServer;

// The creator struct stores information needed to create new servers with the underlying provider.
// The creator struct implements the ServerFn trait, which provides a simple interface for creating new servers.
#[derive(Debug)]
pub struct Creator(
    pub Arc<String>, /* api key */
    pub RqCr,        /* request creator */
);

use super::digital_ocean_request::Server as Request;

#[derive(Debug)]
pub struct RqCr {
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub default: RequestKind,
}

#[derive(Debug)]
pub enum RequestKind {
    WithName(String),
    WithPrefix(String),
}

impl RequestFn for Request {}

impl RequestCreator for RqCr {
    type Request = Request;

    fn with_name(&self, name: &str) -> Self::Request {
        let mut rq = Request::default();
        rq.region = self.region.to_string();
        rq.name = name.to_string();
        rq.image = self.image.to_string();
        rq.size = self.size.to_string();
        rq.ssh_keys = self.ssh_keys.clone();
        rq
    }

    fn with_prefix(&self, prefix: &str) -> Self::Request {
        let mut rq = Request::default();
        rq.region = self.region.to_string();
        rq.name = format!("{}{}", prefix, rand_str());
        rq.image = self.image.to_string();
        rq.size = self.size.to_string();
        rq.ssh_keys = self.ssh_keys.clone();
        rq
    }
}

use crate::StandardServer;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{iter, sync::Arc};

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
impl Creator {
    pub async fn new(meta: &str, request_creator: RqCr) -> Box<dyn CreatorFn + Send + Sync> {
        Box::new(Creator(Arc::new(meta.to_string()), request_creator))
    }
}

#[async_trait]
impl CreatorFn for Creator {
    async fn create(&self) -> Result<Box<dyn ServerFn + Send + Sync>, anyhow::Error> {
        let req = match &self.1.default {
            RequestKind::WithName(name) => self.1.with_name(&name),
            RequestKind::WithPrefix(prefix) => {
                self.1.with_prefix(&format!("{}{}", prefix, rand_str()))
            }
        };
        let str = serde_json::to_string::<Request>(&req)?;
        let req = reqwest::Client::new()
            .post(&format!("{}/v2/droplets", URL))
            .header("Authorization", &format!("Bearer {}", self.0))
            .header("Content-Type", "application/json")
            .body(str);
        let res = req.send().await?.text().await?;

        let mut s: Server = serde_json::from_str(&res)?;
        s.auth = Some(self.0.clone());
        Ok(Box::new(s))
    }

    async fn from_serializable(&self, s: String) -> Result<Box<dyn ServerFn + Send + Sync>, anyhow::Error> {

        let p = match serde_json::from_str::<LoadableServer>(&s) {
            Ok(s) => s,
            Err(e) => return Err(anyhow::anyhow!(e))
        };

        Ok(Box::new(Server {
            droplet: p.droplet,
            links: p.links,
            auth: Some(Arc::new(p.auth.unwrap())),
        }))
    }
}

#[async_trait]
impl ServerFn for Server {
    async fn delete(&self) -> Result<Box<dyn DeleteResult>, anyhow::Error> {
        let _ = reqwest::Client::new()
            .delete(&format!(
                "{}/v2/droplets/{}",
                URL,
                self.droplet.as_ref().unwrap().id.unwrap()
            ))
            .header(
                "Authorization",
                &format!("Bearer {}", self.auth.as_ref().unwrap()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?
            .text()
            .await?;

        Ok(Box::new(()))
    }

    async fn update(&mut self) -> Result<(), anyhow::Error> {
        let text = reqwest::Client::new()
            .get(&format!(
                "{}/v2/droplets/{}",
                URL,
                self.droplet.as_ref().unwrap().id.unwrap()
            ))
            .header(
                "Authorization",
                &format!("Bearer {}", self.auth.as_ref().unwrap()),
            )
            .send()
            .await?
            .text()
            .await?;

        let server = serde_json::from_str::<Server>(&text)?;
        self.droplet = server.droplet;
        self.links = server.links;
        Ok(())
    }

    async fn as_standard_server(&mut self) -> Result<StandardServer, anyhow::Error> {
        let ip = match self.droplet.as_ref() {
            Some(s) => match s.networks.as_ref() {
                Some(s) => match s.v4.as_ref() {
                    Some(s) => {
                        let s = s
                            .into_iter()
                            .filter_map(|s| Some(s.as_ref().unwrap()))
                            .collect::<Vec<_>>();

                        let s = s
                            .into_iter()
                            .filter(|s| s.server_type.as_ref().unwrap() == "public")
                            .collect::<Vec<_>>();

                        match s.first().as_ref() {
                            Some(s) => match s.ip_address.as_ref() {
                                Some(s) => s.clone(),
                                None => return Err(anyhow::anyhow!("No ip address found")),
                            },
                            None => return Err(anyhow::anyhow!("No ip addresses found")),
                        }
                    }
                    None => return Err(anyhow::anyhow!("No networks found")),
                },
                None => return Err(anyhow::anyhow!("No networks found")),
            },
            None => return Err(anyhow::anyhow!("No droplet found")),
        };

        let id = match self.droplet.as_ref().unwrap().id {
            Some(id) => id.to_string(),
            None => return Err(anyhow::anyhow!("No id found")),
        };

        Ok(StandardServer {
            ip,
            id,
            password: None,
        })
    }

    fn needs_update(&self) -> bool {
        true
    }

    fn as_serializable(&self) -> Result<String, anyhow::Error> {
        println!("{}", self.auth.clone().unwrap());

        let auth = self.auth.as_ref().and_then(|s| Some(s.to_string()));
        let lb = LoadableServer {
            droplet: self.droplet.clone(),
            links: self.links.clone(),
            auth,
        };

        match serde_json::to_string::<LoadableServer>(&lb) {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e))
        }
    }

}

