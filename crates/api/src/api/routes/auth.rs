use axum::extract::State;
use axum::Json;

use crate::api::errors::ApiError;
use crate::api::middleware::Caller;
use crate::api::schemas::{
    LoginRequest, RefreshRequest, RegisterRequest, TokenPairResponse, UserResponse,
};
use crate::application::auth::use_cases as auth_uc;
use crate::application::users::use_cases as user_uc;
use crate::infrastructure::auth::adapters::{Argon2Hasher, JwtEncoder, Sha256TokenHasher};
use crate::infrastructure::shared::time::UtcClock;
use crate::infrastructure::shared::unit_of_work::SqliteUnitOfWork;
use crate::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<TokenPairResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let hasher = Argon2Hasher;
    let token_hasher = Sha256TokenHasher;
    let encoder = JwtEncoder::new(&state.jwt_secret);
    let token_pair = auth_uc::Register::new(uow, &hasher, &token_hasher, &encoder, UtcClock)
        .execute(&body.email, &body.password, &body.display_name)
        .await?;

    Ok(Json(TokenPairResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenPairResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let hasher = Argon2Hasher;
    let token_hasher = Sha256TokenHasher;
    let encoder = JwtEncoder::new(&state.jwt_secret);
    let token_pair = auth_uc::Login::new(uow, &hasher, &token_hasher, &encoder, UtcClock)
        .execute(&body.email, &body.password)
        .await?;

    Ok(Json(TokenPairResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenPairResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let token_hasher = Sha256TokenHasher;
    let encoder = JwtEncoder::new(&state.jwt_secret);
    let token_pair = auth_uc::Refresh::new(uow, &token_hasher, &encoder, UtcClock)
        .execute(&body.refresh_token)
        .await?;

    Ok(Json(TokenPairResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

pub async fn me(
    caller: Caller,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let user = user_uc::GetUser::new(uow).execute(&caller.user_id).await?;
    Ok(Json(UserResponse::from(user)))
}
