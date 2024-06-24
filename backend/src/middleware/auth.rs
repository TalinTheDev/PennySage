use crate::jwt::verify_access_token;
use crate::jwt::AccessTokenClaims;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{async_trait, extract::FromRequestParts, http::StatusCode};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

#[async_trait]
impl<S> FromRequestParts<S> for AccessTokenClaims
where
  S: Send + Sync,
{
  type Rejection = StatusCode;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_err(|_| StatusCode::UNAUTHORIZED)?;
    // Decode the user data
    let token_data =
      verify_access_token(bearer.token().to_string()).map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(token_data)
  }
}
