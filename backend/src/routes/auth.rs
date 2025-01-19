use crate::{
  db::{gen_id, hash_password, verify_password},
  jwt::{gen_access_token, gen_refresh_token, verify_refresh_token},
  routes::{ApiError, ApiResponse},
  AppState,
};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_extra::extract::{
  cookie::{self, SameSite},
  CookieJar,
};
use cookie::Cookie;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/login", post(login))
    .route("/logout", post(logout))
    .route("/register", post(register))
    .route("/refresh_tokens", post(refresh_tokens))
}

#[derive(Deserialize, Validate)]
pub struct LoginReq {
  #[validate(length(min = 1, message = "Invalid username format"))]
  username: String,
  #[validate(length(min = 1, message = "Invalid password format"))]
  password: String,
}

pub struct UserRow {
  pub id: String,
  pub username: String,
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub role: i64,
}

#[axum::debug_handler]
pub async fn login(
  State(state): State<AppState>,
  Json(user_details): Json<LoginReq>,
) -> ApiResponse {
  if user_details.validate().err().is_some() {
    // TODO: Tell which fields are invalid
    return ApiResponse::Failure {
      status: StatusCode::BAD_REQUEST,
      error: ApiError::Simple("User details provided have an invalid format".to_owned()),
    };
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
          set_tokens(user, state.db_pool).await
        } else {
          return ApiResponse::Failure {
            status: StatusCode::UNAUTHORIZED,
            error: ApiError::Simple("Invalid username/password combo".to_owned()),
          };
        }
      }
      Err(e) => e,
    },
    Err(sqlx::Error::RowNotFound) => ApiResponse::Failure {
      status: StatusCode::UNAUTHORIZED,
      error: ApiError::Simple("Invalid username/password combo".to_owned()),
    },
    Err(_) => ApiResponse::Failure {
      status: StatusCode::UNAUTHORIZED,
      error: ApiError::Simple("Internal server error while looking up user".to_owned()),
    },
  }
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
  State(state): State<crate::AppState>,
  Json(user_details): Json<RegisterReq>,
) -> ApiResponse {
  if user_details.validate().err().is_some() {
    return ApiResponse::Failure {
      status: StatusCode::BAD_REQUEST,
      error: ApiError::Simple("User details provided have an invalid format".to_owned()),
    };
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
        let user_id = match gen_id(crate::db::Tables::Users, &state.db_pool).await {
          Ok(id) => id,
          Err(e) => return e,
        };
        let hashed_password = match hash_password(user_details.password) {
          Ok(p) => p,
          Err(e) => return e,
        };
        let role = 0;
        let mut transaction = match state.db_pool.begin().await {
          Ok(tx) => tx,
          Err(_) => {
            return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Simple(
                "Internal server error while preparing transaction to register user".to_owned(),
              ),
            }
          }
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
              let family_id = match gen_id(crate::db::Tables::Users, &state.db_pool).await {
                Ok(id) => id,
                Err(e) => return e,
              };
            match sqlx::query!("INSERT INTO families (id, admin_id, name) VALUES (?, ?, ?);", family_id, user_id, name).execute(&mut *transaction).await {
              Ok(_) => {
                match transaction.commit().await {
                  Ok(_) => {
                    return set_tokens(user_row, state.db_pool).await
                  },
                  Err(e) => return ApiResponse::Failure {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    error: ApiError::Complex{message: "Internal server error while registering user".to_owned(), error: Box::new(e.to_string())},
                  }
                }
              },
              Err(_) => {
                match transaction.rollback().await {
                  Ok(_) => return ApiResponse::Failure {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    error: ApiError::Simple("Internal server error while registering user".to_owned()),
                  },
                  Err(e) => return ApiResponse::Failure {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    error: ApiError::Complex{message: "Internal server error while registering user".to_owned(), error: Box::new(e.to_string())},
                  }
                }
              }
            }
          } else {
            match transaction.commit().await {
              Ok(_) => return set_tokens(user_row, state.db_pool).await,
              Err(e) => return ApiResponse::Failure {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ApiError::Complex{message: "Internal server error while registering user".to_owned(), error: Box::new(e.to_string())},
              }
            }
          }
          },
          Err(e) => ApiResponse::Failure {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiError::Complex{message: "Internal server error while registering user".to_owned(), error: Box::new(e.to_string())},
          }
        }
      } else {
        ApiResponse::Failure {
          status: StatusCode::CONFLICT,
          error: ApiError::Simple("Username/email already exists".to_owned()),
        }
      }
    }
    Err(e) => ApiResponse::Failure {
      status: StatusCode::UNAUTHORIZED,
      error: ApiError::Complex {
        message: "Internal server error while registering user".to_owned(),
        error: Box::new(e.to_string()),
      },
    },
  }
}

#[axum::debug_handler]
pub async fn refresh_tokens(State(state): State<crate::AppState>, jar: CookieJar) -> ApiResponse {
  let refresh_token = match jar.get("refresh_token") {
    Some(token) => token.value().to_owned(),
    None => {
      return ApiResponse::Failure {
        status: StatusCode::UNAUTHORIZED,
        error: ApiError::Simple("Refresh token not found".to_owned()),
      };
    }
  };
  match verify_refresh_token(refresh_token, &state.db_pool).await {
    Ok(data) => set_tokens(data.1, state.db_pool).await,
    Err(e) => e,
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

pub async fn set_tokens(user_data: UserRow, db: sqlx::Pool<sqlx::Sqlite>) -> ApiResponse {
  let access_token = match gen_access_token(user_data.id.clone(), user_data.role) {
    Ok(token) => token,
    Err(e) => return e,
  };
  let refresh_token = match gen_refresh_token(user_data.id, db).await {
    Ok(token) => token,
    Err(e) => return e,
  };
  let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
    .secure(true)
    .http_only(true)
    .path("/api")
    .same_site(SameSite::Strict)
    .partitioned(true)
    .build()
    .encoded()
    .to_string();
  ApiResponse::SuccessWithCookies {
    status: StatusCode::OK,
    data: Box::new(json!({
        "access_token": access_token, "user_details": {
        "username": user_data.username,
        "email": user_data.email,
        "first_name": user_data.first_name,
        "last_name": user_data.last_name,
        "role": user_data.role}
    })),
    cookies: vec![refresh_cookie],
  }
}
