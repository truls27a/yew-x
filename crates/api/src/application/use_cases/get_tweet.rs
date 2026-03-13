use crate::application::ports::tweet_repository::TweetRepository;
use crate::domain::entities::tweet::Tweet;

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
        current_user_id: &str,
    ) -> anyhow::Result<Option<Tweet>> {
        self.repo.find_by_id(id, current_user_id).await
    }
}
