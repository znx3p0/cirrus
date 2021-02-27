

// WARNING:
// DO NOT USE. NOT WORKING.






use async_trait::async_trait;

use crate::{CreatorFn, ServerFn};
use super::scaleway_request::ServerRequest;
use super::scaleway_response::Server;

#[allow(unused)]
const URL: &str = "https://api.scaleway.com/instance/v1/zones/fr-par-1";

pub struct CreatorMetadata {
    pub auth_token: String,
    pub zone: &'static str,
    pub kind: &'static str,
}

pub mod datacenters {
    pub mod france {
        pub const PARIS1: &str = "fr-par-1";
        pub const PARIS2: &str = "fr-par-2";
    }

    pub mod poland {
        pub const WARSAW: &str = "pl-waw-1";
    }
    
    pub mod netherlands {
        pub const AMSTERDAM: &str = "nl-ams-1";
    }
}


pub struct Creator {
    auth_token: String,
    zone: &'static str,
    kind: &'static str,
}


#[async_trait]
impl CreatorFn for Creator {
    type Server = Server;
    type Metadata = CreatorMetadata;
    type ServerRequest = ServerRequest;
    async fn new(meta: Self::Metadata) -> Self {
        Creator {
            auth_token: meta.auth_token,
            zone: meta.zone,
            kind: meta.kind,
        }
    }

    async fn create(&self, mut body: ServerRequest) -> Result<Self::Server, anyhow::Error> {
        body.commercial_type = self.kind.into();

        let text = reqwest::Client::new()
            .post(&format!("https://api.scaleway.com/instance/v1/zones/{}/servers", &self.zone))
            .body(serde_json::to_string(&body)?)
            .header("X-Auth-Token", &self.auth_token)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .text()
            .await?;
        
        println!("{}", &text);
        println!("{}", &format!("{}/instance/v1/zones/{}/servers", URL, &self.zone));
        Ok(serde_json::from_str(&text)?)

    }
}

#[async_trait]
impl ServerFn for Server {
    type DeleteResult = ();
    async fn delete(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}


