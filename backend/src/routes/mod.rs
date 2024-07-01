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
