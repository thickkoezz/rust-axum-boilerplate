use crate::{
  dtos::user_dto::{LoginInDto, SignUpUserDto},
  extractors::validation_extractor::ValidationExtractor,
  services::Services,
};
use axum::extract::Path;
use axum::{
  Extension, Json, Router,
  response::IntoResponse,
  routing::{delete, get, post},
};
use axum_extra::{TypedHeader, headers::Cookie};
use database::user::model::User;
use mongodb::results::{DeleteResult, InsertOneResult};
use utils::AppResult;

pub struct UserController;

impl UserController {
  pub fn app() -> Router {
    Router::new()
      .route("/", get(Self::all))
      .route("/signup", post(Self::signup))
      .route("/login", post(Self::login))
      .route("/get/:user_id", get(Self::get_by_id))
      .route("/get/email/:email", get(Self::get_by_email))
      .route("/delete/:user_id", delete(Self::delete))
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
    response.headers_mut().insert(
      axum::http::header::SET_COOKIE,
      cookie.to_string().parse().unwrap(),
    );
    Ok(response)
  }

  pub async fn all(
    Extension(services): Extension<Services>,
    TypedHeader(cookie): TypedHeader<Cookie>,
  ) -> AppResult<Json<Vec<User>>> {
    let users = services.user.get_all_users(cookie).await?;
    Ok(Json(users))
  }

  pub async fn get_by_id(
    Extension(services): Extension<Services>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(user_id): Path<String>,
  ) -> AppResult<Json<Option<User>>> {
    let user = services.user.get_user_by_id(cookie, &user_id).await?;
    Ok(Json(user))
  }

  pub async fn get_by_email(
    Extension(services): Extension<Services>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(email): Path<String>,
  ) -> AppResult<Json<Option<User>>> {
    let user = services.user.get_user_by_email(cookie, &email).await?;
    Ok(Json(user))
  }

  pub async fn delete(
    Extension(services): Extension<Services>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(user_id): Path<String>,
  ) -> AppResult<Json<DeleteResult>> {
    let result = services.user.delete_user(cookie, &user_id).await?;
    Ok(Json(result))
  }
}
