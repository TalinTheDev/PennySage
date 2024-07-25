use super::{ApiError, ApiResponse};
use crate::{db::gen_id, jwt::AccessTokenClaims, AppState};
use axum::{
  extract::State,
  http::StatusCode,
  routing::{delete, get, post},
  Json, Router,
};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationError};

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/new", post(new))
    .route("/all", get(get_transactions))
    .route("/delete", delete(delete_transactions))
}

#[derive(Deserialize, Validate)]
pub struct TransactionItemReq {
  #[validate(length(min = 1))]
  desc: String,
  #[validate(custom(function = "validate_amount"))]
  amount: String,
  category: String,
}

#[derive(Deserialize, Validate)]
pub struct TransactionReq {
  #[validate(length(min = 1))]
  desc: String,
  #[validate(custom(function = "validate_date"))]
  date: i64,
  #[validate(custom(function = "validate_amount"))]
  amount: String,
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
) -> ApiResponse {
  if transaction.validate().err().is_some() {
    // TODO: Tell which fields are invalid
    return ApiResponse::Failure {
      status: StatusCode::BAD_REQUEST,
      error: ApiError::Simple("Transaction details provided have an invalid format".to_owned()),
    };
  }
  let transaction_id = match gen_id(crate::db::Tables::Transactions, &state.db_pool).await {
    Ok(id) => id,
    Err(e) => return e,
  };
  let mut tx = match state.db_pool.begin().await {
    Ok(tx) => tx,
    Err(_) => {
      return ApiResponse::Failure {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        error: ApiError::Simple(
          "Internal server error while preparing transaction to add transaction".to_owned(),
        ),
      }
    }
  };
  match sqlx::query!("INSERT INTO transactions (id, user_id, description, date, amount, type, merchant, category, payment_source) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);", transaction_id, user.id, transaction.desc, transaction.date, transaction.amount, transaction.r#type, transaction.merchant, transaction.category, transaction.payment_source).execute(&mut *tx).await{
    Ok(_) => {
      if let Some(items) = transaction.items {
        if items.is_empty() {
            match tx.commit().await {
              Ok(_) => return ApiResponse::Success { status: StatusCode::OK, data: Box::new(json!({"transaction_id": transaction_id})) },
              Err(_) => return ApiResponse::Failure {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ApiError::Simple("Internal server error while adding transaction".to_owned()),
              }
            }
        } else {
        for item in items.iter() {
          let item_id = match gen_id(crate::db::Tables::TransactionItems, &state.db_pool).await {
            Ok(id) => id,
            Err(e) => { return e; }
          };
          match sqlx::query!("INSERT INTO transaction_items (id, transaction_id, description, amount, category) VALUES (?, ?, ?, ?, ?)", item_id, transaction_id, item.desc, item.amount, item.category).execute(&mut *tx).await {
            Ok(_) => continue,
            Err(_) => {
              match tx.rollback().await {
                Ok(_) => return ApiResponse::Failure {
                  status: StatusCode::INTERNAL_SERVER_ERROR,
                  error: ApiError::Simple("Internal server error while adding transaction".to_owned()),
                },
                Err(_) => return ApiResponse::Failure {
                  status: StatusCode::INTERNAL_SERVER_ERROR,
                  error: ApiError::Simple("Internal server error while adding transaction".to_owned()),
                }
              }
            }
          }
        }
        match tx.commit().await {
          Ok(_) => return ApiResponse::Success { status: StatusCode::OK, data: Box::new(json!({"transaction_id": transaction_id})) },
          Err(_) => return ApiResponse::Failure {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiError::Simple("Internal server error while adding transaction".to_owned()),
          }
        }
      }
      } else {
          match tx.commit().await {
            Ok(_) => return ApiResponse::Success { status: StatusCode::OK, data: Box::new(json!({"transaction_id": transaction_id})) },
            Err(_) => return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Simple("Internal server error while adding transaction".to_owned()),
            }
          }
      }
    },
    Err(_) => ApiResponse::Failure {
      status: StatusCode::UNAUTHORIZED,
      error: ApiError::Simple("Internal server error while adding transaction".to_owned()),
    }
  }
}

pub fn validate_date(date: i64) -> Result<(), ValidationError> {
  if DateTime::from_timestamp(date, 0).is_none() {
    return Err(ValidationError::new("Invalid date"));
  }
  Ok(())
}

pub fn validate_amount(amount: &String) -> Result<(), ValidationError> {
  if amount == "0.00" {
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
) -> ApiResponse {
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
            _ => return ApiResponse::Failure { status: StatusCode::INTERNAL_SERVER_ERROR, error: ApiError::Simple("Could not fetch transaction items from database (internal server error)".to_owned()) }
          }
        }
      }
      ApiResponse::Success { status: StatusCode::OK, data: Box::new(results) }
    },
    Err(e) => match e {
      sqlx::Error::RowNotFound => ApiResponse::Success { status: StatusCode::OK, data: Box::new(results) },
      _ => ApiResponse::Failure { status: StatusCode::INTERNAL_SERVER_ERROR, error: ApiError::Simple("Could not fetch transactions from database (internal server error)".to_owned()) }
    }
  }
}

#[derive(Deserialize)]
pub struct DeleteReq {
  transactions: Vec<String>,
}

pub async fn delete_transactions(
  State(state): State<crate::AppState>,
  user: AccessTokenClaims,
  Json(data): Json<DeleteReq>,
) -> ApiResponse {
  if data.transactions.len() != 0 {
    let mut tx = match state.db_pool.begin().await {
      Ok(tx) => tx,
      Err(_) => {
        return ApiResponse::Failure {
          status: StatusCode::INTERNAL_SERVER_ERROR,
          error: ApiError::Simple(
            "Internal server error while preparing transaction to delete transactions".to_owned(),
          ),
        }
      }
    };
    for transaction in data.transactions {
      match sqlx::query!(
        "DELETE FROM transactions WHERE id = ? AND user_id = ?;",
        transaction,
        user.id
      )
      .execute(&mut *tx)
      .await
      {
        Ok(_) => continue,
        Err(_) => match tx.rollback().await {
          Ok(_) => {
            return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Simple(
                "Internal server error while deleting transaction".to_owned(),
              ),
            }
          }
          Err(_) => {
            return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Simple(
                "Internal server error while deleting transaction".to_owned(),
              ),
            }
          }
        },
      }
    }
    match tx.commit().await {
      Ok(_) => {
        return ApiResponse::Success {
          status: StatusCode::OK,
          data: Box::new("Transactions deleted successfully"),
        }
      }
      Err(_) => {
        return ApiResponse::Failure {
          status: StatusCode::INTERNAL_SERVER_ERROR,
          error: ApiError::Simple("Internal server error while deleting transactions".to_owned()),
        }
      }
    }
  } else {
    return ApiResponse::Success {
      status: StatusCode::OK,
      data: Box::new("No transactions to delete"),
    };
  }
}

pub fn edit_transaction() -> ApiResponse {
  ApiResponse::Success {
    status: StatusCode::OK,
    data: Box::new("Transaction edited successfully"),
  }
}
