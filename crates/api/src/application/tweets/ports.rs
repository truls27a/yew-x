use crate::domain::tweets::entities::Tweet;

pub trait TweetRepository: Send + Sync {
    fn find_all(
        &self,
        current_user_id: Option<&str>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Tweet>>> + Send;

    fn find_by_id(
        &self,
        id: &str,
        current_user_id: Option<&str>,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Tweet>>> + Send;

    fn find_by_user_id(
        &self,
        user_id: &str,
        current_user_id: Option<&str>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Tweet>>> + Send;

    fn create(
        &self,
        id: &str,
        user_id: &str,
        content: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn toggle_like(
        &self,
        tweet_id: &str,
        user_id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<(bool, u32)>> + Send;
}
