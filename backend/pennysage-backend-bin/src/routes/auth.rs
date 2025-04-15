use axum::{extract::State, http::StatusCode, routing::delete, routing::post, Json, Router};
use axum_extra::extract::{
    cookie::{self, SameSite},
    CookieJar,
};
use cookie::Cookie;
use lib::types::api_return::{ApiError, ApiResponse};
use lib::types::{jwt::AccessToken, jwt::RefreshToken, state::AppState};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use validator::Validate;
use lib::auth::password::{hash_password, verify_password};
use lib::db::tables::Tables;
use lib::types::db::UserRow;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
        .route("/refresh_tokens", post(refresh_tokens))
        .route("/user", delete(delete_user))
}

#[derive(Deserialize, Validate)]
pub struct LoginReq {
    #[validate(length(min = 1, message = "Invalid username format"))]
    username: String,
    #[validate(length(min = 1, message = "Invalid password format"))]
    password: String,
}

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(user_details): Json<LoginReq>,
) -> ApiResponse {
    if user_details.validate().err().is_some() {
        // TODO: Tell which fields are invalid
        return ApiResponse::new(StatusCode::BAD_REQUEST)
            .message("User details provided have an invalid format".to_string())
            .error(ApiError::new(
                "User details provided have an invalid format".to_string(),
            ));
    }
    match sqlx::query_as!(
    UserRow,
    "SELECT id, username, email, password, first_name, last_name, role FROM users WHERE username = ?",
    user_details.username
  )
  .fetch_one(&state.db_pool)
  .await
  {
    Ok(user) => match verify_password(user_details.password, user.password.as_str().to_owned()) {
      Ok(valid) => {
        if valid {
          return set_tokens(user, state.db_pool).await;
        } else {
          return ApiResponse::new(StatusCode::UNAUTHORIZED)
            .message("Invalid username/password combo".to_string())
            .error(ApiError::new("Invalid username/password combo".to_string()));
          
        }
      }
      Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server authenticating user".to_string()).error(ApiError::new(format!("Internal server error authenticating user: {}", e.to_string())))
    },
    Err(sqlx::Error::RowNotFound) => return ApiResponse::new(StatusCode::UNAUTHORIZED)
            .message("Invalid username/password combo".to_string())
            .error(ApiError::new("Invalid username/password combo".to_string())),
    Err(_) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("Internal server error while looking up user".to_string())
            .error(ApiError::new("Internal server error while looking up user".to_string()))
    };
  }


#[derive(Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(length(min = 1, message = "Invalid username format"))]
    username: String,
    #[validate(email(message = "Invalid Email"))]
    email: String,
    #[validate(length(min = 1, message = "Invalid password format"))]
    password: String,
    #[validate(length(min = 1, message = "Invalid first name format"))]
    first_name: String,
    #[validate(length(min = 1, message = "Invalid last name format"))]
    last_name: String,
    #[validate(length(min = 1, message = "Invalid family name format"))]
    family_name: Option<String>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(user_details): Json<RegisterReq>,
) -> ApiResponse {
    if user_details.validate().err().is_some() {
        return ApiResponse::new(StatusCode::BAD_REQUEST)
            .message("User details provided have an invalid format".to_string())
            .error(ApiError::new(
                "User details provided have an invalid format".to_string(),
            ));
    }
    match sqlx::query!(
        "SELECT * FROM users WHERE username = ? OR email = ?;",
        user_details.username,
        user_details.email
    )
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(rows) => {
            if rows.is_empty() {
                let user_id = match Tables::Users.gen_id(&state.db_pool).await {
                    Ok(id) => id,
                    Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while generating user id: {}", e.to_string())))
                };
                let hashed_password = match hash_password(user_details.password) {
                    Ok(p) => p,
                    Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while hashing password: {}", e.to_string()))),
                };
                let role = 0;
                let mut transaction = match state.db_pool.begin().await {
                    Ok(tx) => tx,
                    Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while preparing transaction to register user: {}", e.to_string())))
                    
                };
                match sqlx::query!("INSERT INTO users (id, username, first_name, last_name, email, password, role) VALUES (?, ?, ?, ?, ?, ?, ?);", user_id, user_details.username, user_details.first_name, user_details.last_name, user_details.email, hashed_password, role).execute(&mut *transaction).await{
          Ok(_) => {
            let user_row = UserRow {
              id: user_id.clone(),
              username: user_details.username,
              email: user_details.email,
              password: hashed_password,
              first_name: user_details.first_name,
              last_name: user_details.last_name,
              role: role.to_owned(),
            };

            if let Some(name) = user_details.family_name {
              let family_id = match Tables::Users.gen_id(&state.db_pool).await {
                Ok(id) => id,
                Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while generating family id: {}", e.to_string()))),
              };
            match sqlx::query!("INSERT INTO families (id, admin_id, name) VALUES (?, ?, ?);", family_id, user_id, name).execute(&mut *transaction).await {
              Ok(_) => {
                match transaction.commit().await {
                  Ok(_) => {
                    return set_tokens(user_row, state.db_pool).await
                  },
                  Err(e) => 
                  return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while inserting family details into database: {}", e.to_string())))
                }
              },
              Err(_) => {
                match transaction.rollback().await {
                  Ok(_) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while inserting user details into database. Transaction rolled back."))),
                  Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while inserting user details into database. Transaction roll back failed: {}", e.to_string()))),
                }
              }
            }
          } else {
            match transaction.commit().await {
              Ok(_) => return set_tokens(user_row, state.db_pool).await,
              Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while committing user details into database: {}", e.to_string()))),
            }
          }
          },
          Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while inserting user details into database: {}", e.to_string()))),
        }
            } else {
              return ApiResponse::new(StatusCode::CONFLICT).message("Username/email already exists".to_string()).error(ApiError::new(format!("Username/email already exists")));
            }
        }
        Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while registering user".to_string()).error(ApiError::new(format!("Internal server error while  looking up username/email in database: {}", e.to_string())))
        };    
}

