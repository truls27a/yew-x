use super::ports::NotificationRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::domain::notifications::entities::Notification;

pub struct GetNotifications<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> GetNotifications<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(self, user_id: &str) -> Result<Vec<Notification>, AppError> {
        self.uow.notifications().find_by_user_id(user_id).await
    }
}
