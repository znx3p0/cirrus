
use async_trait::async_trait;

#[async_trait]
pub trait ServerFn {
    type DeleteResult;
    async fn delete(&self) -> Result<Self::DeleteResult, anyhow::Error>;
    async fn update(&mut self)-> Result<(), anyhow::Error>;
    async fn as_standard_server(&self) -> Result<StandardServer, anyhow::Error>;
    fn needs_update() -> bool;
}

pub struct StandardServer {
    pub ip: String,
    pub id: String,
    pub password: Option<String>,
}

pub trait Preset {
    fn preset(region: &str, size: &str, image: &str, ssh_keys: Option<Vec<String>>) -> Self;
    fn with_name(self, name: &str) -> Self;
    fn with_prefix(self, name: &str) -> Self;
}

#[async_trait]
pub trait CreatorFn {
    type Server: ServerFn;
    type Metadata;
    type ServerRequest;

    async fn new(metadata: Self::Metadata) -> Self;
    async fn create(&self, server_request: &Self::ServerRequest) -> Result<Self::Server, anyhow::Error>;
}


