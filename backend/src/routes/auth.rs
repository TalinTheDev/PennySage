use crate::{
  db::{gen_id, hash_password, verify_password},
  jwt::{gen_access_token, gen_refresh_token, verify_refresh_token, RefreshTokenVerificationError},
  AppState,
};
use axum::{extract::State, http::StatusCode, routing::post, Form, Router};
use axum_extra::extract::{
  cookie::{self, SameSite},
  CookieJar,
};
use cookie::Cookie;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/login", post(login))
    .route("/register", post(register))
    .route("/refresh_tokens", post(refresh_tokens))
}

#[derive(Deserialize)]
pub struct LoginReq {
  username: String,
  password: String,
}

#[axum::debug_handler]
pub async fn login(
  State(state): State<crate::AppState>,
  jar: CookieJar,
  Form(user_details): Form<LoginReq>,
) -> Result<(StatusCode, CookieJar, String), (StatusCode, String)> {
  match sqlx::query!(
    "SELECT * FROM users WHERE username = ? OR email = ?",
    user_details.username,
    user_details.username
  )
  .fetch_one(&state.db_pool)
  .await
  {
    Ok(user) => match verify_password(user_details.password, user.password) {
      Ok(valid) => {
        if valid {
          match set_tokens(user.id, user.role, jar, state.db_pool).await {
            Ok(tokens) => Ok((StatusCode::OK, tokens.0, tokens.1)),
            Err(e) => Err(e),
          }
        } else {
          Err((
            StatusCode::UNAUTHORIZED,
            json!({"success": false, "message": "Invalid username/password combo."}).to_string(),
          ))
        }
      }
      Err(_) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": "Internal server error while authenticating user"})
          .to_string(),
      )),
    },
    Err(sqlx::Error::RowNotFound) => Err((
      StatusCode::UNAUTHORIZED,
      json!({"success": false, "message": "Invalid username/password combo."}).to_string(),
    )),
    Err(_) => Err((
      StatusCode::INTERNAL_SERVER_ERROR,
      json!({"success": false, "message": "Internal server error while looking up user"})
        .to_string(),
    )),
  }
}

#[derive(Deserialize)]
pub struct RegisterReq {
  username: String,
  email: String,
  password: String,
  first_name: String,
  last_name: String,
}

pub async fn register(
  State(state): State<crate::AppState>,
  jar: CookieJar,
  Form(user_details): Form<RegisterReq>,
) -> Result<(StatusCode, CookieJar, String), (StatusCode, String)> {
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
          Err(_) => {
            return Err((
              StatusCode::INTERNAL_SERVER_ERROR,
              json!({"success": false, "message": "Error registering user"}).to_string(),
            ))
          }
        };
        let password = match hash_password(user_details.password) {
          Ok(p) => p,
          Err(_) => {
            return Err((
              StatusCode::INTERNAL_SERVER_ERROR,
              json!({"success": false, "message": "Error registering user"}).to_string(),
            ))
          }
        };
        let role = "user";
        match sqlx::query!("INSERT INTO users (id, username, first_name, last_name, email, password, role) VALUES (?, ?, ?, ?, ?, ?, ?);", id, user_details.username, user_details.first_name, user_details.last_name, user_details.email, password, role).execute(&state.db_pool).await{
          Ok(_) => {
            match set_tokens(id, role.to_string(), jar, state.db_pool).await {
              Ok(tokens) => Ok((StatusCode::OK, tokens.0, tokens.1)),
              Err(e) => Err(e),
            }
          },
          Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": "Error registering user"}).to_string(),
          )),
        }
      } else {
        Err((
          StatusCode::CONFLICT,
          json!({"success": false, "message": "Email or Username Already Exists"}).to_string(),
        ))
      }
    }
    Err(_) => Err((
      StatusCode::INTERNAL_SERVER_ERROR,
      json!({"success": false, "message": "Error registering user"}).to_string(),
    )),
  }
}
#[axum::debug_handler]
pub async fn refresh_tokens(
  State(state): State<crate::AppState>,
  jar: CookieJar,
) -> Result<(StatusCode, CookieJar, String), (StatusCode, String)> {
  let refresh_token = match jar.get("refresh_token") {
    Some(token) => token.value().to_owned(),
    None => {
      return Err((
        StatusCode::UNAUTHORIZED,
        json!({"success": false, "message": "Refresh token not found"}).to_string(),
      ))
    }
  };
  match verify_refresh_token(refresh_token, &state.db_pool).await {
    Ok(data) => match set_tokens(data.0.user_id, data.1, jar, state.db_pool).await {
      Ok(tokens) => Ok((StatusCode::OK, tokens.0, tokens.1)),
      Err(e) => Err(e),
    },
    Err(e) => match e {
      RefreshTokenVerificationError::InvalidToken => Err((
        StatusCode::UNAUTHORIZED,
        json!({"success": false, "message": "Error refreshing tokens (unauthorized)"}).to_string(),
      )),
      RefreshTokenVerificationError::ServerError => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": "Error refreshing tokens"}).to_string(),
      )),
    },
  }
}

async fn set_tokens(
  user_id: String,
  user_role: String,
  jar: CookieJar,
  db: sqlx::Pool<sqlx::Sqlite>,
) -> Result<(CookieJar, String), (StatusCode, String)> {
  let access_token = match gen_access_token(user_id.clone(), user_role) {
    Ok(token) => token,
    Err(_) => {
      return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": "Can't generate access token"}).to_string(),
      ))
    }
  };
  let refresh_token = match gen_refresh_token(user_id, db).await {
    Ok(token) => token,
    Err(_) => {
      return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": "Can't generate refresh token"}).to_string(),
      ))
    }
  };
  let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
    .secure(true)
    .http_only(true)
    .same_site(SameSite::Strict);
  Ok((
    jar.add(refresh_cookie),
    json!({"success": true, "data": {"access_token": access_token}}).to_string(),
  ))
}
