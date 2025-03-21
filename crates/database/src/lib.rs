pub mod user;

use mongodb::{Client, Collection};
use tracing::info;
use user::model::User;
use utils::{AppResult, config};

#[derive(Clone, Debug)]
pub struct Database {
  pub user_col: Collection<User>,
}

impl Database {
  /// Creates a new `Database` instance.
  ///
  /// # Arguments
  ///
  /// * `config` - An `Arc` containing the application configuration.
  ///
  /// # Returns
  ///
  /// * `AppResult<Self>` - A result containing the `Database` instance or an error.
  ///
  /// # Errors
  ///
  /// This function will return an error if the `MongoDB` client cannot be initialized
  /// or if the specified database or collection cannot be accessed.
  pub async fn new() -> AppResult<Self> {
    let cfg = config::get();
    let client = Client::with_uri_str(&cfg.db.uri).await?;
    let db = client.database(&cfg.db.database);
    let user_col: Collection<User> = db.collection("User");

    info!("initializing database connection...");

    Ok(Database { user_col })
  }
}
