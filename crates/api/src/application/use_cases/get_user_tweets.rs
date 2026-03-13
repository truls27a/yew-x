use crate::application::ports::tweet_repository::TweetRepository;
use crate::domain::entities::tweet::Tweet;

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
        current_user_id: &str,
    ) -> anyhow::Result<Vec<Tweet>> {
        self.repo.find_by_user_id(user_id, current_user_id).await
    }
}
