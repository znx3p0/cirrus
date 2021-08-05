
use async_std::task;
use cirrus::prelude::*;
use cirrus::providers::digital_ocean::*;
use cirrus::providers::digital_ocean::response::{DropletSize, Images, Regions};
use dotenv::dotenv;


#[test]
fn a() {
    dotenv().unwrap();
    task::block_on(rm());
}


async fn rm() {
    let d = DigitalOceanApi::new(std::env::var("KEY").unwrap());
    let request = Request::new("test".into(), Regions::Sfo3, DropletSize::Basic1C1G, Images::Ubuntu2004x64);
    let response = d.create_server(&request).await.unwrap();
    println!("{}", response.droplet.id);
}















