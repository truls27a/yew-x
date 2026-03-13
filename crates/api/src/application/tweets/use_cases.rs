use super::ports::TweetRepository;
use crate::domain::tweets::entities::Tweet;

pub struct GetTweets<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> GetTweets<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, current_user_id: Option<&str>) -> anyhow::Result<Vec<Tweet>> {
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
    ) -> anyhow::Result<Option<Tweet>> {
        self.repo.find_by_id(id, current_user_id).await
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
    ) -> anyhow::Result<Tweet> {
        let id = uuid::Uuid::new_v4().to_string();
        self.repo.create(&id, user_id, content).await?;
        self.repo
            .find_by_id(&id, Some(user_id))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to fetch created tweet"))
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
    ) -> anyhow::Result<Vec<Tweet>> {
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

    pub async fn execute(&self, tweet_id: &str, user_id: &str) -> anyhow::Result<(bool, u32)> {
        self.repo.toggle_like(tweet_id, user_id).await
    }
}
