use crate::application::ports::notification_repository::NotificationRepository;
use crate::domain::entities::notification::Notification;

pub struct GetNotifications<'a, T: NotificationRepository> {
    repo: &'a T,
}

impl<'a, T: NotificationRepository> GetNotifications<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: &str) -> anyhow::Result<Vec<Notification>> {
        self.repo.find_by_user_id(user_id).await
    }
}