#[axum::debug_handler]
pub async fn refresh_tokens(State(state): State<AppState>, jar: CookieJar) -> ApiResponse {
    let refresh_token = match jar.get("refresh_token") {
        Some(token) => token.value().to_owned(),
        None => {
          return ApiResponse::new(StatusCode::UNAUTHORIZED).message("Unauthorized".to_string())
            .error(ApiError::new("Refresh token not found".to_string()));
        }
    };
    let refresh_token = match RefreshToken::from_token(refresh_token) {
      Ok(token) => token,
      Err(e) => {
        return ApiResponse::new(StatusCode::UNAUTHORIZED).message("Unauthorized".to_string())
          .error(ApiError::new(format!("Error parsing refresh token: {}", e.to_string())));
      }
    };
    match refresh_token.verify(&state.db_pool).await {
        Ok(data) => {
          if let Ok(user_details) = Tables::Users.get_user_details(refresh_token.claims.user_id, &state.db_pool).await {
            return set_tokens(user_details, state.db_pool).await;
          } else {
            return ApiResponse::new(StatusCode::UNAUTHORIZED)
                .message("Unauthorized".to_string())
                .error(ApiError::new(format!(
                    "Error looking up user details: {}",
                    data.to_string()
                )));
          }
        },
        Err(e) => {
            return ApiResponse::new(StatusCode::UNAUTHORIZED)
                .message("Unauthorized".to_string())
                .error(ApiError::new(format!(
                    "Error verifying refresh token: {}",
                    e.to_string()
                )));
        },
    }
}

pub async fn logout(jar: CookieJar) -> CookieJar {
    jar.remove(
        Cookie::build(("refresh_token", ""))
            .secure(true)
            .http_only(true)
            .path("/api")
            .same_site(SameSite::Strict) //TODO: CHANGE FOR PROD
            .partitioned(true)
            .build()
            .encoded()
            .to_string(),
    )
}
pub async fn set_tokens(user_data: UserRow, conn: Pool<Sqlite>) -> ApiResponse {
    let access_token = match AccessToken::new(user_data.id.clone(), user_data.role) {
        Ok(token) => token,
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while generating access token".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while generating access token: {}",
                    e.to_string()
                )))
        }
    };
    let refresh_token = match RefreshToken::new(user_data.id.clone(), conn).await {
        Ok(token) => token,
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while generating refresh token".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while generating refresh token: {}",
                    e.to_string()
                )))
        }
    };
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.token))
        .secure(true)
        .http_only(true)
        .path("/api")
        .same_site(SameSite::Strict) //TODO: CHANGE FOR PROD
        .partitioned(true)
        .build()
        .encoded()
        .to_string();
    ApiResponse::new(StatusCode::OK)
        .json(json!({
            "access_token": access_token.token,
            "user_details": {
                "username": user_data.username,
                "email": user_data.email,
                "first_name": user_data.first_name,
                "last_name": user_data.last_name,
                "role": user_data.role
            }
        }))
        .cookies(vec![refresh_cookie])
}

pub async fn delete_user(
    State(state): State<AppState>,
    user: AccessToken,
    jar: CookieJar,
) -> ApiResponse {
    match sqlx::query!("DELETE FROM users WHERE id = ?;", user.claims.user_id)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => {
            logout(jar).await;
            return ApiResponse::new(StatusCode::OK)
                .message("User deleted successfully".to_string()).cookies(vec![]);
            
        }
        Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while deleting user".to_string())
            .error(ApiError::new(format!(
                "Internal server error while deleting user: {}",
                e.to_string()
            ))),
    }
}
