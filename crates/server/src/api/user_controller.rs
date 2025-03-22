use crate::{
  api::authenticate_user,
  dtos::{
    EmailOnlyDto, IdOnlyDto,
    user_dto::{ChangePasswordDto, LoginInDto, SignUpUserDto, UpdateUserDto, UserResponse},
  },
  extractors::validation_extractor::ValidationExtractor,
  services::Services,
};
use axum::{
  Extension, Json, Router,
  body::Body,
  http::{StatusCode, header::SET_COOKIE},
  middleware::from_fn,
  response::IntoResponse,
  routing::{delete, get, post, put},
};
use database::user::model::User;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use utils::{AppResult, cookie};

pub struct UserController;

impl UserController {
  pub fn app() -> Router {
    let unprotected = Router::new()
      .route("/signup", post(Self::signup))
      .route("/login", post(Self::login))
      .route("/logout", get(Self::logout));
    let protected = Router::new()
      .route("/", get(Self::get_all))
      .route("/get", get(Self::get_by_id))
      .route("/get/:email", get(Self::get_by_email))
      .route("/update", put(Self::update))
      .route("/change-password", put(Self::change_password))
      .route("/delete", delete(Self::delete))
      .route_layer(from_fn(authenticate_user::<Body>));
    unprotected.merge(protected)
  }

  pub async fn signup(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<SignUpUserDto>,
  ) -> AppResult<Json<InsertOneResult>> {
    let created_user = services.user.signup_user(req).await?;
    Ok(Json(created_user))
  }

  pub async fn login(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<LoginInDto>,
  ) -> AppResult<impl IntoResponse> {
    let (odata, cookie) = services.user.login_user(req).await?;
    let mut response = Json(odata).into_response();
    response
      .headers_mut()
      .insert(SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
  }

  pub async fn logout() -> AppResult<impl IntoResponse> {
    let cookie = cookie::delete();
    let mut response = StatusCode::OK.into_response();
    response
      .headers_mut()
      .insert(SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
  }

  pub async fn get_all(
    Extension(services): Extension<Services>,
  ) -> AppResult<Json<Vec<UserResponse>>> {
    let users = services.user.get_all_users().await?;
    let users2: Vec<UserResponse> = users.into_iter().map(|u| UserResponse::from(u)).collect();
    Ok(Json(users2))
  }

  pub async fn get_by_id(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<IdOnlyDto>,
  ) -> AppResult<Json<Option<User>>> {
    let id = req.id.unwrap();
    let user = services.user.get_user_by_id(&id).await?;
    Ok(Json(user))
  }

  pub async fn get_by_email(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<EmailOnlyDto>,
  ) -> AppResult<Json<Option<User>>> {
    let email = req.email.unwrap();
    let user = services.user.get_user_by_email(&email).await?;
    Ok(Json(user))
  }

  pub async fn update(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<UpdateUserDto>,
  ) -> AppResult<Json<UpdateResult>> {
    let result = services.user.update_user(req).await?;
    Ok(Json(result))
  }

  pub async fn change_password(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<ChangePasswordDto>,
  ) -> AppResult<Json<UpdateResult>> {
    let result = services.user.change_password(req).await?;
    Ok(Json(result))
  }

  pub async fn delete(
    Extension(services): Extension<Services>,
    ValidationExtractor(req): ValidationExtractor<IdOnlyDto>,
  ) -> AppResult<Json<DeleteResult>> {
    let id = req.id.unwrap();
    let result = services.user.delete_user(&id).await?;
    Ok(Json(result))
  }
}
