
use serde::{Deserialize, Serialize};
use reqwest;
use async_trait::async_trait;

use crate::{CreatorFn, ServerFn};

#[allow(unused)]
const URL: &str = "http://localhost:8080";

pub struct CreatorMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub passwd: String,
    pub ip: String,
}

pub struct Creator;

#[async_trait]
impl CreatorFn for Creator {
    type Server = Server;
    type Metadata = CreatorMetadata;
    type ServerRequest = ();
    async fn new(_meta: Self::Metadata) -> Self {
        Creator
    }

    async fn create(&self, _: &()) -> Result<Self::Server, anyhow::Error> {
        println!("Creating server");
        let res = reqwest::Client::new()
            .get(&format!("{}/fake/create", URL))
            .send()
            .await?
            .text()
            .await?;
        
        let server = serde_json::from_str(&res)?;
        Ok(server)
    }
}

#[async_trait]
impl ServerFn for Server {
    type DeleteResult = ();

    async fn delete(&self) -> Result<(), anyhow::Error> {
        println!("Deleting server");
        reqwest::Client::new()
        .delete(&format!("{}/fake/delete/{}", URL, self.id))
            .send()
            .await?
            .text()
            .await?;

        Ok(())
    }

    async fn update(&mut self) -> Result<(), anyhow::Error> {
        Err(anyhow::anyhow!("Update not implemented for fake"))
    }
}


