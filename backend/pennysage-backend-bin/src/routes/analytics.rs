use axum::{extract::State, routing::get, Router};
use chrono::{DateTime, Local, Months};
use http::StatusCode;
use serde::Serialize;
use serde_json::json;

use lib::types::api_return::{ApiError, ApiResponse};
use lib::types::jwt::AccessToken;
use lib::types::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/analytics", get(analytics))
}

#[derive(Serialize)]
struct AmountPerCategoryCurrentMonth {
    category: String,
    balance: f64,
}

#[derive(Serialize)]
struct TopCategoriesForAmountSpentForCurrentMonth {
    category: String,
    amount: f64,
}

#[derive(Serialize)]
struct LastYearBalanceComparedToThisYearBalance {
    balance: f64,
    year: Option<String>,
}

#[derive(Serialize)]
struct LastMonthsBalanceBySource {
    balance: f64,
    source: String,
}

#[derive(Serialize)]
struct CurrentMonthsBalanceBySource {
    balance: f64,
    source: String,
}

#[derive(Serialize)]
struct AverageSavingsForLast6Months {
    month: Option<String>,
    year: Option<String>,
    savings: f64,
}

#[derive(Serialize)]
struct CurrentMonthsCreditDebitBalance {
    credit: f64,
    debit: f64,
    balance: f64,
}

