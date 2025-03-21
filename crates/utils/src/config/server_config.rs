use crate::config::db_config::DbConfig;
use crate::config::jwt_config::JwtConfig;
use crate::config::log_config::LogConfig;
use serde::Deserialize;

#[derive(clap::ValueEnum, Deserialize, Clone, Debug, Copy)]
pub enum CargoEnv {
  Development,
  Production,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
  pub cargo_env: CargoEnv,
  pub rust_log: String,
  #[serde(default = "default_app_host")]
  pub app_host: String,
  #[serde(default = "default_app_port")]
  pub app_port: u16,
  pub db: DbConfig,
  pub log: LogConfig,
  pub jwt: JwtConfig,
}

fn default_app_host() -> String {
  "127.0.0.1".into()
}

fn default_app_port() -> u16 {
  5000
}
