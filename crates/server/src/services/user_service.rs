use crate::dtos::user_dto::{ChangePasswordDto, LoginInDto, SignUpUserDto, UpdateUserDto};
use async_trait::async_trait;
use database::user::{model::User, repository::DynUserRepository};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use std::sync::Arc;
use tracing::{error, info};
use utils::{
  AppError, AppResult, config, cookie,
  jwt::{TokenType, create_token},
  password::{hash_password, verify_password},
};

#[allow(clippy::module_name_repetitions)]
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait UserServiceTrait {
  async fn signup_user(&self, request: SignUpUserDto) -> AppResult<InsertOneResult>;

  async fn login_user(
    &self,
    request: LoginInDto,
  ) -> AppResult<(String, cookie::Cookie, String, cookie::Cookie)>;

  async fn refresh_access_token(
    &self,
    refresh_token: String,
  ) -> AppResult<(String, cookie::Cookie, String, cookie::Cookie)>;

  async fn get_all_users(&self) -> AppResult<Vec<User>>;

  async fn get_user_by_id(&self, user_id: &str) -> AppResult<Option<User>>;

  async fn get_user_by_email(&self, user_id: &str) -> AppResult<Option<User>>;

  async fn update_user(&self, request: UpdateUserDto) -> AppResult<UpdateResult>;

  async fn change_password(&self, request: ChangePasswordDto) -> AppResult<UpdateResult>;

  async fn delete_user(&self, user_id: &str) -> AppResult<DeleteResult>;

  async fn logout_user(&self) -> AppResult<(cookie::Cookie, cookie::Cookie)>;
}

#[derive(Clone)]
pub struct UserService {
  repository: DynUserRepository,
}

impl UserService {
  pub fn new(repository: DynUserRepository) -> Self {
    Self { repository }
  }
}

#[async_trait]
impl UserServiceTrait for UserService {
  async fn signup_user(&self, request: SignUpUserDto) -> AppResult<InsertOneResult> {
    let email = request.email.unwrap();
    let name = request.name.unwrap();
    let password = request.password.unwrap();
    let password = hash_password(&password)?;

    let existing_user = self.repository.get_user_by_email(&email).await?;
    if existing_user.is_some() {
      error!("user {:?} already exists", email);
      return Err(AppError::Conflict(format!("email {email} is taken")));
    }

    let result = self
      .repository
      .create_user(&name, &email, &password)
      .await?;
    info!("created user {:?}", result);
    Ok(result)
  }

  async fn login_user(
    &self,
    request: LoginInDto,
  ) -> AppResult<(String, cookie::Cookie, String, cookie::Cookie)> {
    let cfg = config::get();
    let email = request.email.unwrap();
    let password = request.password.unwrap();

    let existing_user = self.repository.get_user_by_email(&email).await?;
    if existing_user.is_none() {
      error!("user {:?} does not exist", email);
      return Err(AppError::NotFound(format!(
        "user {:?} does not exist",
        email
      )));
    }

    let user = existing_user.unwrap();
    if verify_password(&password, &user.password).is_err() {
      error!("invalid password for user {:?}", email);
      return Err(AppError::Unauthorized);
    }

    let access_token = create_token(
      &cfg.jwt.access_token_secret,
      &user.email,
      cfg.jwt.access_token_expiry,
      TokenType::Access,
    );

    let access_cookie = cookie::create(
      "access_token",
      access_token.clone(),
      cfg.jwt.access_token_expiry,
    );

    let refresh_token = create_token(
      &cfg.jwt.refresh_token_secret,
      &user.email,
      cfg.jwt.refresh_token_expiry,
      TokenType::Refresh,
    );

    let refresh_cookie = cookie::create(
      "refresh_token",
      refresh_token.clone(),
      cfg.jwt.refresh_token_expiry,
    );

    info!("user {:?} logged in", email);
    Ok((access_token, access_cookie, refresh_token, refresh_cookie))
  }

  async fn refresh_access_token(
    &self,
    refresh_token: String,
  ) -> AppResult<(String, cookie::Cookie, String, cookie::Cookie)> {
    let cfg = config::get();
    let claims = utils::jwt::decode_token(&refresh_token, &cfg.jwt.refresh_token_secret)?;
    if claims.token_type != utils::jwt::TokenType::Refresh {
      return Err(AppError::InvalidToken("Not a refresh token".to_string()));
    }

    let user = self
      .repository
      .get_user_by_email(&claims.sub)
      .await?
      .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let access_token = create_token(
      &cfg.jwt.access_token_secret,
      &user.email,
      cfg.jwt.access_token_expiry,
      TokenType::Access,
    );

    let refresh_token = create_token(
      &cfg.jwt.refresh_token_secret,
      &user.email,
      cfg.jwt.refresh_token_expiry,
      TokenType::Refresh,
    );

    let access_cookie = cookie::create(
      "access_token",
      access_token.clone(),
      cfg.jwt.access_token_expiry,
    );

    let refresh_cookie = cookie::create(
      "refresh_token",
      refresh_token.clone(),
      cfg.jwt.refresh_token_expiry,
    );

    info!("user {:?} refreshed access token", user.email);
    Ok((access_token, access_cookie, refresh_token, refresh_cookie))
  }

  async fn get_all_users(&self) -> AppResult<Vec<User>> {
    let users = self.repository.get_all_users().await?;
    Ok(users)
  }

  async fn get_user_by_id(&self, user_id: &str) -> AppResult<Option<User>> {
    let user = self.repository.get_user_by_id(user_id).await?;
    Ok(user)
  }

  async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
    let user = self.repository.get_user_by_email(email).await?;
    Ok(user)
  }

  async fn update_user(&self, request: UpdateUserDto) -> AppResult<UpdateResult> {
    let id = request.id.unwrap();
    let email = request.email.unwrap();
    let name = request.name.unwrap();
    let existing_user = self.repository.get_user_by_email(&email).await?;

    if existing_user.is_some() {
      let existing_id = existing_user.unwrap().id.unwrap().to_hex();
      if existing_id != id {
        error!("user {:?} already exists", email);
        return Err(AppError::Conflict(format!("email {email} is taken")));
      }
    }

    let result = self.repository.update_user(&id, &name, &email).await?;
    info!("updated user {:?}", result);
    Ok(result)
  }

  async fn change_password(&self, request: ChangePasswordDto) -> AppResult<UpdateResult> {
    let id = request.id.unwrap();
    let password = request.password.unwrap();
    let password = hash_password(&password)?;
    let result = self.repository.change_password(&id, &password).await?;
    info!("updated user {:?}", result);
    Ok(result)
  }

  async fn delete_user(&self, user_id: &str) -> AppResult<DeleteResult> {
    let result = self.repository.delete_user(user_id).await?;
    Ok(result)
  }

  async fn logout_user(&self) -> AppResult<(cookie::Cookie, cookie::Cookie)> {
    let access_cookie = cookie::delete("access_token");
    let refresh_cookie = cookie::delete("refresh_token");
    info!("user logged out");
    Ok((access_cookie, refresh_cookie))
  }
}
