use crate::domain::entities::notification::Notification;

pub trait NotificationRepository: Send + Sync {
    fn find_by_user_id(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Notification>>> + Send;
}
