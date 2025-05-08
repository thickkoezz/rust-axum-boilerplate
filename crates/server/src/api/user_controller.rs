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
  http::{
    HeaderMap, StatusCode,
    header::{COOKIE, SET_COOKIE},
  },
  middleware::from_fn,
  response::IntoResponse,
  routing::{delete, get, post, put},
};
use database::user::model::{LoginResponse, User};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use utils::{AppResult, cookie::Cookie};

pub struct UserController;

impl UserController {
  pub fn app() -> Router {
    let unprotected = Router::new()
      .route("/signup", post(Self::signup))
      .route("/login", post(Self::login))
      .route("/logout", get(Self::logout))
      .route("/refresh-token", post(Self::refresh_token));
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
    let (access_token, access_cookie, refresh_token, refresh_cookie) =
      services.user.login_user(req).await?;
    let login_response = LoginResponse {
      access_token,
      refresh_token,
    };
    let mut response = Json(login_response).into_response();

    response
      .headers_mut()
      .insert(SET_COOKIE, access_cookie.to_string().parse().unwrap());
    response
      .headers_mut()
      .append(SET_COOKIE, refresh_cookie.to_string().parse().unwrap());
    Ok(response)
  }

  pub async fn logout(Extension(services): Extension<Services>) -> AppResult<impl IntoResponse> {
    let (access_cookie, refresh_cookie) = services.user.logout_user().await?;
    let mut response = StatusCode::OK.into_response();
    response
      .headers_mut()
      .insert(SET_COOKIE, access_cookie.to_string().parse().unwrap());
    response
      .headers_mut()
      .append(SET_COOKIE, refresh_cookie.to_string().parse().unwrap());
    Ok(response)
  }

  pub async fn refresh_token(
    Extension(services): Extension<Services>,
    headers: HeaderMap,
  ) -> AppResult<impl IntoResponse> {
    let cookie_header = headers
      .get(COOKIE)
      .and_then(|value| value.to_str().ok())
      .unwrap_or("");

    let mut refresh_token_value = None;
    for cookie_str in cookie_header.split(';') {
      if let Ok(cookie) = Cookie::parse_encoded(cookie_str.trim()) {
        if cookie.name() == "refresh_token" {
          refresh_token_value = Some(cookie.value().to_string());
          break;
        }
      }
    }

    let refresh_token = refresh_token_value.ok_or_else(|| {
      utils::errors::AppError::BadRequest("Missing refresh token cookie".to_string())
    })?;
    let (access_token, access_cookie, refresh_token, refresh_cookie) =
      services.user.refresh_access_token(refresh_token).await?;
    let login_response = LoginResponse {
      access_token,
      refresh_token,
    };
    let mut response = Json(login_response).into_response();

    response
      .headers_mut()
      .insert(SET_COOKIE, access_cookie.to_string().parse().unwrap());
    response
      .headers_mut()
      .append(SET_COOKIE, refresh_cookie.to_string().parse().unwrap());
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
