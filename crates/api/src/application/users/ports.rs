use crate::domain::users::entities::User;

pub trait UserRepository: Send + Sync {
    fn find_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<User>>> + Send;

    fn create(
        &self,
        id: &str,
        display_name: &str,
        handle: &str,
        avatar_url: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
