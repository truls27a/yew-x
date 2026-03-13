use super::ports::TweetRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::domain::tweets::entities::Tweet;

#[derive(Clone)]
pub struct GetTweetsUseCase;

impl GetTweetsUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        current_user_id: Option<&str>,
    ) -> Result<Vec<Tweet>, AppError> {
        uow.tweets().find_all(current_user_id).await
    }
}

#[derive(Clone)]
pub struct GetTweetUseCase;

impl GetTweetUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Tweet, AppError> {
        uow.tweets()
            .find_by_id(id, current_user_id)
            .await?
            .ok_or(AppError::NotFound {
                resource_type: "Tweet",
                field: "id",
                value: id.to_string(),
            })
    }
}

#[derive(Clone)]
pub struct CreateTweetUseCase;

impl CreateTweetUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        user_id: &str,
        content: &str,
    ) -> Result<Tweet, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        uow.tweets().create(&id, user_id, content).await?;
        let tweet = uow
            .tweets()
            .find_by_id(&id, Some(user_id))
            .await?
            .ok_or(AppError::Internal {
                message: "Failed to fetch created tweet".into(),
                source: None,
            })?;
        uow.commit().await?;
        Ok(tweet)
    }
}

#[derive(Clone)]
pub struct GetUserTweetsUseCase;

impl GetUserTweetsUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        user_id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Vec<Tweet>, AppError> {
        uow.tweets()
            .find_by_user_id(user_id, current_user_id)
            .await
    }
}

#[derive(Clone)]
pub struct ToggleLikeUseCase;

impl ToggleLikeUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        tweet_id: &str,
        user_id: &str,
    ) -> Result<(bool, u32), AppError> {
        let result = uow.tweets().toggle_like(tweet_id, user_id).await?;
        uow.commit().await?;
        Ok(result)
    }
}
