use serde::{Deserialize, Serialize};

use crate::domain::auth::entities::{Identity, Session};
use crate::domain::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub trait PasswordHashPort: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, AppError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}

pub trait TokenHashPort: Send + Sync {
    fn hash(&self, token: &str) -> String;
}

pub trait TokenPort: Send + Sync {
    fn encode(&self, sub: &str, iat: usize, exp: usize) -> Result<String, AppError>;
    fn decode(&self, token: &str) -> Result<TokenPayload, AppError>;
}

pub trait AuthRepository: Send + Sync {
    fn find_identity_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<Option<Identity>, AppError>> + Send;

    fn create_identity(
        &self,
        id: &str,
        user_id: &str,
        email: &str,
        password_hash: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn create_session(
        &self,
        id: &str,
        identity_id: &str,
        token_hash: &str,
        expires_at: i64,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn find_session_by_token_hash(
        &self,
        token_hash: &str,
    ) -> impl std::future::Future<Output = Result<Option<Session>, AppError>> + Send;

    fn delete_session(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn find_identity_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Identity>, AppError>> + Send;
}
