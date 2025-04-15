use crate::db::tables::Tables;
use crate::jwt::types::{RefreshToken, RefreshTokenClaims};
use crate::types::returns::Result;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{Pool, Sqlite};
use std::env;

impl RefreshToken {
    /// Returns a new `RefreshToken` instance with the given user id
    ///
    /// Database connection pool must be passed in in order to generate token ID and store it in the database
    ///
    /// May fail when attempting to parse timestamps
    pub async fn new(user_id: String, conn: Pool<Sqlite>) -> Result<Self> {
        let now = Local::now();
        let expiration = now
            .checked_add_signed(Duration::days(7))
            .get_or_insert(now)
            .format("%s")
            .to_string();
        let issued_at = now.format("%s").to_string();

        let mut token = RefreshToken {
            token: "".to_string(),
            claims: RefreshTokenClaims {
                exp: expiration.parse::<i64>()?,
                iat: issued_at.parse::<i64>()?,
                user_id,
                jti: Tables::RefreshTokens.gen_id(&conn).await?,
            },
        };
        token.encode()?;

        token.invalidate_refresh_tokens(&conn).await?;
        token.insert_refresh_token(&conn).await?;
        Ok(token)
    }

    /// Returns a new `RefreshToken` instance from the given token string
    pub fn from_token(&self, token: String) -> Result<Self> {
        let token = RefreshToken {
            claims: RefreshToken::extract_claims(token.as_str())?,
            token,
        };
        Ok(token)
    }

    /// Encodes JWT token based on claims
    ///
    /// May fail when attempting to encode the token or reading the JWT secret from the environment
    pub fn encode(&mut self) -> Result<bool> {
        let token = encode(
            &Header::default(),
            &self.claims,
            &EncodingKey::from_secret(env::var("JWT_REFRESH_SECRET")?.as_bytes()),
        )?;
        self.token = token;
        Ok(true)
    }

    /// Extracts claims from the given token string into RefreshTokenClaims
    fn extract_claims(token: &str) -> Result<RefreshTokenClaims> {
        Ok(decode(
            token,
            &DecodingKey::from_secret(env::var("JWT_REFRESH_SECRET")?.as_bytes()),
            &Validation::default(),
        )?
        .claims)
    }

    /// Verifies refresh token by verifying token details against database
    ///
    /// Returns true if the token is valid, false if a error occurs
    /// (e.g. token not found in database)
    pub async fn verify(&self, conn: &Pool<Sqlite>) -> Result<bool> {
        sqlx::query!(
            "SELECT user_id FROM refresh_tokens WHERE id = ? AND revoked = 0",
            self.claims.jti
        )
        .fetch_one(conn)
        .await?;

        Ok(true)
    }

    /// Invalidates all refresh tokens for the current user
    ///
    /// Gets user id by reading token claims
    pub async fn invalidate_refresh_tokens(&self, conn: &Pool<Sqlite>) -> Result<bool> {
        let updated_at = Local::now().format("%s").to_string();
        sqlx::query!(
            "UPDATE refresh_tokens SET revoked = 1, updated_at = ? WHERE user_id = ?",
            self.claims.user_id,
            updated_at
        )
        .execute(conn)
        .await?;

        Ok(true)
    }

    /// Inserts current token into the database
    pub async fn insert_refresh_token(&self, conn: &Pool<Sqlite>) -> Result<bool> {
        sqlx::query!("INSERT INTO refresh_tokens (id, user_id, revoked, created_at, updated_at) VALUES (?, ?, ?, ?, ?);", self.claims.jti, self.claims.user_id, 0, self.claims.iat, self.claims.iat)
            .execute(conn)
            .await?;

        Ok(true)
    }
}
