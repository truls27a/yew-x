use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::AppError;
use crate::api::schemas::{TweetResponse, UserResponse};
use crate::application::use_cases::{get_user, get_user_tweets};
use crate::AppState;

pub async fn get_single_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, AppError> {
    let uc = get_user::GetUser::new(&state.user_repo);
    let user = uc
        .execute(&id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user_tweets_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<TweetResponse>>, AppError> {
    let uc = get_user_tweets::GetUserTweets::new(&state.tweet_repo);
    let tweets = uc.execute(&id, "current").await?;
    Ok(Json(tweets.into_iter().map(TweetResponse::from).collect()))
}
