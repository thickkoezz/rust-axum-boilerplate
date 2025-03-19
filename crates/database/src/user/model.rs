use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct User {
  #[serde(rename = "_id")]
  pub id: Option<ObjectId>,
  #[validate(length(min = 1))]
  pub name: String,
  #[validate(length(min = 1), email(message = "email is invalid"))]
  pub email: String,
  #[validate(length(min = 6))]
  #[serde(skip_deserializing)]
  pub password: String,
}
