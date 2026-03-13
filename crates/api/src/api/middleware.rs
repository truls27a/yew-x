use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::application::auth::use_cases::decode_access_token;
use crate::api::errors::ApiError;
use crate::AppState;

pub struct Caller {
    pub user_id: String,
    pub identity_id: String,
}

impl FromRequestParts<AppState> for Caller {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| ApiError::Unauthorized {
                message: "Missing authorization header".into(),
            })?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::Unauthorized {
                message: "Invalid authorization header".into(),
            })?;

        let claims = decode_access_token(token, &state.jwt_secret)
            .map_err(|_| ApiError::Unauthorized {
                message: "Invalid or expired token".into(),
            })?;

        Ok(Caller {
            user_id: claims.sub,
            identity_id: claims.identity_id,
        })
    }
}

pub struct OptionalCaller(pub Option<Caller>);

impl FromRequestParts<AppState> for OptionalCaller {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match Caller::from_request_parts(parts, state).await {
            Ok(caller) => Ok(OptionalCaller(Some(caller))),
            Err(_) => Ok(OptionalCaller(None)),
        }
    }
}
