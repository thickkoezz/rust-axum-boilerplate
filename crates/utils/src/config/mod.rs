mod db_config;
mod jwt_config;
mod log_config;
pub mod server_config;

use figment::{
  Figment,
  providers::{Env, Format, Toml},
};
use server_config::ServerConfig;
use std::{process::exit, sync::OnceLock};

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn init() {
  let raw_config = Figment::new()
    .merge(Toml::file(
      Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"),
    ))
    .merge(Env::prefixed("APP_").global());

  let mut config = match raw_config.extract::<ServerConfig>() {
    Ok(s) => s,
    Err(e) => {
      eprintln!("It looks like your config is invalid. The following error occurred: {e}");
      exit(1);
    }
  };
  if config.db.uri.is_empty() {
    config.db.uri = std::env::var("DATABASE_URI").unwrap_or_default();
  }
  if config.db.uri.is_empty() {
    eprintln!("DATABASE_URI is not set");
    exit(1);
  }
  CONFIG.set(config).expect("config should be set");
}

pub fn get() -> &'static ServerConfig {
  CONFIG.get().expect("config should be set")
}

#[allow(dead_code)]
pub fn default_false() -> bool {
  false
}

#[allow(dead_code)]
pub fn default_true() -> bool {
  true
}
