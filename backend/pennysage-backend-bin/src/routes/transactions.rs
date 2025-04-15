use axum::{
    extract::Multipart,
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::DateTime;
use csv::Reader;
use lib::db::tables::Tables;
use lib::types::api_return::{ApiError, ApiResponse};
use lib::types::jwt::AccessToken;
use lib::types::state::AppState;
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

#[derive(Deserialize)]
pub struct TransactionItemReq {
    desc: String,
    amount: String,
    category: String,
}

#[derive(Deserialize)]
pub struct TransactionReq {
    desc: String,
    date: String,
    amount: String,
    merchant: String,
    category: String,
    payment_source: String,
    items: Option<Vec<TransactionItemReq>>,
}

pub async fn new(
    State(state): State<AppState>,
    user: AccessToken,
    Json(transaction): Json<TransactionReq>,
) -> ApiResponse {
    let mut credit_amount = 0.00;
    let mut debit_amount = 0.00;
    if validate_date(&transaction.date).is_err() {
        // TODO: Tell which fields are invalid
        return ApiResponse::new(StatusCode::BAD_REQUEST)
            .message("Transaction details provided have an invalid format".to_string())
            .error(ApiError::new(
                "Transaction details provided have an invalid format".to_string(),
            ));
    }
    let transaction_id = match Tables::Transactions.gen_id(&state.db_pool).await {
        Ok(id) => id,
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while generating transaction ID".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while generating transaction ID: {}",
                    e.to_string()
                )))
        }
    };
    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while adding transaction".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while preparing transaction to add transaction: {}",
                    e.to_string()
                )))
        }
    };
    match validate_amount(&transaction.amount) {
        Ok(amount) => {
            if amount < 0.00 {
                debit_amount = amount.abs();
            } else if amount > 0.00 {
                credit_amount = amount.abs()
            }
        }
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while adding transaction".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while parsing amount: {}",
                    e.to_string()
                )));
        }
    }

    match sqlx::query!("INSERT INTO transactions (id, user_id, description, date, credit_amount, debit_amount, merchant, category, payment_source) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);", transaction_id, user.claims.user_id, transaction.desc, transaction.date, credit_amount, debit_amount, transaction.merchant, transaction.category, transaction.payment_source).execute(&mut *tx).await{
        Ok(_) => {
            if let Some(items) = transaction.items {
                if items.is_empty() {
                    match tx.commit().await {
                        Ok(_) => return ApiResponse::new(StatusCode::OK).message("Transaction added successfully".to_string()).json(json!({"transaction_id": transaction_id})),
                        Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction".to_string()).error(ApiError::new(format!("Internal server error while committing transaction  to database: {}", e.to_string()))),
                    }
                } else {
                    for item in items.iter() {
                        let item_id = match Tables::TransactionItems.gen_id(&state.db_pool).await {
                            Ok(id) => id,
                            Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                            .message("Internal server error while generating transaction item ID".to_string())
                            .error(ApiError::new(format!("Internal server error while generating transaction item ID: {}", e.to_string()),
                            )),
                        };
                        let amount = match validate_amount(&item.amount) {
                          Ok(amount) => amount,
                          Err(e) => {
                            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction item".to_string()).error(ApiError::new(format!("Internal server error while parsing amount for transaction item: {}", e.to_string())));
                          }
                        };
                        match sqlx::query!("INSERT INTO transaction_items (id, transaction_id, description, amount, category) VALUES (?, ?, ?, ?, ?)", item_id, transaction_id, item.desc, amount, item.category).execute(&mut *tx).await {
                            Ok(_) => continue,
                            Err(_) => {
                                match tx.rollback().await {
                                    Ok(_) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction".to_string()).error(ApiError::new(format!("Internal server error while padding transaction. Database transaction rolled back"))),
                                    Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction".to_string()).error(ApiError::new(format!("Internal server error while inserting transaction details into database. Transaction roll back failed: {}", e.to_string()))),
                                }
                            }
                        }
                    }
                    match tx.commit().await {
                        Ok(_) => return ApiResponse::new(StatusCode::OK).message("Transaction & items added successfully".to_string()).json(json!({"transaction_id": transaction_id})),
                        Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction".to_string()).error(ApiError::new(format!("Internal server error while committing transaction items to database: {}", e.to_string()))),
                        }
                    }
            } else {
                match tx.commit().await {
                    Ok(_) => return ApiResponse::new(StatusCode::OK).message("Transaction added successfully".to_string()).json(json!({"transaction_id": transaction_id})),
                    Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction".to_string()).error(ApiError::new(format!("Internal server error while committing transaction items to database: {}", e.to_string()))),
                }
            }
        },
        Err(e) => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while adding transaction".to_string()).error(ApiError::new(format!("Internal server error while adding transaction to database: {}", e.to_string()))),
    }
}

pub fn validate_date(date: &String) -> Result<(), ValidationError> {
    if DateTime::parse_from_rfc3339(date.as_str()).is_err() {
        return Err(ValidationError::new("Invalid date"));
    }
    Ok(())
}

