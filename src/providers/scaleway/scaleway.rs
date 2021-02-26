
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::{CreatorFn, ServerFn};

#[allow(unused)]
const URL: &str = "http://<url>";

pub struct CreatorMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server;

pub struct Creator;

#[async_trait]
impl CreatorFn for Creator {
    type Server = Server;
    type Metadata = CreatorMetadata;
    async fn new(_meta: Self::Metadata) -> Self {
        Creator
    }

    async fn create(&self) -> Result<Self::Server, anyhow::Error> {
        Ok(Server)
    }
}

#[async_trait]
impl ServerFn for Server {
    type DeleteResult = ();
    async fn delete(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}


