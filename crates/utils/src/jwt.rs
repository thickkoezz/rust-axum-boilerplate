use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
}

pub fn create_token(secret: &str, sub: &str, jwt_expiry: usize) -> String {
  let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs() as usize
    + jwt_expiry; // Token valid for 1 week
  let claims = Claims {
    sub: sub.into(),
    exp: expiration,
  };
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .unwrap()
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, errors::Error> {
  let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  )?;
  Ok(token_data.claims)
}
