use serde::{Deserialize, Serialize};

/// Type definition for JWT access token claims
#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    /// Expiry time
    exp: i64,
    /// Issued at time
    iat: i64,
    /// User id
    pub id: String,
    /// Role id
    pub role: i64,
}

/// Type definition for JWT refresh token claims
#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    /// Expiry time
    exp: i64,
    /// Issued at time
    iat: i64,
    /// JWT Token Identifier (saved in DB as `id`
    jti: String,
    /// User id
    pub id: String,
}
