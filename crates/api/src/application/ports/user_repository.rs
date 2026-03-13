use crate::domain::entities::user::User;

pub trait UserRepository: Send + Sync {
    fn find_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<User>>> + Send;
}
