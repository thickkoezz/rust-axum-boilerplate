use crate::dtos::user_dto::{
  ChangePasswordDto, LoginInDto, LoginOutDto, SignUpUserDto, UpdateUserDto,
};
use async_trait::async_trait;
use axum_extra::headers::Cookie;
use database::user::{model::User, repository::DynUserRepository};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use std::sync::Arc;
use tracing::{error, info};
use utils::{
  AppConfig, AppError, AppResult, cookie,
  jwt::{create_token, decode_token},
  password::{hash_password, verify_password},
};

#[allow(clippy::module_name_repetitions)]
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait UserServiceTrait {
  async fn signup_user(&self, request: SignUpUserDto) -> AppResult<InsertOneResult>;

  async fn login_user(&self, request: LoginInDto) -> AppResult<(LoginOutDto, cookie::Cookie)>;

  async fn get_all_users(&self, cookie: Cookie) -> AppResult<Vec<User>>;

  async fn get_user_by_id(&self, cookie: Cookie, user_id: &str) -> AppResult<Option<User>>;

  async fn get_user_by_email(&self, cookie: Cookie, user_id: &str) -> AppResult<Option<User>>;

  async fn update_user(&self, cookie: Cookie, request: UpdateUserDto) -> AppResult<UpdateResult>;

  async fn change_password(
    &self,
    cookie: Cookie,
    request: ChangePasswordDto,
  ) -> AppResult<UpdateResult>;

  async fn delete_user(&self, cookie: Cookie, user_id: &str) -> AppResult<DeleteResult>;
}

#[derive(Clone)]
pub struct UserService {
  repository: DynUserRepository,
  config: Arc<AppConfig>,
}

impl UserService {
  pub fn new(repository: DynUserRepository, config: Arc<AppConfig>) -> Self {
    Self { repository, config }
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

  async fn login_user(&self, request: LoginInDto) -> AppResult<(LoginOutDto, cookie::Cookie)> {
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
    println!("password: {:?}", &password);
    println!("user.password: {:?}", &user);
    if verify_password(&password, &user.password).is_err() {
      error!("invalid password for user {:?}", email);
      return Err(AppError::Unauthorized);
    }

    let (token, exp) = create_token(
      &self.config.jwt_secret,
      &user.email,
      self.config.jwt_expiration,
    );
    let odata = LoginOutDto {
      email: user.email,
      token: token.clone(),
      exp,
    };

    let cookie = cookie::create(token);
    info!("user {:?} logged in", email);
    Ok((odata, cookie))
  }

  async fn get_all_users(&self, cookie: Cookie) -> AppResult<Vec<User>> {
    if let Some(jwt_token) = cookie.get("jwt_token") {
      if decode_token(&jwt_token, &self.config.jwt_secret).is_err() {
        error!("can't decode token");
        return Err(AppError::Unauthorized);
      }

      let users = self.repository.get_all_users().await?;
      return Ok(users);
    }

    error!("token not found in cookie");
    Err(AppError::Unauthorized)
  }

  async fn get_user_by_id(&self, cookie: Cookie, user_id: &str) -> AppResult<Option<User>> {
    if let Some(jwt_token) = cookie.get("jwt_token") {
      if decode_token(&jwt_token, &self.config.jwt_secret).is_err() {
        error!("can't decode token");
        return Err(AppError::Unauthorized);
      }

      let user = self.repository.get_user_by_id(user_id).await?;
      return Ok(user);
    }

    error!("token not found in cookie");
    Err(AppError::Unauthorized)
  }

  async fn get_user_by_email(&self, cookie: Cookie, email: &str) -> AppResult<Option<User>> {
    if let Some(jwt_token) = cookie.get("jwt_token") {
      if decode_token(&jwt_token, &self.config.jwt_secret).is_err() {
        error!("can't decode token");
        return Err(AppError::Unauthorized);
      }

      let user = self.repository.get_user_by_email(email).await?;
      return Ok(user);
    }

    error!("token not found in cookie");
    Err(AppError::Unauthorized)
  }

  async fn update_user(&self, cookie: Cookie, request: UpdateUserDto) -> AppResult<UpdateResult> {
    if let Some(jwt_token) = cookie.get("jwt_token") {
      if decode_token(&jwt_token, &self.config.jwt_secret).is_err() {
        error!("can't decode token");
        return Err(AppError::Unauthorized);
      }

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
      return Ok(result);
    }

    error!("token not found in cookie");
    Err(AppError::Unauthorized)
  }

  async fn change_password(
    &self,
    cookie: Cookie,
    request: ChangePasswordDto,
  ) -> AppResult<UpdateResult> {
    if let Some(jwt_token) = cookie.get("jwt_token") {
      if decode_token(&jwt_token, &self.config.jwt_secret).is_err() {
        error!("can't decode token");
        return Err(AppError::Unauthorized);
      }

      let id = request.id.unwrap();
      let password = request.password.unwrap();
      let password = hash_password(&password)?;

      let result = self.repository.change_password(&id, &password).await?;
      info!("updated user {:?}", result);
      return Ok(result);
    }

    error!("token not found in cookie");
    Err(AppError::Unauthorized)
  }

  async fn delete_user(&self, cookie: Cookie, user_id: &str) -> AppResult<DeleteResult> {
    if let Some(jwt_token) = cookie.get("jwt_token") {
      if decode_token(&jwt_token, &self.config.jwt_secret).is_err() {
        error!("can't decode token");
        return Err(AppError::Unauthorized);
      }

      let result = self.repository.delete_user(user_id).await?;
      return Ok(result);
    }

    error!("token not found in cookie");
    Err(AppError::Unauthorized)
  }
}
