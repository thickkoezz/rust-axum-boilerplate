use crate::{AppError, AppResult};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TokenType {
  Access,
  Refresh,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String, // usually email or username
  pub exp: usize,
  pub token_type: TokenType,
}

pub fn create_token(secret: &str, email: &str, jwt_expiry: usize, token_type: TokenType) -> String {
  let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs() as usize
    + jwt_expiry;

  let claims = Claims {
    sub: email.into(),
    exp: expiration,
    token_type,
  };

  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .unwrap()
}

pub fn decode_token(token: &str, secret: &str) -> AppResult<Claims> {
  let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  );
  let token_data = token_data.map_err(|e| AppError::InvalidToken(e.to_string()))?;
  Ok(token_data.claims)
}
