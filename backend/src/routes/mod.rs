use crate::AppState;
use axum::{
  body::Body,
  http::{header::SET_COOKIE, StatusCode},
  response::{IntoResponse, Response},
  Router,
};
use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;
use serde_json::json;

mod auth;
mod transactions;

pub fn app() -> Router<AppState> {
  Router::new()
    .nest("/auth", auth::routes())
    .nest("/transactions", transactions::routes())
}

#[derive(Serialize)]
pub enum ApiResponse {
  #[serde(with = "http_serde::status_code")]
  Simple(StatusCode),
  Success {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    data: Box<dyn ErasedSerialize + Send>,
  },
  SuccessWithCookies {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    data: Box<dyn ErasedSerialize + Send>,
    cookies: Vec<String>,
  },
  Failure {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    error: ApiError,
  },
}

#[derive(Serialize)]
pub enum ApiError {
  Simple(String),
  Complex {
    message: String,
    error: Box<dyn ErasedSerialize + Send>,
  },
}

impl IntoResponse for ApiResponse {
  fn into_response(self) -> Response {
    let mut res_builder = Response::builder().header("Content-Type", "application/json");

    match self {
      ApiResponse::Simple(status) => res_builder.status(status).body(Body::empty()).unwrap(),
      ApiResponse::Success { data, status } => res_builder
        .status(status)
        .body(Body::from(json!(*data).to_string()))
        .unwrap(),
      ApiResponse::SuccessWithCookies {
        data,
        cookies,
        status,
      } => {
        let header_map = res_builder.headers_mut().expect("Failed to get headers");

        for cookie in cookies {
          header_map.append(SET_COOKIE, cookie.parse().unwrap());
        }
        res_builder
          .status(status)
          .body(Body::from(json!(*data).to_string()))
          .unwrap()
      }
      ApiResponse::Failure { error, status } => match error {
        ApiError::Simple(e) => res_builder
          .status(status)
          .body(Body::from(json!({ "message": e }).to_string()))
          .unwrap(),
        ApiError::Complex { message, error } => res_builder
          .status(status)
          .body(Body::from(
            json!({"message": message, "error": *error}).to_string(),
          ))
          .unwrap(),
      },
    }
  }
}
