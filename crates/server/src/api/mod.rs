mod user_controller;

use axum::{
  body::Body,
  http::Request,
  middleware::Next,
  response::Response,
  routing::{Router, get},
};
use axum_extra::{TypedHeader, headers::Cookie};
use tracing::error;
use utils::{AppError, config, jwt::decode_token};

pub async fn health() -> &'static str {
  "🚀 Server is running! 🚀"
}

pub fn app() -> Router {
  Router::new()
    .route("/", get(health))
    .nest("/users", user_controller::UserController::app())
}

async fn authenticate_user<B>(
  TypedHeader(cookie): TypedHeader<Cookie>,
  request: Request<Body>,
  next: Next,
) -> Result<Response, AppError> {
  let cfg = config::get();

  if let Some(access_token) = cookie.get("access_token") {
    if decode_token(access_token, &cfg.jwt.access_token_secret).is_err() {
      error!("can't decode token");
      return Err(AppError::Unauthorized);
    }
    return Ok(next.run(request).await);
  }

  error!("token not found in cookie");
  Err(AppError::Unauthorized)
}
