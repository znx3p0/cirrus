
use crate::{prelude::{Creator, Server}, providers::digital_ocean::response::Response};
use super::request::Request;
use thiserror::Error;
use serde_json::Error as JsonError;

const URL: &'static str = "https://api.digitalocean.com/v2/droplets";

pub struct DigitalOceanApi   {
    pub(crate) api_key: String,
}

#[derive(Debug, Error)]
pub enum CreateServerError {
    #[error("error sending request: {_0}")]
    SendError(surf::Error),
    #[error("error serializing / deserializing request")]
    SerdeError(surf::Error),
    #[error("error receiving response")]
    ReceivingError(surf::Error),

    #[error("unauthorized access")]
    Unauthorized,
    #[error("unauthorized access")]
    ApiLimit,
    #[error("server error")]
    ServerError,
    #[error("unknown error {0}")]
    Unknown(String),
}

impl DigitalOceanApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn create_server(&self, request: &Request) -> Result<Response, CreateServerError> {
        // println!("{}", serde_json::to_string(&request).unwrap());
        let request = match surf::Body::from_json(&request) {
            Ok(res) => res,
            Err(error) => return Err(CreateServerError::SerdeError(error)),
        };
 
        let mut bytes = match surf::post(URL)
            .body(request)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .await
        {
            Ok(res) => res,
            Err(error) => return Err(CreateServerError::SendError(error)),
        };

        match bytes.status() as u32 {
            401 => return Err(CreateServerError::Unauthorized),
            429 => return Err(CreateServerError::ApiLimit),
            500 => return Err(CreateServerError::ServerError),
            _ => {}
        };

        let bytes = match bytes.body_bytes().await {
            Ok(bytes) => bytes,
            Err(err) => return Err(CreateServerError::ReceivingError(err)),
        };

        let response = match serde_json::from_slice(&bytes) {
            Ok(res) => res,
            Err(_) => return Err(
                CreateServerError::Unknown(
                    std::string::String::from_utf8_lossy(&bytes).to_string()
                )
            ),
        };

        Ok(response)
    }
}


