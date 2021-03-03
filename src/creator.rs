
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[async_trait]
pub trait ServerFn {
    async fn delete(&self) -> Result<Box<dyn DeleteResult>, anyhow::Error>;
    async fn update(&mut self)-> Result<(), anyhow::Error>;
    async fn as_standard_server(&mut self) -> Result<StandardServer, anyhow::Error>;
    fn needs_update(&self) -> bool;
}

pub trait DeleteResult {}
impl DeleteResult for () {}

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



impl std::fmt::Debug for dyn CreatorFn<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn CreatorFn")
    }
}

impl <T> std::fmt::Debug for dyn RequestCreator<Request = T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn RequestCreator<T>")
    }
}

impl std::fmt::Debug for dyn RequestFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn RequestFn")
    }
}

impl std::fmt::Debug for dyn ServerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn ServerFn")
    }
}

impl std::fmt::Debug for dyn DeleteResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn DeleteResult")
    }
}