pub fn validate_amount(amount: &String) -> Result<f64, ValidationError> {
    match amount.parse::<f64>() {
        Ok(amount) => {
            if amount == 0.00 {
                Err(ValidationError::new("Amount cannot be $0"))
            } else {
                Ok(amount)
            }
        }
        Err(_) => Err(ValidationError::new("Invalid amount")),
    }
}

#[derive(Clone, Serialize)]
pub struct TransactionRow {
    pub id: String,
    pub desc: String,
    pub date: String,
    pub credit_amount: f64,
    pub debit_amount: f64,
    pub merchant: String,
    pub category: String,
    pub payment_source: String,
}

#[derive(Clone, Serialize)]
pub struct TransactionRowWithItems {
    pub id: String,
    pub desc: String,
    pub date: String,
    pub amount: f64, // Only has amount field here because this is what gets sent to client (not credit_amount or debit_amount)
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

pub async fn get_transactions(State(state): State<AppState>, user: AccessToken) -> ApiResponse {
    let mut results: Vec<TransactionRowWithItems> = Vec::new();
    match sqlx::query_as!(TransactionRow, "SELECT id, description AS desc, date, credit_amount, debit_amount, merchant, category, payment_source FROM transactions WHERE user_id = ?;", user.claims.user_id).fetch_all(&state.db_pool).await {
        Ok(rows) => {
            for row in rows.iter(){
              let amount;
              if row.credit_amount > 0.00 {
                amount = row.credit_amount;
              } else if row.debit_amount > 0.00 {
                amount = -1.00 * row.debit_amount;
              } else {
                amount = 0.00;
              }
                match sqlx::query_as!(TransactionItem, "SELECT description as desc, amount, category FROM transaction_items WHERE transaction_id = ?", row.id).fetch_all(&state.db_pool).await {
                    Ok(items) => {
                        let transaction = row.clone();
                        results.push(TransactionRowWithItems {
                            id: transaction.id, desc: transaction.desc, date: transaction.date, amount: amount, merchant: transaction.merchant, category: transaction.category, payment_source: transaction.payment_source, items: Some(items)
                        });
                    }
                    Err(e) => match e {
                        sqlx::Error::RowNotFound =>{
                          let transaction = row.clone();
                          results.push(TransactionRowWithItems {
                            id: transaction.id, desc: transaction.desc, date: transaction.date, amount: amount, merchant: transaction.merchant, category: transaction.category, payment_source: transaction.payment_source, items: None
                          });
                        },
                        e => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while fetching transactions".to_string()).error(ApiError::new(format!("Could not fetch transaction items from database (internal server error): {}", e.to_string())))
                    }
                }
            }
            ApiResponse::new(StatusCode::OK).message("Transactions fetched successfully".to_string()).json(json!({"transactions": results}))
        },
        Err(e) => match e {
            sqlx::Error::RowNotFound => ApiResponse::new(StatusCode::OK).message("Transactions fetched successfully".to_string()).json(json!({"transactions": results})),
            _ => return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while fetching transactions".to_string()).error(ApiError::new(format!("Could not fetch transaction from database (internal server error): {}", e.to_string())))
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteReq {
    transactions: Vec<String>,
}

pub async fn delete_transactions(
    State(state): State<AppState>,
    user: AccessToken,
    Json(data): Json<DeleteReq>,
) -> ApiResponse {
    if data.transactions.len() != 0 {
        let mut tx = match state.db_pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while deleting transactions".to_string()).error(ApiError::new(format!("Internal server error while preparing transaction to delete transactions: {}", e.to_string())));
            }
        };
        for transaction in data.transactions {
            match sqlx::query!(
                "DELETE FROM transactions WHERE id = ? AND user_id = ?;",
                transaction,
                user.claims.user_id
            )
            .execute(&mut *tx)
            .await
            {
                Ok(_) => continue,
                Err(e) => match tx.rollback().await {
                    Ok(_) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error while deleting transactions".to_string()).error(ApiError::new(format!("Internal server error while deleting transaction: {}. Transaction rolled back;", e.to_string())));
                    }
                    Err(_) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR).message("Internal server error wile deleting transactions".to_string()).error(ApiError::new(format!("Internal server error while deleting transaction: {}. Transaction roll back failed.", e.to_string())));
                    }
                },
            }
        }
        match tx.commit().await {
            Ok(_) => {
                return ApiResponse::new(StatusCode::OK)
                    .message("Transactions deleted successfully".to_string());
            }
            Err(e) => {
                return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
          .message("Internal server error while deleting transactions".to_string())
          .error(ApiError::new(format!("Internal server error while committing transaction to delete transactions: {}", e.to_string())));
            }
        }
    } else {
        return ApiResponse::new(StatusCode::OK).message("No transactions to delete".to_string());
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
    State(state): State<AppState>,
    user: AccessToken,
    Json(data): Json<EditReq>,
) -> ApiResponse {
    if data.edits.len() == 0 {
        return ApiResponse::new(StatusCode::OK)
            .message("Nothing to update in transaction".to_string());
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
                "amount" => {
                    let mut debit_amount = 0.00;
                    let mut credit_amount = 0.00;
                    match validate_amount(&edit.value) {
                        Ok(amount) => {
                            if amount < 0.00 {
                                debit_amount = amount.abs();
                            } else if amount > 0.00 {
                                credit_amount = amount.abs()
                            }
                        }
                        Err(e) => {
                            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                                .message(
                                    "Internal server error while updating transaction".to_string(),
                                )
                                .error(ApiError::new(format!(
                                    "Internal server error while parsing amount: {}",
                                    e.to_string()
                                )));
                        }
                    }
                    query.push("credit_amount = ");
                    query.push_bind(credit_amount);
                    query.push(", ");
                    query.push("debit_amount = ");
                    query.push_bind(debit_amount);
                    continue;
                }
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
        query.push_bind(user.claims.user_id);
        if let Err(e) = query.build().execute(&state.db_pool).await {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Internal server error while editing transaction".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while editing transaction: {}",
                    e.to_string()
                )));
        }
    }
    ApiResponse::new(StatusCode::OK).message("Transaction edited successfully".to_string())
}

