use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::ApiError;
use crate::api::middleware::{Caller, OptionalCaller};
use crate::api::schemas::{CreateTweetRequest, LikeResponse, TweetResponse};
use crate::application::tweets::use_cases;
use crate::AppState;

pub async fn list_tweets(
    OptionalCaller(caller): OptionalCaller,
    State(state): State<AppState>,
) -> Result<Json<Vec<TweetResponse>>, ApiError> {
    let uc = use_cases::GetTweets::new(&state.tweet_repo);
    let user_id = caller.as_ref().map(|c| c.user_id.as_str());
    let tweets = uc.execute(user_id).await?;
    Ok(Json(tweets.into_iter().map(TweetResponse::from).collect()))
}

pub async fn get_single_tweet(
    OptionalCaller(caller): OptionalCaller,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TweetResponse>, ApiError> {
    let uc = use_cases::GetTweet::new(&state.tweet_repo);
    let user_id = caller.as_ref().map(|c| c.user_id.as_str());
    let tweet = uc.execute(&id, user_id).await?;
    Ok(Json(TweetResponse::from(tweet)))
}

pub async fn create(
    caller: Caller,
    State(state): State<AppState>,
    Json(body): Json<CreateTweetRequest>,
) -> Result<Json<TweetResponse>, ApiError> {
    let uc = use_cases::CreateTweet::new(&state.tweet_repo);
    let tweet = uc.execute(&caller.user_id, &body.content).await?;
    Ok(Json(TweetResponse::from(tweet)))
}

pub async fn like(
    caller: Caller,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LikeResponse>, ApiError> {
    let uc = use_cases::ToggleLike::new(&state.tweet_repo);
    let (liked, count) = uc.execute(&id, &caller.user_id).await?;
    Ok(Json(LikeResponse { liked, count }))
}
