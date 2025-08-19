use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lib::db::tables::Tables;
use lib::types::{
    api_return::{ApiError, ApiResponse},
    jwt::AccessToken,
    state::AppState,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/get", get(get_options))
        .route("/new", post(new))
}

#[derive(Clone, Serialize, Deserialize)]
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

#[axum::debug_handler]
pub async fn new(
    State(state): State<AppState>,
    user: AccessToken,
    Json(option): Json<OptionItem>,
) -> ApiResponse {
    let option_id = match Tables::Options.gen_id(&state.db_pool).await {
        Ok(id) => id,
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while generating option ID".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while generating option ID: {}",
                    e.to_string()
                )));
        }
    };
    match sqlx::query!(
        "INSERT INTO options (id, user_id, value, type) VALUES (?, ?, ?, ?);",
        option_id,
        user.claims.user_id,
        option.value,
        option.r#type
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            return ApiResponse::new(StatusCode::OK)
                .message("Option added successfully.".to_string())
                .json(json!({"option_id": option_id}));
        }
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while adding option".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while adding option: {}",
                    e.to_string()
                )));
        }
    }
}
