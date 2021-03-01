
// use serde::{Deserialize, Serialize};
// use reqwest;
// use async_trait::async_trait;

// use crate::{CreatorFn, Preset, ServerFn};

// #[allow(unused)]
// const URL: &str = "http://localhost:8080";

// pub struct CreatorMetadata;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Server {
//     pub id: String,
//     pub passwd: String,
//     pub ip: String,
// }

// pub struct Creator;

// impl Preset for () {
//     fn preset(&self, _region: &str, _size: &str, _image: &str, _ssh_keys: Option<Vec<String>>) -> Self {()}
//     fn with_name(&self, _name: &str) -> &Self {&()}
//     fn with_prefix(&self, _name: &str) -> &Self {&()}
// }

// impl Creator {
//     pub async fn new() -> Self {
//         Creator
//     }
// }

// #[async_trait]
// impl CreatorFn for Creator {
//     type Server = Server;
//     type Metadata = CreatorMetadata;
//     type ServerRequest = ();

//     async fn create(&self, _: &()) -> Result<Self::Server, anyhow::Error> {
//         println!("Creating server");
//         let res = reqwest::Client::new()
//             .get(&format!("{}/fake/create", URL))
//             .send()
//             .await?
//             .text()
//             .await?;
        
//         let server = serde_json::from_str(&res)?;
//         Ok(server)
//     }
// }

// use crate::StandardServer;

// #[async_trait]
// impl ServerFn for Server {
//     type DeleteResult = ();


//     async fn delete(&self) -> Result<(), anyhow::Error> {
//         println!("Deleting server");
//         reqwest::Client::new()
//         .delete(&format!("{}/fake/delete/{}", URL, self.id))
//             .send()
//             .await?
//             .text()
//             .await?;

//         Ok(())
//     }

//     async fn update(&mut self) -> Result<(), anyhow::Error> {
//         Err(anyhow::anyhow!("Update not implemented for fake"))
//     }

//     async fn as_standard_server(&self) -> Result<StandardServer, anyhow::Error> {

//         Ok(StandardServer {
//             ip: self.ip.clone(),
//             id: self.id.clone(),
//             password: None
//         })
//     }

//     fn needs_update(&self) -> bool { true }
// }


