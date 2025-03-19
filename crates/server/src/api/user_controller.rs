use crate::{
  dtos::user_dto::{LoginInDto, SignUpUserDto},
  extractors::validation_extractor::ValidationExtractor,
  services::Services,
};
use axum::{
  Extension, Json, Router,
  response::IntoResponse,
  routing::{get, post},
};
use axum_extra::{TypedHeader, headers::Cookie};
use database::user::model::User;
use mongodb::results::InsertOneResult;
use utils::AppResult;

pub struct UserController;

impl UserController {
  pub fn app() -> Router {
    Router::new()
      .route("/", get(Self::all))
      .route("/signup", post(Self::signup))
      .route("/login", post(Self::login))
  }

  pub async fn all(
    Extension(services): Extension<Services>,
    TypedHeader(cookie): TypedHeader<Cookie>,
  ) -> AppResult<Json<Vec<User>>> {
    let users = services.user.get_all_users(services.config, cookie).await?;

    Ok(Json(users))
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
    let (odata, cookie) = services.user.login_user(services.config, req).await?;

    let mut response = Json(odata).into_response();
    response.headers_mut().insert(
      axum::http::header::SET_COOKIE,
      cookie.to_string().parse().unwrap(),
    );

    Ok(response)
  }
}
