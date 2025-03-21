pub(crate) mod api;
pub(crate) mod app;
pub(crate) mod dtos;
pub(crate) mod extractors;
pub(crate) mod logger;
pub(crate) mod router;
pub(crate) mod services;

use anyhow::{Context, Result};
use app::ApplicationServer;
use utils::config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  config::init();
  ApplicationServer::serve()
    .await
    .context("Failed to start server")?;

  Ok(())
}
