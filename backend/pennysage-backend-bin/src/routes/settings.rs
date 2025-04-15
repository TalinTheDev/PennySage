use axum::{extract::State, routing::patch, Json, Router};
use http::StatusCode;
use lib::types::jwt::AccessToken;
use lib::types::state::AppState;
use serde::Deserialize;
use sqlx::Row;
use validator::Validate;

use super::auth::set_tokens;
use lib::types::api_return::{ApiError, ApiResponse};
use lib::types::db::UserRow;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/editUser", patch(edit_user))
        .route("/updatePassword", patch(change_password))
}

#[derive(Deserialize, Validate)]
pub struct EditField {
    #[validate(length(min = 1))]
    field: String,
    #[validate(length(min = 1))]
    value: String,
}
#[derive(Deserialize, Validate)]
pub struct EditReq {
    edits: Vec<EditField>,
}

pub async fn edit_user(
    State(state): State<AppState>,
    user: AccessToken,
    Json(data): Json<EditReq>,
) -> ApiResponse {
    if data.edits.len() == 0 {
        return ApiResponse::new(StatusCode::OK).message("No user details to update".to_string());
    } else {
        let mut query: sqlx::QueryBuilder<'_, sqlx::Sqlite> =
            sqlx::QueryBuilder::new("UPDATE users SET ".to_owned());
        for (i, edit) in data.edits.iter().enumerate() {
            if edit.field.as_str() == "id" {
                continue;
            }
            if i > 0 {
                query.push(", ");
            }
            match edit.field.as_str() {
                "firstName" => query.push("first_name = "),
                "lastName" => query.push("last_name = "),
                "username" => query.push("username = "),
                "email" => query.push("email = "),
                &_ => continue,
            };
            query.push_bind(edit.value.clone());
        }
        query.push(" WHERE id = ");
        query.push_bind(user.claims.user_id);
        query.push(" RETURNING id, username, email, password, first_name, last_name, role");
        let result = query.build().fetch_one(&state.db_pool).await;
        if let Err(e) = result {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while editing user details".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while editing user details: {}",
                    e.to_string()
                )));
        } else if let Ok(r) = result {
            let user: UserRow = UserRow {
                id: r.get("id"),
                username: r.get("username"),
                email: r.get("email"),
                password: r.get("password"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                role: r.get("role"),
            };
            return set_tokens(user, state.db_pool).await;
        }
    }
    ApiResponse::new(StatusCode::OK).message("User details edited successfully".to_string())
}

#[derive(Deserialize, Validate)]
pub struct PasswordChange {
    #[validate(length(min = 1))]
    old: String,
    #[validate(length(min = 1))]
    new: String,
}

pub async fn change_password(
    State(state): State<AppState>,
    user: AccessToken,
    Json(data): Json<PasswordChange>,
) -> ApiResponse {
    match sqlx::query!(
        "SELECT password FROM users WHERE id = ?",
        user.claims.user_id
    )
    .fetch_one(&state.db_pool)
    .await
    {
        // Old password fetched successfully from database in `old.password`
        Ok(old) => {
            // If password is valid
            if match lib::auth::password::verify_password(data.old, old.password) {
                Ok(val) => val,
                Err(e) => {
                    return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                        .message("Internal server error while changing password".to_string())
                        .error(ApiError::new(format!(
                            "Internal server error while changing password: {}",
                            e.to_string()
                        )));
                }
            } {
                // Hash the new password
                let new_hash = match lib::auth::password::hash_password(data.new) {
                    Ok(hash) => hash,
                    Err(e) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                            .message("Internal server error while changing password".to_string())
                            .error(ApiError::new(format!(
                                "Internal server error while changing password: {}",
                                e.to_string()
                            )));
                    }
                };

                // Update the password in the database
                match sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    new_hash,
                    user.claims.user_id
                )
                .execute(&state.db_pool)
                .await
                {
                    Ok(_) => {
                        return ApiResponse::new(StatusCode::OK)
                            .message("Password changed successfully".to_string());
                    }
                    Err(e) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                            .message("Internal server error while changing password".to_string())
                            .error(ApiError::new(format!(
                                "Internal server error while changing password: {}",
                                e.to_string()
                            )));
                    }
                }
            }
            // Else if the password is invalid
            else {
                return ApiResponse::new(StatusCode::BAD_REQUEST)
                    .message("Incorrect password".to_string())
                    .error(ApiError::new("Incorrect password".to_string()));
            }
        }
        // Error while fetching the old password from database
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while changing password".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while changing password: {}",
                    e.to_string()
                )));
        }
    }
}
