use super::ports::NotificationRepository;
use crate::domain::error::AppError;
use crate::domain::notifications::entities::Notification;

pub struct GetNotifications<'a, T: NotificationRepository> {
    repo: &'a T,
}

impl<'a, T: NotificationRepository> GetNotifications<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: &str) -> Result<Vec<Notification>, AppError> {
        self.repo.find_by_user_id(user_id).await
    }
}
