use axum::{routing::get, Router};
mod analytics;
use lib::types::state::AppState;
pub mod auth;
mod options;
mod settings;
mod transactions;

pub fn app() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/transactions", transactions::routes())
        .nest("/analytics", analytics::routes())
        .nest("/settings", settings::routes())
        .nest("/options", options::routes())
        .route("/hello", get(|| async { "Hello World! Server running..." }))
}
