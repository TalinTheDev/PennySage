use axum::{
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
  Router,
};
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

pub struct ApiRes<D, E> {
  success: bool,
  status: StatusCode,
  data: Option<D>,
  error: Option<E>,
}

impl<D, E> IntoResponse for ApiRes<D, E>
where
  D: Serialize,
  E: Serialize,
{
  fn into_response(self) -> Response {
    if self.success {
      if let Some(d) = self.data {
        Response::builder()
          .status(self.status)
          .header("Content-Type", "application/json")
          .body(Body::from(json!({"success": true, "data": d}).to_string()))
          .unwrap()
      } else {
        Response::builder()
          .status(self.status)
          .header("Content-Type", "application/json")
          .body(Body::from(
            json!({"success": true, "data": null}).to_string(),
          ))
          .unwrap()
      }
    } else if let Some(e) = self.error {
      Response::builder()
        .status(self.status)
        .header("Content-Type", "application/json")
        .body(Body::from(
          json!({"success": false, "error": e}).to_string(),
        ))
        .unwrap()
    } else {
      Response::builder()
        .status(self.status)
        .header("Content-Type", "application/json")
        .body(Body::from(
          json!({"success": false, "error": null}).to_string(),
        ))
        .unwrap()
    }
  }
}
