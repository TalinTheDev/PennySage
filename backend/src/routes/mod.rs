use axum::{
  body::Body,
  http::{header::SET_COOKIE, StatusCode},
  response::{IntoResponse, Response},
  Router,
};
use axum_extra::extract::cookie::Cookie;
use serde::Serialize;
use serde_json::json;

use crate::AppState;
mod auth;
mod transactions;

pub fn app() -> Router<AppState> {
  Router::new()
    .nest("/auth", auth::routes())
    .nest("/transactions", transactions::routes())
}

pub struct ApiError {
  pub code: StatusCode,
  pub message: String,
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    (
      self.code,
      json!({"success": false, "message": format!("Something went wrong: {}", self.message)})
        .to_string(),
    )
      .into_response()
  }
}

#[derive(Serialize)]
pub struct OkRes<T> {
  success: bool,
  data: T,
}

pub struct ApiRes<'c, D, E> {
  pub success: bool,
  pub cookies: Option<Vec<Cookie<'c>>>,
  pub status: StatusCode,
  pub data: Option<D>,
  pub error: Option<E>,
}

impl<'c, D, E> IntoResponse for ApiRes<'c, D, E>
where
  D: Serialize,
  E: Serialize,
{
  fn into_response(self) -> Response {
    let mut builder = Response::builder()
      .status(self.status)
      .header("Content-Type", "application/json");

    let header_map = builder.headers_mut().expect("Failed to get headers");
    if let Some(cookies) = self.cookies {
      for cookie in cookies {
        header_map.append(
          SET_COOKIE,
          cookie
            .encoded()
            .to_string()
            .parse()
            .expect("Failed to encode cookie"),
        );
      }
    }
    if self.success {
      if let Some(d) = self.data {
        Response::builder()
          .body(Body::from(json!({"success": true, "data": d}).to_string()))
          .expect("Failed to build response")
      } else {
        Response::builder()
          .body(Body::from(
            json!({"success": true, "data": null}).to_string(),
          ))
          .expect("Failed to build response")
      }
    } else {
      Response::builder()
        .body(Body::from(
          json!({"success": false, "error": self.error}).to_string(),
        ))
        .expect("Failed to build response")
    }
  }
}
