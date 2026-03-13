use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::AppError;
use crate::api::middleware::OptionalCaller;
use crate::api::schemas::{TweetResponse, UserResponse};
use crate::application::users::use_cases;
use crate::application::tweets::use_cases as tweet_use_cases;
use crate::AppState;

pub async fn get_single_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, AppError> {
    let uc = use_cases::GetUser::new(&state.user_repo);
    let user = uc
        .execute(&id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user_tweets_handler(
    OptionalCaller(caller): OptionalCaller,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<TweetResponse>>, AppError> {
    let uc = tweet_use_cases::GetUserTweets::new(&state.tweet_repo);
    let user_id = caller.as_ref().map(|c| c.user_id.as_str());
    let tweets = uc.execute(&id, user_id).await?;
    Ok(Json(tweets.into_iter().map(TweetResponse::from).collect()))
}
