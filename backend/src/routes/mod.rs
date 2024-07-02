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

/*
pub struct ApiResponse<'a, D, E> {
  pub status: StatusCode,
  pub cookies: Option<Vec<Cookie<'a>>>,
  pub response: ResponseType<D, E>,
}

pub enum ResponseType<D, E> {
  Success(Option<D>),
  Failure(Option<E>),
}

impl<'a, D, E> IntoResponse for ApiResponse<'a, D, E>
where
  D: Serialize,
  E: Serialize,
{
  fn into_response(self) -> Response {
    let mut res_builder = Response::builder()
      .status(self.status)
      .header("Content-Type", "application/json");

    let header_map = res_builder.headers_mut().expect("Failed to get headers");

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

    let body = match self.response {
      ResponseType::Success(data) => {
        Body::from(json!({"success": true, "data": data }).to_string())
      }
      ResponseType::Failure(error) => {
        Body::from(json!({"success": false, "error": error }).to_string())
      }
    };

    res_builder.body(body).unwrap()
  }
}
*/

#[derive(Serialize)]
pub enum ApiResponse {
  #[serde(with = "http_serde::status_code")]
  Simple(StatusCode),
  Success {
    data: Box<dyn ErasedSerialize>,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
  },
  SuccessWithCookies {
    data: Box<dyn ErasedSerialize>,
    cookies: Vec<String>,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
  },
  Failure {
    error: ApiError,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
  },
}

#[derive(Serialize)]
pub enum ApiError {
  Simple(String),
  Complex {
    message: String,
    error: Box<dyn ErasedSerialize>,
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
      ApiResponse::Failure { error, status } => res_builder
        .status(status)
        .body(Body::from(json!(error).to_string()))
        .unwrap(),
    }
  }
}
