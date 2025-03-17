use axum::{
  Json, async_trait,
  extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use utils::AppError;
use validator::Validate;

pub struct ValidationExtractor<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidationExtractor<T>
where
  T: DeserializeOwned + Validate,
  S: Send + Sync,
  Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
  type Rejection = AppError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Json(value) = Json::<T>::from_request(req, state).await?;
    value.validate()?;
    Ok(ValidationExtractor(value))
  }
}
