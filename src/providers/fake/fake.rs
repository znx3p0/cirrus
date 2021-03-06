


use async_trait::async_trait;

use crate::prelude::*;

#[allow(unused)]
const URL: &str = "http://localhost:8080";



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

use crate::StandardServer;


// fn rand_str() -> String {
//     let mut rng = thread_rng();
//     iter::repeat(())
//         .map(|()| rng.sample(Alphanumeric))
//         .map(char::from)
//         .take(8)
//         .collect()
// }

impl Creator {
    pub async fn new(meta: &str, request_creator: RqCr) -> Box<dyn CreatorFn> {
        Box::new(Creator(meta.to_string(), request_creator))
    }
}

#[async_trait]
impl CreatorFn for Creator {
    async fn create(&self) -> Result<Box<dyn ServerFn>, anyhow::Error> {
        let res = reqwest::Client::new()
            .get(&format!("{}/fake/create", URL))
            .send()
            .await?
            .text()
            .await?;
                
        let s: Server = serde_json::from_str(&res)?;
        Ok(Box::new(s))
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
            password: self.password.clone()
        })
    }

    fn needs_update(&self) -> bool { false }
}