async fn dashboard(State(state): State<AppState>, user: AccessToken) -> ApiResponse {
    let mut return_data = json!({});
    let now = Local::now();
    let current_month = format!("{}-%", now.format("%Y-%m").to_string());
    let current_year = format!("{}-%", now.format("%Y").to_string());
    let last_month = format!(
        "{}-%",
        now.checked_sub_months(Months::new(1))
            .unwrap_or(DateTime::default())
            .format("%Y-%m")
            .to_string()
    );
    let last_year = format!(
        "{}-%",
        now.checked_sub_months(Months::new(12))
            .unwrap_or(DateTime::default())
            .format("%Y-%m")
            .to_string()
    );

    let amount_per_category_current_month = sqlx::query_as!(AmountPerCategoryCurrentMonth, "SELECT category, SUM(debit_amount) - SUM(credit_amount) AS balance FROM transactions WHERE date LIKE ? AND user_id = ? GROUP BY category;", current_month, user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = amount_per_category_current_month {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("Failed to fetch data (amount per category for current month)".to_string())
            .error(ApiError::new(
                "Failed to fetch data (amount per category for current month)".to_string(),
            ));
    } else if let Ok(d) = amount_per_category_current_month {
        return_data["amount_per_category_current_month"] = json!(d);
    }

    let top_categories_for_amount_spent_for_current_month = sqlx::query_as!(TopCategoriesForAmountSpentForCurrentMonth, "SELECT category, SUM(debit_amount) as amount FROM transactions WHERE date LIKE ? AND user_id = ? GROUP BY category ORDER BY debit_amount DESC LIMIT 5;", current_month, user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = top_categories_for_amount_spent_for_current_month {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message(
                "Failed to fetch data (top 5 categories based on amount spent for current month)"
                    .to_string(),
            )
            .error(ApiError::new(
                "Failed to fetch data (top 5 categories based on amount spent for current month)"
                    .to_string(),
            ));
    } else if let Ok(d) = top_categories_for_amount_spent_for_current_month {
        return_data["top_categories_for_amount_spent_for_current_month"] = json!(d);
    }

    let current_month_balance_compared_to_last_year = sqlx::query_as!(LastYearBalanceComparedToThisYearBalance, "SELECT SUM(debit_amount) - SUM(credit_amount) AS balance, STRFTIME('%Y', date) AS year FROM transactions WHERE user_id = ? AND (date LIKE ? OR date LIKE ?) GROUP BY STRFTIME('%Y', date) ORDER BY STRFTIME('%Y', date);", user.claims.user_id, current_year, last_year).fetch_all(&state.db_pool).await;
    if let Err(_) = current_month_balance_compared_to_last_year {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message(
                "Failed to fetch data (current month's balance as compared to last year's balance)"
                    .to_string(),
            )
            .error(ApiError::new(
                "Failed to fetch data (current month's balance as compared to last year's balance)"
                    .to_string(),
            ));
    } else if let Ok(d) = current_month_balance_compared_to_last_year {
        return_data["current_month_balance_compared_to_last_year"] = json!(d);
    }

    let last_months_balance_by_source = sqlx::query_as!(LastMonthsBalanceBySource, "SELECT SUM(debit_amount) - SUM(credit_amount) AS balance, payment_source AS source FROM transactions WHERE date LIKE ? AND user_id = ? GROUP BY payment_source;", last_month, user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = last_months_balance_by_source {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("Failed to fetch data (last months balance by source)".to_string())
            .error(ApiError::new(
                "Failed to fetch data (last months balance by source)".to_string(),
            ));
    } else if let Ok(d) = last_months_balance_by_source {
        return_data["last_months_balance_by_source"] = json!(d);
    }

    let current_months_balance_by_source = sqlx::query_as!(CurrentMonthsBalanceBySource, "SELECT SUM(debit_amount) - SUM(credit_amount) AS balance, payment_source AS source FROM transactions WHERE date LIKE ? AND user_id = ? GROUP BY payment_source;", current_month, user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = current_months_balance_by_source {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("Failed to fetch data (current months balance by source)".to_string())
            .error(ApiError::new(
                "Failed to fetch data (current months balance by source)".to_string(),
            ));
    } else if let Ok(d) = current_months_balance_by_source {
        return_data["current_months_balance_by_source"] = json!(d);
    }

    let current_months_credit_debit_balance = sqlx::query_as!(CurrentMonthsCreditDebitBalance, "SELECT IFNULL(SUM(credit_amount), 0.00) AS credit, IFNULL(SUM(debit_amount), 0.00) AS debit, IFNULL(SUM(debit_amount) - SUM(credit_amount), 0.00) AS balance FROM transactions WHERE date LIKE ? AND user_id = ?;", current_month, user.claims.user_id).fetch_one(&state.db_pool).await;
    if let Err(_) = current_months_credit_debit_balance {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("Failed to fetch data (current month's credit, debit, balance)".to_string())
            .error(ApiError::new(
                "Failed to fetch data (current month's credit, debit, balance)".to_string(),
            ));
    } else if let Ok(d) = current_months_credit_debit_balance {
        return_data["current_months_credit_debit_balance"] = json!(d);
    }

    let average_savings = sqlx::query_as!(AverageSavingsForLast6Months, "SELECT IFNULL(AVG(credit_amount - debit_amount), 0.00) as savings, STRFTIME('%m', date) AS month, STRFTIME('%Y', date) AS year FROM transactions WHERE user_id = ? GROUP BY STRFTIME('%Y-%m', date) ORDER BY STRFTIME('%Y-%m', date) DESC LIMIT 6;", user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = average_savings {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("Failed to fetch data (average savings for the past 6 months)".to_string())
            .error(ApiError::new(
                "Failed to fetch data (average savings for the past 6 months)".to_string(),
            ));
    } else if let Ok(d) = average_savings {
        return_data["average_savings"] = json!(d);
    }

    ApiResponse::new(StatusCode::OK)
        .message("Dashboard data fetched successfully".to_string())
        .json(json!(return_data))
}

#[derive(Serialize)]
struct MonthlyDebitBalanceForLast6Months {
    debit_balance: f64,
    date: Option<String>,
}

#[derive(Serialize)]
struct NetBalanceForPast5Years {
    balance: f64,
    year: Option<String>,
}

async fn analytics(State(state): State<AppState>, user: AccessToken) -> ApiResponse {
    let mut return_data = json!({});

    let monthly_debit_balance_for_last_6_months = sqlx::query_as!(MonthlyDebitBalanceForLast6Months, "SELECT SUM(debit_amount) as debit_balance, date FROM transactions WHERE user_id = ? GROUP BY STRFTIME('%Y-%m', date) ORDER BY DATE(date) DESC LIMIT 6;", user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = monthly_debit_balance_for_last_6_months {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message(
                "Failed to fetch data (monthly debit balance for the last 6 months)".to_string(),
            )
            .error(ApiError::new(
                "Failed to fetch data (monthly debit balance for the last 6 months)".to_string(),
            ));
    } else if let Ok(d) = monthly_debit_balance_for_last_6_months {
        return_data["monthly_debit_balance_for_last_6_months"] = json!(d);
    }

    let net_balance_for_past_5_years = sqlx::query_as!(NetBalanceForPast5Years, "SELECT SUM(debit_amount) - SUM(credit_amount) as balance, STRFTIME('%Y', date) as year FROM transactions WHERE user_id = ? GROUP BY STRFTIME('%Y', date) ORDER BY DATE(date) DESC LIMIT 5;", user.claims.user_id).fetch_all(&state.db_pool).await;
    if let Err(_) = net_balance_for_past_5_years {
        return ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .message("message".to_string())
            .error(ApiError::new(
                "Failed to fetch data (net balance at the end of the year for the past 5 years)"
                    .to_string(),
            ));
    } else if let Ok(d) = net_balance_for_past_5_years {
        return_data["net_balance_for_past_5_years"] = json!(d);
    }

    ApiResponse::new(StatusCode::OK)
        .message("Analytics data fetched successfully".to_string())
        .json(json!(return_data))
}
