use axum::{routing::get, Router};

use crate::{jwt::AccessTokenClaims, AppState};

pub fn routes() -> Router<AppState> {
  Router::new().route("/test", get(test))
}

pub async fn test(user_claims: AccessTokenClaims) -> String {
  format!("Hello authorized user #{}", user_claims.user_id)
}
