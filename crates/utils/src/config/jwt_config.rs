use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
  pub access_token_secret: String,
  pub access_token_expiry: usize,
  pub refresh_token_secret: String,
  pub refresh_token_expiry: usize,
}
