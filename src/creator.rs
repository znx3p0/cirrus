use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardServer {
    pub ip: String,
    pub id: String,
    pub password: Option<String>,
}
pub mod prelude {
    use crate::*;
    use async_trait::async_trait;

    #[async_trait]
    pub trait CreatorFn: Send + Sync {
        async fn create(&self) -> Result<Box<dyn ServerFn + Send + Sync>, anyhow::Error>;
    }

    pub trait DeleteResult {}
    impl DeleteResult for () {}

    pub trait RequestFn {}

    pub trait RequestCreator {
        type Request: RequestFn;

        fn with_name(&self, name: &str) -> Self::Request;
        fn with_prefix(&self, name: &str) -> Self::Request;
    }

    #[async_trait]
    pub trait ServerFn {
        async fn delete(&self) -> Result<Box<dyn DeleteResult>, anyhow::Error>;
        async fn update(&mut self) -> Result<(), anyhow::Error>;
        async fn as_standard_server(&mut self) -> Result<StandardServer, anyhow::Error>;
        fn needs_update(&self) -> bool;
    }
}

use prelude::*;

impl std::fmt::Debug for dyn CreatorFn + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn CreatorFn")
    }
}

impl<T> std::fmt::Debug for dyn RequestCreator<Request = T> + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn RequestCreator<T>")
    }
}

impl std::fmt::Debug for dyn RequestFn + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn RequestFn")
    }
}

impl std::fmt::Debug for dyn ServerFn + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn ServerFn")
    }
}

impl std::fmt::Debug for dyn DeleteResult + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn DeleteResult")
    }
}