#[derive(Debug, Deserialize)]
struct ImportedTransaction {
    description: String,
    date: String,
    amount: String,
    merchant: String,
    category: String,
    payment_source: String,
}

pub async fn import_transactions(
    State(state): State<AppState>,
    user: AccessToken,
    mut multipart: Multipart,
) -> ApiResponse {
    let mut total_transactions = 0;
    let mut failed_transactions = 0;
    let form_data = multipart.next_field().await;
    match form_data {
        Err(e) => {
            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("Error while reading transaction data".to_string())
                .error(ApiError::new(format!(
                    "Internal server error while reading form data: {}",
                    e.to_string()
                )));
        }
        Ok(field) => {
            if let Some(file) = field {
                let data = match file.text().await {
                    Ok(data) => data,
                    Err(e) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                            .message("Error while reading transaction data".to_string())
                            .error(ApiError::new(format!(
                                "Internal server error while reading import file: {}",
                                e.to_string()
                            )));
                    }
                };
                let mut rdr = Reader::from_reader(data.as_bytes());
                let mut tx = match state.db_pool.begin().await {
                    Ok(tx) => tx,
                    Err(e) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
              .message("Internal server error while importing transactions".to_string())
              .error(ApiError::new(format!("Internal server error while preparing transaction to import transactions: {}", e.to_string())));
                    }
                };
                for result in rdr.deserialize() {
                    total_transactions += 1;
                    let transaction: ImportedTransaction = match result {
                        Ok(r) => r,
                        Err(_) => {
                            failed_transactions += 1;
                            continue;
                        }
                    };
                    let transaction_id = match Tables::Transactions.gen_id(&state.db_pool).await {
                        Ok(id) => id,
                        Err(e) => {
                            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                                .message(
                                    "Internal server error while importing transactions"
                                        .to_string(),
                                )
                                .error(ApiError::new(format!(
                                    "Internal server error while generating transaction ID: {}",
                                    e.to_string()
                                )));
                        }
                    };
                    let mut credit_amount = 0.00;
                    let mut debit_amount = 0.00;
                    match validate_amount(&transaction.amount) {
                        Ok(amount) => {
                            if amount < 0.00 {
                                debit_amount = amount.abs();
                            } else if amount > 0.00 {
                                credit_amount = amount.abs()
                            }
                        }
                        Err(e) => {
                            return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                                .message(
                                    "Internal server error while importing transactions"
                                        .to_string(),
                                )
                                .error(ApiError::new(format!(
                                    "Internal server error while parsing amount: {}",
                                    e.to_string()
                                )));
                        }
                    }
                    match sqlx::query!("INSERT INTO transactions (id, user_id, description, date, credit_amount, debit_amount, merchant, category, payment_source) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);", transaction_id, user.claims.user_id, transaction.description, transaction.date, credit_amount, debit_amount, transaction.merchant, transaction.category, transaction.payment_source).execute(&mut *tx).await {
                Ok(_) => continue,
                Err(_) => {
                    failed_transactions += 1;
                    continue;
                }
            }
                }
                match tx.commit().await {
                    Ok(_) => {
                        return ApiResponse::new(StatusCode::OK).message("Transactions imported successfully".to_string()).json(json!({"message": "Transactions imported successfully", "transaction_count": (total_transactions - failed_transactions), "fail_count": failed_transactions}));
                    }
                    Err(e) => {
                        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                            .message("Internal server error while importing transactions".to_string())
                            .error(ApiError::new(format!(
                                "Internal server error while committing transaction to import transactions: {}",
                                e.to_string()
                            )));
                    }
                }
            } else {
                return ApiResponse::new(StatusCode::BAD_REQUEST)
                    .message("No file provided".to_string())
                    .error(ApiError::new("No file provided".to_string()));
            }
        }
    }
}
