use std::env;

use crate::routes::auth::UserRow;
use crate::{
    db::gen_id,
    routes::{ApiError, ApiResponse},
};
use axum::http::StatusCode;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    exp: i64, // Expiry
    iat: i64, // Issued At
    pub id: String,
    pub role: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    exp: i64,    // Expiry
    iat: i64,    // Issued At
    jti: String, // JWT Token Identifier (saved in DB as id)
    pub user_id: String,
}

pub fn gen_access_token(user_id: String, role: i64) -> Result<String, ApiResponse> {
    let now = Local::now();
    let expiration = match now.checked_add_signed(Duration::minutes(5)) {
        Some(time) => time.format("%s").to_string(),
        None => now.format("%s").to_string(),
    };

    let issued_at = now.format("%s").to_string();
    let claims = AccessTokenClaims {
        exp: expiration.parse().unwrap(),
        iat: issued_at.parse().unwrap(),
        id: user_id,
        role,
    };
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT secret should be set in env file")
                .as_bytes(),
        ),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(ApiResponse::Failure {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiError::Simple(
                "Can't generate access token (internal server error)".to_string(),
            ),
        }),
    }
}

pub async fn gen_refresh_token(
    user_id: String,
    connection: sqlx::Pool<sqlx::Sqlite>,
) -> Result<String, ApiResponse> {
    let now = Local::now();
    let expiration = match now.checked_add_signed(Duration::days(7)) {
        Some(time) => time.format("%s").to_string(),
        None => now.format("%s").to_string(),
    };

    let issued_at = now.format("%s").to_string();
    let jti = match gen_id(crate::db::Tables::RefreshTokens, &connection).await {
        Ok(jti) => jti,
        Err(_) => {
            return Err(ApiResponse::Failure {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ApiError::Simple(
                    "Can't generate refresh token (internal server error)".to_string(),
                ),
            })
        }
    };
    let claims = RefreshTokenClaims {
        exp: expiration.parse().unwrap(),
        iat: issued_at.clone().parse().unwrap(),
        user_id: user_id.clone(),
        jti: jti.clone(),
    };
    match encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(
      env::var("JWT_REFRESH_SECRET")
        .expect("JWT secret should be set in env file")
        .as_bytes(),
    ),
  ) {
    Ok(token) => {
      match sqlx::query!("INSERT INTO refresh_tokens (id, user_id, revoked, created_at, updated_at) VALUES (?, ?, ?, ?, ?);", jti, user_id, 0, issued_at, issued_at).execute(&connection).await {
        Ok(_) => {
          match sqlx::query!("UPDATE refresh_tokens SET revoked = 1 WHERE id != ? AND user_id = ?", jti, user_id).execute(&connection).await {
            Ok(_) => Ok(token),
            Err(_) => Err(ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Simple("Can't generate refresh token (internal server error)".to_string()),
            })
          }
        },
        Err(_) => Err(ApiResponse::Failure {
          status: StatusCode::INTERNAL_SERVER_ERROR,
          error: ApiError::Simple("Can't generate refresh token (internal server error)".to_string()),
        })
      }
    },
    Err(_) => Err(ApiResponse::Failure {
      status: StatusCode::INTERNAL_SERVER_ERROR,
      error: ApiError::Simple("Can't generate refresh token (internal server error)".to_string()),
    })
  }
}

pub fn verify_access_token(token: String) -> Result<AccessTokenClaims, ApiResponse> {
    match decode::<AccessTokenClaims>(
        &token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT secret should be set in env file")
                .as_bytes(),
        ),
        &Validation::default(),
    ) {
        Ok(token) => Ok(token.claims),
        Err(_) => Err(ApiResponse::Failure {
            status: StatusCode::UNAUTHORIZED,
            error: ApiError::Simple("Access token could not be verified".to_string()),
        }),
    }
}

/// If refresh token is valid (exp) and isn't revoked from database, return the claims. If token invalid from validation or database revoked, error returned. If error while doing this, also returns that error
pub async fn verify_refresh_token(
    token: String,
    connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<(RefreshTokenClaims, UserRow), ApiResponse> {
    let auth_error = ApiResponse::Failure {
        status: StatusCode::UNAUTHORIZED,
        error: ApiError::Simple("Error refreshing tokens (unauthorized)".to_string()),
    };
    let server_error = ApiResponse::Failure {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        error: ApiError::Simple("Error refreshing tokens (internal server error)".to_string()),
    };
    match decode::<RefreshTokenClaims>(
    &token,
    &DecodingKey::from_secret(
      env::var("JWT_REFRESH_SECRET")
        .expect("JWT secret should be set in env file")
        .as_bytes(),
    ),
    &Validation::default(),
  ) {
    Ok(token) => {
      match sqlx::query_as!(UserRow,
        "SELECT id, username, email, password, first_name, last_name, role FROM users u WHERE u.id = (SELECT rt.user_id FROM refresh_tokens rt WHERE rt.id = ? AND rt.user_id = ? AND revoked = 0)",
        token.claims.jti,
        token.claims.user_id
      )
      .fetch_one(connection)
      .await
      {
        Ok(user) => Ok((token.claims, user)),
        Err(e) => {
          match e {
            sqlx::Error::RowNotFound => Err(auth_error),
            _ => Err(server_error)
          }
        }
      }
    }
    Err(e) => {
      println!("{:?}", e.kind());
      match e.into_kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::InvalidToken => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::InvalidSignature => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::MissingRequiredClaim(_) => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::InvalidIssuer => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::InvalidAudience => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::InvalidSubject => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::ImmatureSignature => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => Err(auth_error),
        jsonwebtoken::errors::ErrorKind::Utf8(_) => Err(auth_error),
        _ => Err(server_error),
      }
    }
  }
}
