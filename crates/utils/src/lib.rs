use argon2::{Argon2, PasswordHash, password_hash::SaltString};

pub mod config;
pub mod errors;

pub use config::*;
pub use errors::*;

#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<()> {
  let hash = PasswordHash::new(&password_hash)
    .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
  let result = hash.verify_password(&[&Argon2::default()], password);
  match result {
    Ok(_) => Ok(()),
    Err(_) => Err(anyhow::anyhow!("invalid password")),
  }
}

pub fn hash_password(password: &str) -> anyhow::Result<String> {
  let salt = SaltString::generate(rand::thread_rng());
  Ok(
    PasswordHash::generate(Argon2::default(), password, &salt)
      .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
      .to_string(),
  )
}
