
use async_std::task;
use cirrus::prelude::*;
use cirrus::providers::digital_ocean::*;
use cirrus::providers::digital_ocean::response::{DropletSize, Images, Regions};

const KEY: &'static str = "8b23d874cc029210e3ee91dbfadf301e4025b8c9a3599ee87eecca60cbd28044";

#[test]
fn a() {
    task::block_on(rm());
}


async fn rm() {
    let d = DigitalOceanApi::new(KEY.into());
    let request = Request::new("test".into(), Regions::Sfo3, DropletSize::Basic1C1G, Images::Ubuntu2004x64);
    let response = d.create_server(&request).await.unwrap();
    println!("{}", response.droplet.id);
}















