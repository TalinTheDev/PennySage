use axum::Router;

use crate::AppState;
mod auth;

pub fn app() -> Router<AppState> {
  Router::new().nest("/auth", auth::routes())
}
