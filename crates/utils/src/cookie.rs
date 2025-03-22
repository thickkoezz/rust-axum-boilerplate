pub use cookie::Cookie;

pub fn create(jwt_token: String) -> Cookie<'static> {
  Cookie::build(("jwt_token", jwt_token))
    .path("/")
    .http_only(true)
    .secure(true)
    .max_age(time::Duration::days(7))
    .build()
}

pub fn delete() -> Cookie<'static> {
  Cookie::build("jwt_token")
    .path("/")
    .http_only(true)
    .secure(true)
    .max_age(time::Duration::seconds(-1))
    .build()
}
