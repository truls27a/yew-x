use crate::domain::error::AppError;
use crate::domain::tweets::entities::Tweet;

pub trait TweetRepository: Send + Sync {
    fn find_all(
        &self,
        current_user_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<Tweet>, AppError>> + Send;

    fn find_by_id(
        &self,
        id: &str,
        current_user_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Option<Tweet>, AppError>> + Send;

    fn find_by_user_id(
        &self,
        user_id: &str,
        current_user_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<Tweet>, AppError>> + Send;

    fn create(
        &self,
        id: &str,
        user_id: &str,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn toggle_like(
        &self,
        tweet_id: &str,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<(bool, u32), AppError>> + Send;
}
