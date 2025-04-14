use crate::types::returns::Result;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// Type definition for JWT access token claims
#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    /// Expiry time
    pub(crate) exp: i64,
    /// Issued at time
    pub(crate) iat: i64,
    /// User id
    pub user_id: String,
    /// Role id
    pub role: i64,
}

pub struct AccessToken {
    pub token: String,
    pub claims: AccessTokenClaims,
}

/// Type definition for JWT refresh token claims
#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    /// Expiry time
    pub(crate) exp: i64,
    /// Issued at time
    pub(crate) iat: i64,
    /// JWT Token Identifier (saved in DB as `id`
    pub(crate) jti: String,
    /// User id
    pub user_id: String,
}
