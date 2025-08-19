use crate::db::tables::Tables;
use crate::types::db::UserRow;
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
            Tables::Options => query.push_str(" options WHERE id = ?"),
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

    pub async fn get_user_details(&self, id: String, conn: &Pool<Sqlite>) -> Result<UserRow> {
        let result = sqlx::query!(
            "SELECT id, username, email, password, first_name, last_name, role FROM users WHERE id = ?",
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(UserRow {
            id: result.id,
            username: result.username,
            email: result.email,
            password: result.password,
            first_name: result.first_name,
            last_name: result.last_name,
            role: result.role,
        })
    }
}
