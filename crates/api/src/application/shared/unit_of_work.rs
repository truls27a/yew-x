use std::future::Future;

use crate::application::auth::ports::AuthRepository;
use crate::application::notifications::ports::NotificationRepository;
use crate::application::tweets::ports::TweetRepository;
use crate::application::users::ports::UserRepository;
use crate::domain::error::AppError;

pub trait UnitOfWork: Send + Sync {
    type UserRepo: UserRepository;
    type TweetRepo: TweetRepository;
    type NotificationRepo: NotificationRepository;
    type AuthRepo: AuthRepository;

    fn users(&self) -> Self::UserRepo;
    fn tweets(&self) -> Self::TweetRepo;
    fn notifications(&self) -> Self::NotificationRepo;
    fn auth(&self) -> Self::AuthRepo;

    fn commit(self) -> impl Future<Output = Result<(), AppError>> + Send;
    fn rollback(self) -> impl Future<Output = Result<(), AppError>> + Send;
}
