use serde::{Deserialize, Serialize};

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

/// Type definition for JWT access token
#[derive(Serialize, Deserialize)]
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

/// Type definition for JWT refresh token
#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub claims: RefreshTokenClaims,
}
