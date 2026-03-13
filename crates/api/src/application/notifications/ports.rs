use crate::domain::error::AppError;
use crate::domain::notifications::entities::Notification;

pub trait NotificationRepository: Send + Sync {
    fn find_by_user_id(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Notification>, AppError>> + Send;
}
