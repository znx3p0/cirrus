
use std::sync::Arc;

use serde::{Deserialize};

use crate::prelude::CreatorFn;

#[derive(Deserialize, Debug)]
pub struct LoadableDynamicCreator {
    pub api_key: String,
    pub provider: String,

    pub distro: Option<String>,
    pub region: Option<String>,
    pub size: Option<String>,
    pub image: Option<String>,
    pub ssh_keys: Option<Vec<String>>,
    pub default: Option<String>,
}

#[derive(Debug)]
pub struct DynamicCreator {
    pub api_key: String,
    pub provider: Provider,
    pub creator: Arc<Box<dyn CreatorFn>>,
}


#[derive(Debug, Clone)]
pub enum Provider {
    DigitalOcean,
    Fake,
}

pub async fn load() -> Result<DynamicCreator, anyhow::Error> {
    let s = std::fs::read_to_string("provider.json")?;
    let dn: LoadableDynamicCreator = serde_json::from_str(&s)?;

    let provider = match dn.provider.as_str() {
        "fake" => Provider::Fake,
        "digital ocean" => Provider::DigitalOcean,
        _ => return Err(anyhow::anyhow!("non-existing provider"))
    };

    let creator: Arc<Box<dyn CreatorFn>> = match &provider {
        Provider::DigitalOcean => {

            let image = match dn.image.unwrap_or_default().as_str() {
                "ubuntu" => crate::providers::digital_ocean::images::UBUNTU_20_04.into(),
                "default" => crate::providers::digital_ocean::images::UBUNTU_20_04.into(),
                "" => crate::providers::digital_ocean::images::UBUNTU_20_04.into(),
                s => s.into(),
            };

            let region = dn.region.unwrap_or(crate::providers::digital_ocean::datacenters::NY1.into());
            let size = dn.size.unwrap_or(crate::providers::digital_ocean::droplets::S1GB_1CPU.into());

            let default = crate::providers::digital_ocean::RequestKind::WithPrefix(dn.default.unwrap_or_default());

            // dn.api_key
            let rqcr = crate::providers::digital_ocean::RqCr {
                region,
                size,
                image,
                ssh_keys: dn.ssh_keys,
                default,
            };
            let p = crate::providers::digital_ocean::Creator::new(&dn.api_key, rqcr).await;
            Arc::new(p)
        }
        Provider::Fake => {
            let p = crate::providers::fake::Creator::new("", crate::providers::fake::RqCr).await;
            Arc::new(p)
        }
    };

    let p = DynamicCreator {
        api_key: dn.api_key.to_string(),
        provider,
        creator,
    };

    Ok(p)
}
