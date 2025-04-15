use crate::db::tables::Tables;
use crate::types::lib_return::{Error, Result};
use nanoid::nanoid;
use sqlx::{Pool, Sqlite};

impl Tables {
    /// Generates a unique ID for the given table
    pub async fn gen_id(&self, conn: &Pool<Sqlite>) -> Result<String> {
        let mut query = "SELECT * FROM".to_string();
        match self {
            Tables::Users => query.push_str(" users WHERE id = ?"),
            Tables::Families => query.push_str(" families WHERE id = ?"),
            Tables::Transactions => query.push_str(" transactions WHERE id = ?"),
            Tables::TransactionItems => query.push_str(" transaction_items WHERE id = ?"),
            Tables::RefreshTokens => query.push_str(" refresh_tokens WHERE id = ?"),
        }

        let mut id;
        loop {
            id = nanoid!();
            let statement = sqlx::query(&query).bind(&id);
            match statement.fetch_one(conn).await {
                Ok(_) => continue,
                Err(sqlx::Error::RowNotFound) => return Ok(id),
                Err(e) => return Err(Error::DatabaseError(e)),
            }
        }
    }
}
