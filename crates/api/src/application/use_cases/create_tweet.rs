use crate::application::ports::tweet_repository::TweetRepository;
use crate::domain::entities::tweet::Tweet;

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
        // Re-fetch the created tweet to get the full entity with user data
        self.repo
            .find_by_id(&id, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to fetch created tweet"))
    }
}
