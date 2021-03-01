
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[async_trait]
pub trait ServerFn {
    async fn delete<'b>(&self) -> Result<&'b dyn DeleteResult, anyhow::Error>;
    async fn update(&mut self)-> Result<(), anyhow::Error>;
    async fn as_standard_server(&self) -> Result<StandardServer, anyhow::Error>;
    fn needs_update(&self) -> bool;
}

pub trait DeleteResult {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardServer {
    pub ip: String,
    pub id: String,
    pub password: Option<String>,
}

pub trait RequestFn {}

pub trait RequestCreator {
    type Request: RequestFn;

    fn with_name(&self, name: &str) -> Self::Request;
    fn with_prefix(&self, name: &str) -> Self::Request;
}

#[async_trait]
pub trait CreatorFn <'creator, 'server> {
    async fn create(&'static self) -> Result<Box<dyn ServerFn>, anyhow::Error>;
}

impl DeleteResult for () {}

