pub use cookie::Cookie;

pub fn create(name: &'static str, value: String, max_age: usize) -> Cookie<'static> {
  Cookie::build((name, value))
    .path("/")
    .http_only(true)
    .secure(true)
    .expires(Some(
      time::OffsetDateTime::now_utc() + time::Duration::seconds(max_age as i64),
    ))
    .max_age(time::Duration::seconds(max_age as i64))
    .build()
}

pub fn delete(name: &'static str) -> Cookie<'static> {
  Cookie::build(name)
    .path("/")
    .http_only(true)
    .same_site(cookie::SameSite::Lax)
    .secure(true)
    .expires(Some(
      time::OffsetDateTime::now_utc() - time::Duration::days(1),
    ))
    .max_age(time::Duration::seconds(-1))
    .build()
}
