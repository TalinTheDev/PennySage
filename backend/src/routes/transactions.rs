use axum::{routing::get, Router};

use crate::{jwt::AccessTokenClaims, AppState};

pub fn routes() -> Router<AppState> {
  Router::new().route("/test", get(test))
}

pub async fn test(user: AccessTokenClaims) -> String {
  format!("Hello authorized user #{}", user.id)
}
