use crate::application::ports::tweet_repository::TweetRepository;

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
