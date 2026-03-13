use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::AppError;
use crate::api::schemas::{CreateTweetRequest, LikeResponse, TweetResponse};
use crate::application::use_cases::{create_tweet, get_tweet, get_tweets, toggle_like};
use crate::AppState;

pub async fn list_tweets(State(state): State<AppState>) -> Result<Json<Vec<TweetResponse>>, AppError> {
    let uc = get_tweets::GetTweets::new(&state.tweet_repo);
    let tweets = uc.execute("current").await?;
    Ok(Json(tweets.into_iter().map(TweetResponse::from).collect()))
}

pub async fn get_single_tweet(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TweetResponse>, AppError> {
    let uc = get_tweet::GetTweet::new(&state.tweet_repo);
    let tweet = uc
        .execute(&id, "current")
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".into()))?;
    Ok(Json(TweetResponse::from(tweet)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateTweetRequest>,
) -> Result<Json<TweetResponse>, AppError> {
    let uc = create_tweet::CreateTweet::new(&state.tweet_repo);
    let tweet = uc.execute(&body.user_id, &body.content).await?;
    Ok(Json(TweetResponse::from(tweet)))
}

pub async fn like(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LikeResponse>, AppError> {
    let uc = toggle_like::ToggleLike::new(&state.tweet_repo);
    let (liked, count) = uc.execute(&id, "current").await?;
    Ok(Json(LikeResponse { liked, count }))
}
