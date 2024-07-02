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
    .route("/register", post(register))
    .route("/refresh_tokens", post(refresh_tokens))
}

#[derive(Deserialize, Validate)]
pub struct LoginReq {
  #[validate(email(message = "Invalid Email"))]
  email: Option<String>,
  #[validate(length(min = 1, message = "Invalid username format"))]
  username: Option<String>,
  #[validate(length(min = 1, message = "Invalid password format"))]
  password: String,
}

#[axum::debug_handler]
pub async fn login(
  State(state): State<crate::AppState>,
  Json(user_details): Json<LoginReq>,
) -> ApiResponse {
  if user_details.email.is_none() && user_details.username.is_none() {
    return ApiResponse::Failure {
      status: StatusCode::BAD_REQUEST,
      error: ApiError::Simple("Email or username not provided".to_owned()),
    };
  }
  if user_details.validate().err().is_some() {
    // TODO: Tell which fields are invalid
    return ApiResponse::Failure {
      status: StatusCode::BAD_REQUEST,
      error: ApiError::Simple("User details provided have an invalid format".to_owned()),
    };
  }
  match sqlx::query!(
    "SELECT * FROM users WHERE username = ? OR email = ?",
    user_details.username,
    user_details.email
  )
  .fetch_one(&state.db_pool)
  .await
  {
    Ok(user) => match verify_password(user_details.password, user.password) {
      Ok(valid) => {
        if valid {
          set_tokens(user.id, user.role, state.db_pool).await
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
        let id = match gen_id(crate::db::Tables::Users, &state.db_pool).await {
          Ok(id) => id,
          Err(e) => return e,
        };
        let password = match hash_password(user_details.password) {
          Ok(p) => p,
          Err(e) => return e,
        };
        let role = "user";
        match sqlx::query!("INSERT INTO users (id, username, first_name, last_name, email, password, role) VALUES (?, ?, ?, ?, ?, ?, ?);", id, user_details.username, user_details.first_name, user_details.last_name, user_details.email, password, role).execute(&state.db_pool).await{
          Ok(_) => set_tokens(id, role.to_string(), state.db_pool).await,
          Err(_) => ApiResponse::Failure {
            status: StatusCode::UNAUTHORIZED,
            error: ApiError::Simple("Internal server error while registering user".to_owned()),
          }
        }
      } else {
        ApiResponse::Failure {
          status: StatusCode::CONFLICT,
          error: ApiError::Simple("Username/email already exists".to_owned()),
        }
      }
    }
    Err(_) => ApiResponse::Failure {
      status: StatusCode::UNAUTHORIZED,
      error: ApiError::Simple("Internal server error while registering user".to_owned()),
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
    Ok(data) => set_tokens(data.0.user_id, data.1, state.db_pool).await,
    Err(e) => e,
  }
}

async fn set_tokens(
  user_id: String,
  user_role: String,
  db: sqlx::Pool<sqlx::Sqlite>,
) -> ApiResponse {
  let access_token = match gen_access_token(user_id.clone(), user_role) {
    Ok(token) => token,
    Err(e) => return e,
  };
  let refresh_token = match gen_refresh_token(user_id, db).await {
    Ok(token) => token,
    Err(e) => return e,
  };
  let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
    .secure(true)
    .http_only(true)
    .same_site(SameSite::Strict)
    .build()
    .encoded()
    .to_string();
  ApiResponse::SuccessWithCookies {
    status: StatusCode::OK,
    data: Box::new(json!({"access_token": access_token})),
    cookies: vec![refresh_cookie],
  }
}
