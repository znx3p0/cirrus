use async_trait::async_trait;

use crate::prelude::*;

const URL: &str = "http://localhost:8080";
use crate::StandardServer;

// The server struct is obligatory.
// This server struct implements the ServerFn trait, which provides a simple interface
// for interacting with the underlying server.

// The server struct represents a server to be created by the creator,
// and stores information about it, such as the ip, region, etc.
use crate::StandardServer as Server;

// The creator struct stores information needed to create new servers with the underlying provider.
// The creator struct implements the ServerFn trait, which provides a simple interface for creating new servers.
#[derive(Debug)]
pub struct Creator(pub String, pub RqCr);

type Request = ();

#[derive(Debug)]
pub struct RqCr;

impl RequestFn for Request {}

impl RequestCreator for RqCr {
    type Request = Request;

    fn with_name(&self, _name: &str) -> Self::Request {
        ()
    }

    fn with_prefix(&self, _prefix: &str) -> Self::Request {
        ()
    }
}

impl Creator {
    pub async fn new(meta: &str, request_creator: RqCr) -> Box<dyn CreatorFn + Send + Sync> {
        Box::new(Creator(meta.to_string(), request_creator))
    }
}

#[async_trait]
impl CreatorFn for Creator {
    async fn create(&self) -> Result<Box<dyn ServerFn + Send + Sync>, anyhow::Error> {
        let res = reqwest::Client::new()
            .get(&format!("{}/fake/create", URL))
            .send()
            .await?
            .text()
            .await?;

        let s: Server = serde_json::from_str(&res)?;
        Ok(Box::new(s))
    }

    async fn from_serializable(&self, s: String) -> Result<Box<dyn ServerFn + Send + Sync>, anyhow::Error> {
        match serde_json::from_str::<Server>(&s) {
            Ok(s) => Ok(Box::new(s)),
            Err(e) => Err(anyhow::anyhow!(e))
        }
    }
}

#[async_trait]
impl ServerFn for Server {
    async fn delete(&self) -> Result<Box<dyn DeleteResult>, anyhow::Error> {
        reqwest::Client::new()
            .delete(&format!("{}/fake/delete/{}", URL, self.id))
            .send()
            .await?
            .text()
            .await?;

        Ok(Box::new(()))
    }

    async fn update(&mut self) -> Result<(), anyhow::Error> {
        Err(anyhow::anyhow!("Update not implemented for fake api"))
    }

    async fn as_standard_server(&mut self) -> Result<StandardServer, anyhow::Error> {
        Ok(StandardServer {
            ip: self.ip.clone(),
            id: self.id.clone(),
            password: self.password.clone(),
        })
    }

    fn needs_update(&self) -> bool {
        false
    }

    fn as_serializable(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string(self).unwrap())
    }
}
