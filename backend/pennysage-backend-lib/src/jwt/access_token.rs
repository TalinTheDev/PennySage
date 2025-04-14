use super::types::{AccessToken, AccessTokenClaims};
use crate::types::returns::Result;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

impl AccessToken {
    /// Returns a new `AccessToken` instance with the given user id and role
    ///
    /// May fail when attempting to parse timestamps
    pub fn new(user_id: String, role: i64) -> Result<Self> {
        let now = Local::now();
        let expiration = now
            .checked_add_signed(Duration::minutes(5))
            .get_or_insert(now)
            .format("%s")
            .to_string();
        let issued_at = now.format("%s").to_string();

        let mut token = AccessToken {
            token: "".to_string(),
            claims: AccessTokenClaims {
                exp: expiration.parse::<i64>()?,
                iat: issued_at.parse::<i64>()?,
                user_id,
                role,
            },
        };
        token.encode()?;
        Ok(token)
    }

    /// Returns a new `AccessToken` instance from the given token string
    pub fn from_token(&self, token: String) -> Result<Self> {
        let token = AccessToken {
            claims: AccessToken::extract_claims(token.as_str())?,
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
            &EncodingKey::from_secret(env::var("JWT_SECRET")?.as_bytes()),
        )?;
        self.token = token;
        Ok(true)
    }

    /// Extracts claims from the given token string into AccessTokenClaims
    fn extract_claims(token: &str) -> Result<AccessTokenClaims> {
        Ok(decode(
            token,
            &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_bytes()),
            &Validation::default(),
        )?
        .claims)
    }
}
