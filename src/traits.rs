
use std::io::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Creator {
    type Request;
    type Server: Server;
    async fn create_server(&self, request: &Self::Request) -> Result<Self::Server>;
}

#[async_trait]
pub trait Server {
    type Status;
    async fn delete_server(self) -> Result<Self::Status>;
}
