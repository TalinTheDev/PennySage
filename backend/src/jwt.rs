use std::{env, error::Error};

use crate::db::gen_id;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
  exp: i64, // Expiry
  iat: i64, // Issued At
  pub user_id: String,
  pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
  exp: i64,    // Expiry
  iat: i64,    // Issued At
  jti: String, // JWT Token Identifier (saved in DB as id)
  user_id: String,
}

pub fn gen_access_token(user_id: String, role: String) -> Result<String, Box<dyn Error>> {
  let expiration = match Local::now().checked_add_signed(Duration::minutes(5)) {
    Some(time) => time.timestamp(),
    None => Local::now().timestamp(),
  };

  let issued_at = Local::now().timestamp();
  let claims = AccessTokenClaims {
    exp: expiration,
    iat: issued_at,
    user_id,
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
    Err(e) => Err(Box::new(e)),
  }
}

pub async fn gen_refresh_token(
  user_id: String,
  connection: sqlx::Pool<sqlx::Sqlite>,
) -> Result<String, Box<dyn Error>> {
  let expiration = match Local::now().checked_add_signed(Duration::days(7)) {
    Some(time) => time.timestamp(),
    None => Local::now().timestamp(),
  };

  let issued_at = Local::now().timestamp();
  let jti = match gen_id(crate::db::Tables::RefreshTokens, &connection).await {
    Ok(jti) => jti,
    Err(e) => return Err(Box::new(e)),
  };
  let claims = RefreshTokenClaims {
    exp: expiration,
    iat: issued_at,
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
            Err(e) => Err(Box::new(e))
          }
        },
        Err(e) => Err(Box::new(e))
      }
    },
    Err(e) => Err(Box::new(e)),
  }
}

pub fn verify_access_token(
  token: String,
) -> Result<AccessTokenClaims, jsonwebtoken::errors::Error> {
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
    Err(e) => Err(e),
  }
}

/// If refresh token is valid (exp) and isn't revoked from database, return the claims. If token invalid from validation or database revoked, error returned. If error while doing this, also returns that error
pub async fn verify_refresh_token(
  token: String,
  connection: sqlx::Pool<sqlx::Sqlite>,
) -> Result<RefreshTokenClaims, Box<dyn Error>> {
  match decode::<RefreshTokenClaims>(
    &token,
    &DecodingKey::from_secret(
      env::var("JWT_SECRET")
        .expect("JWT secret should be set in env file")
        .as_bytes(),
    ),
    &Validation::default(),
  ) {
    Ok(token) => {
      match sqlx::query!(
        "SELECT * FROM refresh_tokens WHERE id = ? AND user_id = ? AND revoked = 0",
        token.claims.jti,
        token.claims.user_id
      )
      .fetch_one(&connection)
      .await
      {
        Ok(_) => Ok(token.claims),
        Err(e) => Err(Box::new(e)),
      }
    }
    Err(e) => Err(Box::new(e)),
  }
}
