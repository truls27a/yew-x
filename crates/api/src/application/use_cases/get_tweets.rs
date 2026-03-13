use crate::application::ports::tweet_repository::TweetRepository;
use crate::domain::entities::tweet::Tweet;

pub struct GetTweets<'a, T: TweetRepository> {
    repo: &'a T,
}

impl<'a, T: TweetRepository> GetTweets<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, current_user_id: &str) -> anyhow::Result<Vec<Tweet>> {
        self.repo.find_all(current_user_id).await
    }
}
