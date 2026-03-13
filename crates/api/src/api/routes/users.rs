use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::ApiError;
use crate::api::middleware::OptionalCaller;
use crate::api::schemas::{TweetResponse, UserResponse};
use crate::infrastructure::shared::unit_of_work::SqliteUnitOfWork;
use crate::AppState;

pub async fn get_single_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let user = state.get_user.execute(uow, &id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user_tweets_handler(
    OptionalCaller(caller): OptionalCaller,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<TweetResponse>>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let user_id = caller.as_ref().map(|c| c.user_id.as_str());
    let tweets = state.get_user_tweets.execute(uow, &id, user_id).await?;
    Ok(Json(tweets.into_iter().map(TweetResponse::from).collect()))
}
