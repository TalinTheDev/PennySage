use std::{fmt, iter::Successors};

use super::{ApiError, ApiResponse};
use crate::{db::gen_id, jwt::AccessTokenClaims, AppState};
use axum::{
  extract::Multipart,
  extract::State,
  http::StatusCode,
  routing::{delete, get, patch, post},
  Json, Router,
};
use chrono::DateTime;
use csv::Reader;
use futures_util::{stream::StreamExt, TryFutureExt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationError};

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/new", post(new))
    .route("/all", get(get_transactions))
    .route("/delete", delete(delete_transactions))
    .route("/edit", patch(edit_transaction))
    .route("/import", post(import_transactions))
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
enum TransactionType {
  Credit,
  Debit,
}
impl fmt::Display for TransactionType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      TransactionType::Credit => write!(f, "credit"),
      TransactionType::Debit => write!(f, "debit"),
    }
  }
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
  date: String,
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

pub fn validate_date(date: &String) -> Result<(), ValidationError> {
  if DateTime::parse_from_rfc3339(date.as_str()).is_err() {
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
  pub date: String,
  pub amount: f64,
  pub r#type: String,
  pub merchant: String,
  pub category: String,
  pub payment_source: String,
}

#[derive(Clone, Serialize)]
pub struct TransactionRowWithItems {
  pub id: String,
  pub desc: String,
  pub date: String,
  pub amount: f64,
  pub r#type: String,
  pub merchant: String,
  pub category: String,
  pub payment_source: String,
  pub items: Option<Vec<TransactionItem>>,
}

#[derive(Clone, Serialize)]
pub struct TransactionsRes {
  transaction: TransactionRowWithItems,
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
  let mut results: Vec<TransactionRowWithItems> = Vec::new();
  match sqlx::query_as!(TransactionRow, "SELECT id, description AS desc, date, amount, type, merchant, category, payment_source FROM transactions WHERE user_id = ?;", user.id).fetch_all(&state.db_pool).await {
        Ok(rows) => {
            for row in rows.iter(){
                match sqlx::query_as!(TransactionItem, "SELECT description as desc, amount, category FROM transaction_items WHERE transaction_id = ?", row.id).fetch_all(&state.db_pool).await {
                    Ok(items) => {
                        let transaction = row.clone();
                        results.push(TransactionRowWithItems {
                            id: transaction.id, desc: transaction.desc, date: transaction.date, amount: transaction.amount, merchant: transaction.merchant, category: transaction.category, payment_source: transaction.payment_source, r#type: transaction.r#type, items: Some(items)
                        }
                        );
                    }
                    Err(e) => match e {
                        sqlx::Error::RowNotFound =>{let transaction = row.clone(); results.push(TransactionRowWithItems {
                            id: transaction.id, desc: transaction.desc, date: transaction.date, amount: transaction.amount, merchant: transaction.merchant, category: transaction.category, payment_source: transaction.payment_source, r#type: transaction.r#type, items: None
                        }
                        );},
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

#[derive(Deserialize, Validate)]
pub struct EditField {
  #[validate(length(min = 1))]
  field: String,
  #[validate(length(min = 1))]
  value: String,
}

#[derive(Deserialize, Validate)]
pub struct EditReq {
  #[validate(length(min = 1))]
  id: String,
  edits: Vec<EditField>,
}

pub async fn edit_transaction(
  State(state): State<crate::AppState>,
  user: AccessTokenClaims,
  Json(data): Json<EditReq>,
) -> ApiResponse {
  if data.edits.len() == 0 {
    return ApiResponse::Success {
      status: StatusCode::OK,
      data: Box::new(format!("Nothing to update in transaction #{}", data.id)),
    };
  } else {
    let mut query: sqlx::QueryBuilder<'_, sqlx::Sqlite> =
      sqlx::QueryBuilder::new("UPDATE transactions SET ".to_owned());
    for (i, edit) in data.edits.iter().enumerate() {
      if edit.field.as_str() == "id" {
        continue;
      }
      if i > 0 {
        query.push(", ");
      }
      match edit.field.as_str() {
        "desc" => query.push("description = "),
        "date" => query.push("date = "),
        "amount" => query.push("amount = "),
        "merchant" => query.push("merchant = "),
        "category" => query.push("category = "),
        "payment_source" => query.push("payment_source = "),
        "type" => query.push("type = "),
        &_ => continue,
      };
      query.push_bind(edit.value.clone());
    }
    query.push(" WHERE id = ");
    query.push_bind(data.id);
    query.push(" AND user_id = ");
    query.push_bind(user.id);
    if let Err(e) = query.build().execute(&state.db_pool).await {
      return ApiResponse::Failure {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        error: ApiError::Complex {
          message: "Internal server error while editing transaction".to_owned(),
          error: Box::new(e.to_string()),
        },
      };
    }
  }
  ApiResponse::Success {
    status: StatusCode::OK,
    data: Box::new("Transaction edited successfully"),
  }
}

#[derive(Debug, Deserialize)]
struct ImportedTransaction {
  description: String,
  date: String,
  amount: String,
  r#type: TransactionType,
  merchant: String,
  category: String,
  payment_source: String,
}

pub async fn import_transactions(
  State(state): State<crate::AppState>,
  user: AccessTokenClaims,
  mut multipart: Multipart,
) -> ApiResponse {
  let total_transactions;
  let mut failed_transactions = 0;
  let form_data = multipart.next_field().await;
  match form_data {
    Err(e) => {
      return ApiResponse::Failure {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        error: ApiError::Complex {
          message: String::from("Error while reading form data"),
          error: Box::new(e.body_text()),
        },
      };
    }
    Ok(field) => {
      if let Some(file) = field {
        let data = match file.text().await {
          Ok(data) => data,
          Err(e) => {
            return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Complex {
                message: String::from("Error while reading file"),
                error: Box::new(e.body_text()),
              },
            };
          }
        };
        let mut rdr = Reader::from_reader(data.as_bytes());
        let mut tx = match state.db_pool.begin().await {
          Ok(tx) => tx,
          Err(_) => {
            return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Simple(
                "Internal server error while preparing transaction to import transactions"
                  .to_owned(),
              ),
            }
          }
        };
        total_transactions = rdr.records().count();
        for result in rdr.deserialize() {
          let transaction: ImportedTransaction = match result {
            Ok(r) => r,
            Err(_) => {
              failed_transactions += 1;
              continue;
            }
          };
          let transaction_id = match gen_id(crate::db::Tables::Transactions, &state.db_pool).await {
            Ok(id) => id,
            Err(e) => return e,
          };
          match sqlx::query!("INSERT INTO transactions (id, user_id, description, date, amount, type, merchant, category, payment_source) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);", transaction_id, user.id, transaction.description, transaction.date, transaction.amount, transaction.r#type, transaction.merchant, transaction.category, transaction.payment_source).execute(&mut *tx).await {
                Ok(_) => continue,
                Err(_) => {
                    failed_transactions += 1;
                    continue;
                }
            }
        }
        match tx.commit().await {
          Ok(_) => {
            return ApiResponse::Success {
              status: StatusCode::OK,
              data: Box::new(
                json!({"message": "Transactions imported successfully", "transaction_count": (total_transactions - failed_transactions)}),
              ),
            };
          }
          Err(e) => {
            return ApiResponse::Failure {
              status: StatusCode::INTERNAL_SERVER_ERROR,
              error: ApiError::Complex {
                message: "Internal server error while importing transactions".to_owned(),
                error: Box::new(e.to_string()),
              },
            }
          }
        }
      } else {
        return ApiResponse::Failure {
          status: StatusCode::BAD_REQUEST,
          error: ApiError::Simple("No file provided".to_owned()),
        };
      }
    }
  }
}
