use std::sync::Arc;

use async_trait::async_trait;
use database::user::{model::User, repository::DynUserRepository};
use mongodb::results::InsertOneResult;
use tracing::{error, info};
use utils::{
  AppConfig, AppError, AppResult,
  jwt::create_token,
  password::{hash_password, verify_password},
};

use crate::dtos::user_dto::{LoginInDto, LoginOutDto, SignUpUserDto};

#[allow(clippy::module_name_repetitions)]
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait UserServiceTrait {
  // async fn get_current_user(&self, user_id: &str) -> AppResult<Option<User>>;

  async fn get_all_users(&self) -> AppResult<Vec<User>>;

  async fn signup_user(&self, request: SignUpUserDto) -> AppResult<InsertOneResult>;

  async fn login_user(&self, config: Arc<AppConfig>, request: LoginInDto)
  -> AppResult<LoginOutDto>;
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
  async fn get_all_users(&self) -> AppResult<Vec<User>> {
    let users = self.repository.get_all_users().await?;

    Ok(users)
  }

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

    let new_user = self
      .repository
      .create_user(&name, &email, &password)
      .await?;

    info!("created user {:?}", new_user);

    Ok(new_user)
  }

  async fn login_user(
    &self,
    config: Arc<AppConfig>,
    request: LoginInDto,
  ) -> AppResult<LoginOutDto> {
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

    let (token, exp) = create_token(&config.jwt_secret, &user.email, config.jwt_expiration);

    let odata = LoginOutDto {
      email: user.email,
      token: token.clone(),
      exp,
    };

    info!("user {:?} logged in", email);

    Ok(odata)
  }

  // async fn get_current_user(&self, user_id: &str) -> AppResult<Option<User>> {
  //     let user = self.repository.get_user_by_id(user_id).await?;

  //     Ok(user)
  // }
}
