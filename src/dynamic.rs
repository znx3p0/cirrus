
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct LoadableDynamicCreator {
    pub api_key: String,
    pub provider: String
}

#[derive(Debug)]
pub struct DynamicCreator {
    pub api_key: String,
    pub provider: Provider
}

#[derive(Debug)]
pub enum Provider {
    DigitalOcean,
    Fake,
}

pub fn load() -> Result<DynamicCreator, anyhow::Error> {
    let s = std::fs::read_to_string("provider.json")?;
    let dn: LoadableDynamicCreator = serde_json::from_str(&s)?;

    let p = DynamicCreator {
        api_key: dn.api_key,
        provider: match dn.provider.as_str() {
            "fake" => Provider::Fake,
            "digital ocean" => Provider::DigitalOcean,
            _ => return Err(anyhow::anyhow!("non-existing provider"))
        }
    };

    Ok(p)
}
