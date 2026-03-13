use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::ApiError;
use crate::api::middleware::{Caller, OptionalCaller};
use crate::api::schemas::{CreateTweetRequest, LikeResponse, TweetResponse};
use crate::infrastructure::shared::unit_of_work::SqliteUnitOfWork;
use crate::AppState;

pub async fn list_tweets(
    OptionalCaller(caller): OptionalCaller,
    State(state): State<AppState>,
) -> Result<Json<Vec<TweetResponse>>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let user_id = caller.as_ref().map(|c| c.user_id.as_str());
    let tweets = state.get_tweets_use_case.execute(uow, user_id).await?;
    Ok(Json(tweets.into_iter().map(TweetResponse::from).collect()))
}

pub async fn get_single_tweet(
    OptionalCaller(caller): OptionalCaller,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TweetResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let user_id = caller.as_ref().map(|c| c.user_id.as_str());
    let tweet = state.get_tweet_use_case.execute(uow, &id, user_id).await?;
    Ok(Json(TweetResponse::from(tweet)))
}

pub async fn create(
    caller: Caller,
    State(state): State<AppState>,
    Json(body): Json<CreateTweetRequest>,
) -> Result<Json<TweetResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let tweet = state.create_tweet_use_case.execute(uow, &caller.user_id, &body.content).await?;
    Ok(Json(TweetResponse::from(tweet)))
}

pub async fn like(
    caller: Caller,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LikeResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let (liked, count) = state.toggle_like_use_case.execute(uow, &id, &caller.user_id).await?;
    Ok(Json(LikeResponse { liked, count }))
}
