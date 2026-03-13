use crate::domain::comments::entities::Comment;
use crate::domain::error::AppError;

pub trait CommentRepository: Send + Sync {
    fn find_by_tweet_id(
        &self,
        tweet_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Comment>, AppError>> + Send;

    fn create(
        &self,
        id: &str,
        tweet_id: &str,
        user_id: &str,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn find_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Comment>, AppError>> + Send;
}
