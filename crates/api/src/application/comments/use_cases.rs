use super::ports::CommentRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::comments::entities::Comment;
use crate::domain::error::AppError;

#[derive(Clone)]
pub struct GetCommentsUseCase;

impl GetCommentsUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        tweet_id: &str,
    ) -> Result<Vec<Comment>, AppError> {
        uow.comments().find_by_tweet_id(tweet_id).await
    }
}

#[derive(Clone)]
pub struct CreateCommentUseCase;

impl CreateCommentUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        tweet_id: &str,
        user_id: &str,
        content: &str,
    ) -> Result<Comment, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        uow.comments().create(&id, tweet_id, user_id, content).await?;
        let comment = uow
            .comments()
            .find_by_id(&id)
            .await?
            .ok_or(AppError::Internal {
                message: "Failed to fetch created comment".into(),
                source: None,
            })?;
        uow.commit().await?;
        Ok(comment)
    }
}
