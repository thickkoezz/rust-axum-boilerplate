use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
  /// Settings for the primary database. This is usually writeable, but will be read-only in
  /// some configurations.
  /// An optional follower database. Always read-only.
  #[serde(alias = "database_uri")]
  pub uri: String,
  #[serde(default = "default_database")]
  pub database: String,
  #[serde(default = "default_collection")]
  pub collection: String,
}

fn default_database() -> String {
  "rust-axum-boilerplate-db".into()
}
fn default_collection() -> String {
  "User".into()
}
