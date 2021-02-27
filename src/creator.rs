
use async_trait::async_trait;

#[async_trait]
pub trait ServerFn {
    type DeleteResult;
    async fn delete(&self) -> Result<Self::DeleteResult, anyhow::Error>;
}

#[async_trait]
pub trait CreatorFn {
    type Server: ServerFn;
    type Metadata;
    type ServerRequest;

    async fn new(metadata: Self::Metadata) -> Self;
    async fn create(&self, server_request: Self::ServerRequest) -> Result<Self::Server, anyhow::Error>;
}


