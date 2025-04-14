use crate::{
    db::{hash_password, verify_password},
    jwt::AccessTokenClaims,
    AppState,
};
use axum::{extract::State, routing::patch, Json, Router};
use http::StatusCode;
use serde::Deserialize;
use sqlx::Row;
use validator::Validate;

use super::{
    auth::{set_tokens, UserRow},
    ApiError, ApiResponse,
};

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
    State(state): State<crate::AppState>,
    user: AccessTokenClaims,
    Json(data): Json<EditReq>,
) -> ApiResponse {
    if data.edits.len() == 0 {
        return ApiResponse::Success {
            status: StatusCode::OK,
            data: Box::new(format!("No user details to update")),
        };
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
        query.push_bind(user.id);
        query.push(" RETURNING id, username, email, password, first_name, last_name, role");
        let result = query.build().fetch_one(&state.db_pool).await;
        if let Err(e) = result {
            return ApiResponse::Failure {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ApiError::Complex {
                    message: "Internal server error while editing user details".to_owned(),
                    error: Box::new(e.to_string()),
                },
            };
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

    ApiResponse::Success {
        status: StatusCode::OK,
        data: Box::new("User details edited successfully"),
    }
}

#[derive(Deserialize, Validate)]
pub struct PasswordChange {
    #[validate(length(min = 1))]
    old: String,
    #[validate(length(min = 1))]
    new: String,
}

pub async fn change_password(
    State(state): State<crate::AppState>,
    user: AccessTokenClaims,
    Json(data): Json<PasswordChange>,
) -> Result<ApiResponse, ApiResponse> {
    // let old_hashed = hash_password(data.old)?;

    match sqlx::query!("SELECT password FROM users WHERE id = ?", user.id)
        .fetch_one(&state.db_pool)
        .await
    {
        Ok(old) => {
            if verify_password(data.old, old.password)? {
                let new_hash = hash_password(data.new)?;
                match sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    new_hash,
                    user.id
                )
                .execute(&state.db_pool)
                .await
                {
                    Ok(_) => {
                        return Ok(ApiResponse::Success {
                            status: StatusCode::OK,
                            data: Box::new("Password changed successfully"),
                        });
                    }
                    Err(e) => {
                        return Err(ApiResponse::Failure {
                            status: StatusCode::INTERNAL_SERVER_ERROR,
                            error: ApiError::Complex {
                                message: "Internal server error while changing password".to_owned(),
                                error: Box::new(e.to_string()),
                            },
                        });
                    }
                }
            } else {
                return Ok(ApiResponse::Success {
                    status: StatusCode::BAD_REQUEST,
                    data: Box::new("Incorrect password"),
                });
            }
        }
        Err(e) => {
            return Err(ApiResponse::Failure {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ApiError::Complex {
                    message: "Internal server error while changing password".to_owned(),
                    error: Box::new(e.to_string()),
                },
            });
        }
    }
}
