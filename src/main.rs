use anyhow::Result;

mod cli;
mod client;
mod endpoints;
mod models;
mod params;

#[tokio::main]
async fn main() -> Result<()> {
    cli::Cli::run().await
}
