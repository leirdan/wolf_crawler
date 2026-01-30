use anyhow::Result;
use log::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Hello, world!");
    debug!("Making request to page...");

    let res = reqwest::get("https://en.uesp.net/wiki/Skyrim:Skyrim")
        .await?
        .text()
        .await?;

    info!("{:?}", res);

    Ok(())
}
