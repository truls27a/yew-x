use super::ports::NotificationRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::domain::notifications::entities::Notification;

#[derive(Clone)]
pub struct GetNotifications;

impl GetNotifications {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        user_id: &str,
    ) -> Result<Vec<Notification>, AppError> {
        uow.notifications().find_by_user_id(user_id).await
    }
}
