use axum::extract::State;
use axum::Json;

use crate::api::errors::AppError;
use crate::api::middleware::Caller;
use crate::api::schemas::{
    LoginRequest, RefreshRequest, RegisterRequest, TokenPairResponse, UserResponse,
};
use crate::application::auth::use_cases as auth_uc;
use crate::application::users::use_cases as user_uc;
use crate::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<TokenPairResponse>, AppError> {
    let token_pair = auth_uc::register(
        &state.auth_repo,
        &state.user_repo,
        &body.email,
        &body.password,
        &body.display_name,
        &state.jwt_secret,
    )
    .await
    .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    Ok(Json(TokenPairResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenPairResponse>, AppError> {
    let token_pair = auth_uc::login(
        &state.auth_repo,
        &body.email,
        &body.password,
        &state.jwt_secret,
    )
    .await
    .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    Ok(Json(TokenPairResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenPairResponse>, AppError> {
    let token_pair = auth_uc::refresh(
        &state.auth_repo,
        &body.refresh_token,
        &state.jwt_secret,
    )
    .await
    .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    Ok(Json(TokenPairResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

pub async fn me(
    caller: Caller,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, AppError> {
    let uc = user_uc::GetUser::new(&state.user_repo);
    let user = uc
        .execute(&caller.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    Ok(Json(UserResponse::from(user)))
}
