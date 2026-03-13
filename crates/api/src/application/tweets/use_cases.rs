use super::ports::TweetRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::domain::tweets::entities::Tweet;

pub struct GetTweets<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> GetTweets<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(self, current_user_id: Option<&str>) -> Result<Vec<Tweet>, AppError> {
        self.uow.tweets().find_all(current_user_id).await
    }
}

pub struct GetTweet<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> GetTweet<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(
        self,
        id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Tweet, AppError> {
        self.uow.tweets().find_by_id(id, current_user_id).await?.ok_or(
            AppError::NotFound {
                resource_type: "Tweet",
                field: "id",
                value: id.to_string(),
            },
        )
    }
}

pub struct CreateTweet<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> CreateTweet<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(
        self,
        user_id: &str,
        content: &str,
    ) -> Result<Tweet, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        self.uow.tweets().create(&id, user_id, content).await?;
        let tweet = self.uow
            .tweets()
            .find_by_id(&id, Some(user_id))
            .await?
            .ok_or(AppError::Internal {
                message: "Failed to fetch created tweet".into(),
                source: None,
            })?;
        self.uow.commit().await?;
        Ok(tweet)
    }
}

pub struct GetUserTweets<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> GetUserTweets<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(
        self,
        user_id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Vec<Tweet>, AppError> {
        self.uow.tweets().find_by_user_id(user_id, current_user_id).await
    }
}

pub struct ToggleLike<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> ToggleLike<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(self, tweet_id: &str, user_id: &str) -> Result<(bool, u32), AppError> {
        let result = self.uow.tweets().toggle_like(tweet_id, user_id).await?;
        self.uow.commit().await?;
        Ok(result)
    }
}
