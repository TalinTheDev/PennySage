use argon2::{
  password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::Error;

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

pub struct UserRecord {
  id: String,
  username: String,
  email: String,
  first_name: String,
  last_name: String,
  password: String,
  family_id: Option<String>,
  role: String,
}

pub async fn gen_id(
  table: Tables,
  connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<String, sqlx::Error> {
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
      Err(e) => return Err(e),
    }
  }
}

pub fn hash_password(password: String) -> Result<String, argon2::password_hash::errors::Error> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let hashed_password = argon2.hash_password(password.as_bytes(), &salt);

  match hashed_password {
    Ok(hp) => Ok(hp.to_string()),
    Err(e) => Err(e),
  }
}

pub fn verify_password(
  password: String,
  hash: String,
) -> Result<bool, argon2::password_hash::errors::Error> {
  let parsed_hash = PasswordHash::new(&hash)?;
  Ok(
    Argon2::default()
      .verify_password(password.as_bytes(), &parsed_hash)
      .is_ok(),
  )
}
