

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::{CreatorFn, ServerFn};


// All providers must have the following structs:
//      Server              -> implement ServerFn
//      Creator             -> implement CreatorFn
//      CreatorMetadata
// To see an example implementation take a look at the fake provider

// All providers implement the CreatorFn and ServerFn traits.
// These traits provide an easy-to-use interface.
// The inner workings of the api are not strict, but they should look a bit like this file
// Adding new traits and functions to the standard structures


#[allow(unused)]
const URL: &str = "http://<url>";

pub struct CreatorMetadata;


// The server struct is obligatory.
// This server struct implements the ServerFn trait, which provides a simple interface
// for interacting with the underlying server.

// The server struct represents a server to be created by the creator,
// and stores information about it, such as the ip, region, etc.
#[derive(Debug, Deserialize)]
pub struct Server;

// The creator struct stores information needed to create new servers with the underlying provider.
// The creator struct implements the ServerFn trait, which provides a simple interface for creating new servers.
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

