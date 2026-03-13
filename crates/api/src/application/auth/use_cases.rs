use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::ports::{PasswordHashPort, TokenHashPort, TokenPort};
use crate::application::shared::time::Clock;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::application::users::ports::UserRepository;
use crate::domain::error::AppError;

use super::ports::AuthRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct RegisterUseCase {
    password_hasher: Arc<dyn PasswordHashPort>,
    token_hasher: Arc<dyn TokenHashPort>,
    token_port: Arc<dyn TokenPort>,
    clock: Arc<dyn Clock>,
}

impl RegisterUseCase {
    pub fn new(
        password_hasher: Arc<dyn PasswordHashPort>,
        token_hasher: Arc<dyn TokenHashPort>,
        token_port: Arc<dyn TokenPort>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            password_hasher,
            token_hasher,
            token_port,
            clock,
        }
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        email: &str,
        password: &str,
        display_name: &str,
    ) -> Result<TokenPair, AppError> {
        // Check uniqueness
        if uow.auth().find_identity_by_email(email).await?.is_some() {
            return Err(AppError::Conflict {
                resource_type: "Identity",
                reason: "Email already registered",
            });
        }

        // Hash password
        let password_hash = self.password_hasher.hash(password)?;

        // Create user
        let user_id = uuid::Uuid::new_v4().to_string();
        let handle = email.split('@').next().unwrap_or("user");
        let avatar_url = format!("https://i.pravatar.cc/150?u={}", handle);
        uow.users()
            .create(&user_id, display_name, handle, &avatar_url)
            .await?;

        // Create identity
        let identity_id = uuid::Uuid::new_v4().to_string();
        uow.auth()
            .create_identity(&identity_id, &user_id, email, &password_hash)
            .await?;

        // Create session
        let now = self.clock.now() as usize;
        let access_token = self.token_port.encode(&user_id, now, now + 15 * 60)?;
        let refresh_token = uuid::Uuid::new_v4().to_string();
        let refresh_hash = self.token_hasher.hash(&refresh_token);
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + 7 * 24 * 3600;
        uow.auth()
            .create_session(&session_id, &identity_id, &refresh_hash, expires_at)
            .await?;

        uow.commit().await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

#[derive(Clone)]
pub struct LoginUseCase {
    password_hasher: Arc<dyn PasswordHashPort>,
    token_hasher: Arc<dyn TokenHashPort>,
    token_port: Arc<dyn TokenPort>,
    clock: Arc<dyn Clock>,
}

impl LoginUseCase {
    pub fn new(
        password_hasher: Arc<dyn PasswordHashPort>,
        token_hasher: Arc<dyn TokenHashPort>,
        token_port: Arc<dyn TokenPort>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            password_hasher,
            token_hasher,
            token_port,
            clock,
        }
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        email: &str,
        password: &str,
    ) -> Result<TokenPair, AppError> {
        let identity = uow
            .auth()
            .find_identity_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized {
                reason: "Invalid credentials",
            })?;

        // Verify password
        let valid = self.password_hasher.verify(password, &identity.password_hash)?;
        if !valid {
            return Err(AppError::Unauthorized {
                reason: "Invalid credentials",
            });
        }

        // Create session
        let now = self.clock.now() as usize;
        let access_token = self.token_port.encode(&identity.user_id, now, now + 15 * 60)?;
        let refresh_token = uuid::Uuid::new_v4().to_string();
        let refresh_hash = self.token_hasher.hash(&refresh_token);
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + 7 * 24 * 3600;
        uow.auth()
            .create_session(&session_id, &identity.id, &refresh_hash, expires_at)
            .await?;

        uow.commit().await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

#[derive(Clone)]
pub struct RefreshUseCase {
    token_hasher: Arc<dyn TokenHashPort>,
    token_port: Arc<dyn TokenPort>,
    clock: Arc<dyn Clock>,
}

impl RefreshUseCase {
    pub fn new(
        token_hasher: Arc<dyn TokenHashPort>,
        token_port: Arc<dyn TokenPort>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            token_hasher,
            token_port,
            clock,
        }
    }

    pub async fn execute<U: UnitOfWork>(
        &self,
        uow: U,
        refresh_token: &str,
    ) -> Result<TokenPair, AppError> {
        let token_hash = self.token_hasher.hash(refresh_token);
        let session = uow
            .auth()
            .find_session_by_token_hash(&token_hash)
            .await?
            .ok_or(AppError::Unauthorized {
                reason: "Invalid refresh token",
            })?;

        // Check expiry
        if self.clock.now() > session.expires_at {
            uow.auth().delete_session(&session.id).await?;
            uow.commit().await?;
            return Err(AppError::Unauthorized {
                reason: "Refresh token expired",
            });
        }

        // Delete old session
        uow.auth().delete_session(&session.id).await?;

        let identity = uow
            .auth()
            .find_identity_by_id(&session.identity_id)
            .await?
            .ok_or(AppError::NotFound {
                resource_type: "Identity",
                field: "id",
                value: session.identity_id.clone(),
            })?;

        let now = self.clock.now() as usize;
        let access_token = self.token_port.encode(&identity.user_id, now, now + 15 * 60)?;
        let new_refresh_token = uuid::Uuid::new_v4().to_string();
        let refresh_hash = self.token_hasher.hash(&new_refresh_token);
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + 7 * 24 * 3600;
        uow.auth()
            .create_session(&session_id, &identity.id, &refresh_hash, expires_at)
            .await?;

        uow.commit().await?;

        Ok(TokenPair {
            access_token,
            refresh_token: new_refresh_token,
        })
    }
}
