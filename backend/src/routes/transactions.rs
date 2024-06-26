use crate::{db::gen_id, jwt::AccessTokenClaims, AppState};
use axum::{
  extract::State,
  http::StatusCode,
  routing::{get, post},
  Form, Json, Router,
};
use chrono::DateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationError};

use super::{ApiError, OkRes};

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/new", post(new))
    .route("/all", get(get_transactions))
}

#[derive(Deserialize, Validate)]
pub struct TransactionItemReq {
  #[validate(length(min = 1))]
  desc: String,
  #[validate(custom(function = "validate_amount"))]
  amount: Decimal,
  category: String,
}

#[derive(Deserialize, Validate)]
pub struct TransactionReq {
  #[validate(length(min = 1))]
  desc: String,
  #[validate(custom(function = "validate_date"))]
  date: i64,
  #[validate(custom(function = "validate_amount"))]
  amount: Decimal,
  #[validate(custom(function = "validate_type"))]
  r#type: String,
  merchant: String,
  category: String,
  payment_source: String,
  items: Option<Vec<TransactionItemReq>>,
}

pub async fn new(
  State(state): State<crate::AppState>,
  user: AccessTokenClaims,
  Json(transaction): Json<TransactionReq>,
) -> Result<(StatusCode, String), ApiError> {
  if transaction.validate().err().is_some() {
    return Err(ApiError {
      code: StatusCode::BAD_REQUEST,
      message: "Details provided have an invalid format.".to_string(), // TODO: Try to also tell which fields are invalid.
    });
  }
  let transaction_id = match gen_id(crate::db::Tables::Transactions, &state.db_pool).await {
    Ok(id) => id,
    Err(e) => return Err(e),
  };
  let amount = transaction.amount.to_f32();
  match sqlx::query!("INSERT INTO transactions (id, user_id, description, date, amount, type, merchant, category, payment_source) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);", transaction_id, user.id, transaction.desc, transaction.date, amount, transaction.r#type, transaction.merchant, transaction.category, transaction.payment_source).execute(&state.db_pool).await{
    Ok(_) => {
      if let Some(items) = transaction.items {
        if items.is_empty() {
          Ok((
            StatusCode::OK,
            json!({"success": true, "data": {"transaction_id": transaction_id}}).to_string(),
          ))
        } else{
        let mut errors: Vec<String> = Vec::new();
        for item in items.iter() {
          let item_id = match gen_id(crate::db::Tables::TransactionItems, &state.db_pool).await {
            Ok(id) => id,
            Err(e) => return Err(e),
          };
          let amount = item.amount.to_f32();
          match sqlx::query!("INSERT INTO transaction_items (id, transaction_id, description, amount, category) VALUES (?, ?, ?, ?, ?)", item_id, transaction_id, item.desc, amount, item.category).execute(&state.db_pool).await {
            Ok(_) => continue,
            Err(_) => errors.push(item_id)
          }
          if errors.is_empty() {
            return Ok((
              StatusCode::OK,
              json!({"success": true, "data": {"transaction_id": transaction_id}}).to_string(),
            ));
          } else {
            return Ok((
              StatusCode::OK,
              json!({"success": false, "data": {"transaction_id": transaction_id, "failed_items": errors}}).to_string(),
            ));
          }
        }
        Ok(( // Assume everthing is okay and just return this...
          StatusCode::OK,
          json!({"success": true, "data": {"transaction_id": transaction_id}}).to_string(),
        ))
      }
      } else {
        Ok((
          StatusCode::OK,
          json!({"success": true, "data": {"transaction_id": transaction_id}}).to_string(),
        ))
      }
    },
    Err(_) => Err(ApiError {
      code: StatusCode::INTERNAL_SERVER_ERROR,
      message: "Error registering user".to_string(),
    }),
  }
}

pub fn validate_date(date: i64) -> Result<(), ValidationError> {
  if DateTime::from_timestamp(date, 0).is_none() {
    return Err(ValidationError::new("Invalid date"));
  }
  Ok(())
}

pub fn validate_amount(amount: &Decimal) -> Result<(), ValidationError> {
  if amount == &Decimal::new(0, 0) {
    return Err(ValidationError::new("Amount cannot be $0"));
  }
  Ok(())
}

pub fn validate_type(transaction_type: &String) -> Result<(), ValidationError> {
  if *transaction_type != "credit".to_string() && *transaction_type != "debit".to_string() {
    return Err(ValidationError::new("Invalid transaction type"));
  }
  Ok(())
}

#[derive(Clone, Serialize)]
pub struct TransactionRow {
  pub id: String,
  pub desc: String,
  pub date: i64,
  pub amount: f64,
  pub r#type: String,
  pub merchant: String,
  pub category: String,
  pub payment_source: String,
}

#[derive(Clone, Serialize)]
pub struct TransactionsRes {
  transaction: TransactionRow,
  items: Option<Vec<TransactionItem>>,
}

#[derive(Clone, Serialize)]
pub struct TransactionItem {
  pub desc: String,
  pub amount: f64,
  pub category: String,
}

pub async fn get_transactions(
  State(state): State<crate::AppState>,
  user: AccessTokenClaims,
) -> Result<(StatusCode, Json<OkRes<Vec<TransactionsRes>>>), ApiError> {
  let mut results: Vec<TransactionsRes> = Vec::new();
  match sqlx::query_as!(TransactionRow, "SELECT id, description AS desc, date, amount, type, merchant, category, payment_source FROM transactions WHERE user_id = ?;", user.id).fetch_all(&state.db_pool).await {
    Ok(rows) => {
      for row in rows.iter(){
        match sqlx::query_as!(TransactionItem, "SELECT description as desc, amount, category FROM transaction_items WHERE transaction_id = ?", row.id).fetch_all(&state.db_pool).await {
          Ok(items) => {
            results.push(TransactionsRes{transaction: row.clone(), items: Some(items)});
          }
          Err(e) => match e {
            sqlx::Error::RowNotFound => results.push(TransactionsRes{transaction: row.clone(), items: None}),
            _ => return Err(ApiError{code: StatusCode::INTERNAL_SERVER_ERROR, message: "Could not fetch transaction items from database".to_string()}),}
        }
      }
      Ok((StatusCode::OK, Json(OkRes{success: true, data: results})))
    },
    Err(e) => match e {
      sqlx::Error::RowNotFound => Ok((StatusCode::OK, Json(OkRes{success: true, data: results}))),
      _ => Err(ApiError{code: StatusCode::INTERNAL_SERVER_ERROR, message: "Could not fetch transactions from database".to_string()})
    }
  }
}
