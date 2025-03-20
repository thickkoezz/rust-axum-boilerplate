use database::user::model::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct SignUpUserDto {
  #[validate(required, length(min = 1))]
  pub name: Option<String>,
  #[validate(required, length(min = 1), email(message = "email is invalid"))]
  pub email: Option<String>,
  #[validate(required, length(min = 6))]
  pub password: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct LoginInDto {
  #[validate(required, email(message = "email is invalid"))]
  pub email: Option<String>,
  #[validate(required)]
  pub password: Option<String>,
}

#[derive(Clone, Serialize, Debug, Validate, Default)]
pub struct LoginOutDto {
  pub email: String,
  pub token: String,
  pub exp: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct UpdateUserDto {
  #[validate(required)]
  pub id: Option<String>,
  #[validate(required, length(min = 1))]
  pub name: Option<String>,
  #[validate(required, length(min = 1), email(message = "email is invalid"))]
  pub email: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct ChangePasswordDto {
  #[validate(required)]
  pub id: Option<String>,
  #[validate(required, length(min = 6))]
  pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct UserResponse {
  #[serde(rename = "_id")]
  pub id: Option<ObjectId>,
  #[validate(length(min = 1))]
  pub name: String,
  #[validate(length(min = 1), email(message = "email is invalid"))]
  pub email: String,
}

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      name: user.name,
      email: user.email,
    }
  }
}
