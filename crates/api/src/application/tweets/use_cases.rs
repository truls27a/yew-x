use super::ports::TweetRepository;
use crate::domain::error::AppError;
use crate::domain::tweets::entities::Tweet;

pub struct GetTweets<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> GetTweets<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, current_user_id: Option<&str>) -> Result<Vec<Tweet>, AppError> {
        self.repo.find_all(current_user_id).await
    }
}

pub struct GetTweet<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> GetTweet<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Tweet, AppError> {
        self.repo.find_by_id(id, current_user_id).await?.ok_or(
            AppError::NotFound {
                resource_type: "Tweet",
                field: "id",
                value: id.to_string(),
            },
        )
    }
}

pub struct CreateTweet<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> CreateTweet<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        user_id: &str,
        content: &str,
    ) -> Result<Tweet, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        self.repo.create(&id, user_id, content).await?;
        self.repo
            .find_by_id(&id, Some(user_id))
            .await?
            .ok_or(AppError::Internal {
                message: "Failed to fetch created tweet".into(),
                source: None,
            })
    }
}

pub struct GetUserTweets<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> GetUserTweets<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        user_id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Vec<Tweet>, AppError> {
        self.repo.find_by_user_id(user_id, current_user_id).await
    }
}

pub struct ToggleLike<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> ToggleLike<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, tweet_id: &str, user_id: &str) -> Result<(bool, u32), AppError> {
        self.repo.toggle_like(tweet_id, user_id).await
    }
}
