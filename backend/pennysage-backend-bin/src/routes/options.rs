use axum::{extract::State, http::StatusCode, routing::get, Router};
use lib::types::{
    api_return::{ApiError, ApiResponse},
    jwt::AccessToken,
    state::AppState,
};
use serde::Serialize;
use serde_json::json;

pub fn routes() -> Router<AppState> {
    Router::new().route("/get", get(get_options))
}

#[derive(Clone, Serialize)]
pub struct OptionItem {
    pub value: String,
    pub r#type: String,
}

#[axum::debug_handler]
pub async fn get_options(State(state): State<AppState>, user: AccessToken) -> ApiResponse {
    let mut results: Vec<OptionItem> = Vec::new();
    match sqlx::query_as!(
        OptionItem,
        "SELECT value, type FROM options WHERE user_id = ?;",
        user.claims.user_id
    )
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(rows) => {
            results = rows;
            ApiResponse::new(StatusCode::OK)
                .message("Options fetched successfully".to_string())
                .json(json!({"options": results}))
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => ApiResponse::new(StatusCode::OK)
                .message("Options fetched successfully".to_string())
                .json(json!({"options": results})),
            _ => {
                return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("Internal server error while fetching options".to_string())
                    .error(ApiError::new(format!(
                        "Could not fetch options from database (internal server error): {}",
                        e.to_string()
                    )))
            }
        },
    }
}
