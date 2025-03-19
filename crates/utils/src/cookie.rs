pub use cookie::Cookie;

pub fn create(jwt_token: String) -> Cookie<'static> {
  Cookie::build(("jwt_token", jwt_token))
    .path("/")
    .http_only(true)
    .secure(true)
    .max_age(time::Duration::days(7))
    .build()
}

// pub async fn read_cookie(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
//   if let Some(jwt_token) = cookie.get("jwt_token") {
//     format!("Found jwt_token: {}", jwt_token)
//   } else {
//     "No jwt_token found".to_string()
//   }
// }
