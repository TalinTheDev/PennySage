use axum::{
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
