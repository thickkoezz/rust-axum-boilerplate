mod user_controller;

use axum::routing::{Router, get};

pub async fn health() -> &'static str {
  "🚀 Server is running! 🚀"
}

pub fn app() -> Router {
  Router::new()
    .route("/", get(health))
    .nest("/users", user_controller::UserController::app())
}
