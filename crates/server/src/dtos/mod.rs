use serde::Deserialize;
use validator::Validate;

pub mod user_dto;

#[derive(Clone, Deserialize, Debug, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct IdOnlyDto {
  #[validate(required)]
  pub id: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct EmailOnlyDto {
  #[validate(length(min = 1), email(message = "email is invalid"))]
  pub email: Option<String>,
}
