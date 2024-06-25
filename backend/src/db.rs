use argon2::{
  password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use axum::http::StatusCode;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::routes::ApiError;

#[derive(Serialize, Deserialize)]
pub enum Roles {
  User,
  Admin,
  Dev,
}

pub enum Tables {
  Users,
  Families,
  Transactions,
  TransactionItems,
  RefreshTokens,
}

pub async fn gen_id(
  table: Tables,
  connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<String, ApiError> {
  let mut id;
  let mut query = "SELECT * FROM".to_owned();
  match table {
    Tables::Users => query.push_str(" users WHERE id = ?"),
    Tables::Families => query.push_str(" families WHERE id = ?"),
    Tables::Transactions => query.push_str(" transactions WHERE id = ?"),
    Tables::TransactionItems => query.push_str(" transaction_items WHERE id = ?"),
    Tables::RefreshTokens => query.push_str(" refresh_tokens WHERE id = ?"),
  };

  loop {
    id = nanoid!();
    let statement = sqlx::query(&query).bind(&id);
    match statement.fetch_one(connection).await {
      Ok(_) => continue,
      Err(Error::RowNotFound) => return Ok(id),
      Err(_) => {
        return Err(ApiError {
          code: StatusCode::INTERNAL_SERVER_ERROR,
          message: "Error registering user".to_string(),
        })
      }
    }
  }
}

pub fn hash_password(password: String) -> Result<String, ApiError> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let hashed_password = argon2.hash_password(password.as_bytes(), &salt);

  match hashed_password {
    Ok(hp) => Ok(hp.to_string()),
    Err(_) => Err(ApiError {
      code: StatusCode::INTERNAL_SERVER_ERROR,
      message: "Error registering user".to_string(),
    }),
  }
}

pub fn verify_password(password: String, hash: String) -> Result<bool, ApiError> {
  let parsed_hash = match PasswordHash::new(&hash) {
    Ok(hash) => hash,
    Err(_) => {
      return Err(ApiError {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Internal server error while authenticating user".to_string(),
      });
    }
  };
  Ok(
    Argon2::default()
      .verify_password(password.as_bytes(), &parsed_hash)
      .is_ok(),
  )
}
