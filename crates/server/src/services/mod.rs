pub(crate) mod user_service;

use crate::services::user_service::{DynUserService, UserService};
use database::Database;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct Services {
  pub user: DynUserService,
}

impl Services {
  pub fn new(db: Database) -> Self {
    info!("initializing services...");
    let repository = Arc::new(db);
    let user = Arc::new(UserService::new(repository.clone())) as DynUserService;
    Self { user }
  }
}
