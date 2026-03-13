use crate::domain::auth::entities::{Identity, Session};

pub trait AuthRepository: Send + Sync {
    fn find_identity_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Identity>>> + Send;

    fn create_identity(
        &self,
        id: &str,
        user_id: &str,
        email: &str,
        password_hash: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn create_session(
        &self,
        id: &str,
        identity_id: &str,
        token_hash: &str,
        expires_at: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_session_by_token_hash(
        &self,
        token_hash: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Session>>> + Send;

    fn delete_session(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_identity_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Identity>>> + Send;
}
