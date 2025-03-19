use std::time::Duration;

use axum::routing::get;
use axum::{
  BoxError, Extension, Json, Router,
  error_handling::HandleErrorLayer,
  http::{
    Method, StatusCode,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  },
  response::IntoResponse,
};
use axum_prometheus::PrometheusMetricLayer;
use lazy_static::lazy_static;
use serde_json::json;
use tower::{ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tower_http::{
  cors::{Any, CorsLayer},
  services::{ServeDir, ServeFile},
  trace::TraceLayer,
};

use super::services::Services;
use crate::api;

lazy_static! {
  static ref HTTP_TIMEOUT: u64 = 30;
}

#[allow(clippy::module_name_repetitions)]
pub struct AppRouter;

impl AppRouter {
  pub fn init(services: Services) -> Router {
    let cors = CorsLayer::new()
      .allow_origin(Any)
      .allow_methods([
        Method::GET,
        Method::POST,
        Method::DELETE,
        Method::PUT,
        Method::PATCH,
      ])
      .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let index = ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html"));

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
      .nest_service("/", index)
      .nest("/api/v1", api::app())
      .layer(cors)
      .layer(
        ServiceBuilder::new()
          .layer(Extension(services))
          .layer(TraceLayer::new_for_http())
          .layer(HandleErrorLayer::new(Self::handle_timeout_error))
          .timeout(Duration::from_secs(*HTTP_TIMEOUT))
          .layer(BufferLayer::new(1024))
          .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
      )
      .route("/metrics", get(|| async move { metric_handle.render() }))
      .layer(prometheus_layer)
      .fallback(Self::handle_404)
  }

  #[allow(clippy::unused_async)]
  async fn handle_404() -> impl IntoResponse {
    (
      StatusCode::NOT_FOUND,
      axum::response::Json(serde_json::json!({
      "errors":{
      "message": vec!(String::from("The requested resource does not exist on this server!")),}
      })),
    )
  }

  #[allow(clippy::unused_async)]
  async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
      (
        StatusCode::REQUEST_TIMEOUT,
        Json(json!({
            "error":
                format!(
                    "request took longer than the configured {} second timeout",
                    *HTTP_TIMEOUT
                )
        })),
      )
    } else {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "error": format!("unhandled internal error: {}", err)
        })),
      )
    }
  }
}
