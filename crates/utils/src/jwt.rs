use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub company: String,
  pub exp: usize,
}

pub fn create_token(secret: &str, sub: &str, jwt_expiration: u16) -> (String, i64) {
  let exp = OffsetDateTime::now_utc() + Duration::seconds(jwt_expiration as i64);
  let exp = exp.unix_timestamp();
  let token = encode(
    &Header::default(),
    &Claims {
      sub: sub.into(),
      company: "mieky.id".into(),
      exp: exp as usize,
    },
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .unwrap();
  (token, exp)
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, errors::Error> {
  let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  )?;
  Ok(token_data.claims)
}
