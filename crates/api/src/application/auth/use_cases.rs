use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::ports::{AuthRepository, HashPort, TokenPort};
use crate::application::shared::time::Clock;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::application::users::ports::UserRepository;
use crate::domain::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

fn hash_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub struct Register<'a, U: UnitOfWork> {
    uow: U,
    hasher: &'a dyn HashPort,
    token_port: &'a dyn TokenPort,
    clock: Box<dyn Clock>,
}

impl<'a, U: UnitOfWork> Register<'a, U> {
    pub fn new(
        uow: U,
        hasher: &'a dyn HashPort,
        token_port: &'a dyn TokenPort,
        clock: impl Clock + 'static,
    ) -> Self {
        Self {
            uow,
            hasher,
            token_port,
            clock: Box::new(clock),
        }
    }

    pub async fn execute(
        self,
        email: &str,
        password: &str,
        display_name: &str,
    ) -> Result<TokenPair, AppError> {
        // Check uniqueness
        if self.uow.auth().find_identity_by_email(email).await?.is_some() {
            return Err(AppError::Conflict {
                resource_type: "Identity",
                reason: "Email already registered",
            });
        }

        // Hash password
        let password_hash = self.hasher.hash(password)?;

        // Create user
        let user_id = uuid::Uuid::new_v4().to_string();
        let handle = email.split('@').next().unwrap_or("user");
        let avatar_url = format!("https://i.pravatar.cc/150?u={}", handle);
        self.uow
            .users()
            .create(&user_id, display_name, handle, &avatar_url)
            .await?;

        // Create identity
        let identity_id = uuid::Uuid::new_v4().to_string();
        self.uow
            .auth()
            .create_identity(&identity_id, &user_id, email, &password_hash)
            .await?;

        // Create session
        let now = self.clock.now() as usize;
        let access_token = self.token_port.encode(&user_id, now, now + 15 * 60)?;
        let refresh_token = uuid::Uuid::new_v4().to_string();
        let refresh_hash = hash_refresh_token(&refresh_token);
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + 7 * 24 * 3600;
        self.uow
            .auth()
            .create_session(&session_id, &identity_id, &refresh_hash, expires_at)
            .await?;

        self.uow.commit().await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

pub struct Login<'a, U: UnitOfWork> {
    uow: U,
    hasher: &'a dyn HashPort,
    token_port: &'a dyn TokenPort,
    clock: Box<dyn Clock>,
}

impl<'a, U: UnitOfWork> Login<'a, U> {
    pub fn new(
        uow: U,
        hasher: &'a dyn HashPort,
        token_port: &'a dyn TokenPort,
        clock: impl Clock + 'static,
    ) -> Self {
        Self {
            uow,
            hasher,
            token_port,
            clock: Box::new(clock),
        }
    }

    pub async fn execute(self, email: &str, password: &str) -> Result<TokenPair, AppError> {
        let identity = self
            .uow
            .auth()
            .find_identity_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized {
                reason: "Invalid credentials",
            })?;

        // Verify password
        let valid = self.hasher.verify(password, &identity.password_hash)?;
        if !valid {
            return Err(AppError::Unauthorized {
                reason: "Invalid credentials",
            });
        }

        // Create session
        let now = self.clock.now() as usize;
        let access_token = self.token_port.encode(&identity.user_id, now, now + 15 * 60)?;
        let refresh_token = uuid::Uuid::new_v4().to_string();
        let refresh_hash = hash_refresh_token(&refresh_token);
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + 7 * 24 * 3600;
        self.uow
            .auth()
            .create_session(&session_id, &identity.id, &refresh_hash, expires_at)
            .await?;

        self.uow.commit().await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

pub struct Refresh<'a, U: UnitOfWork> {
    uow: U,
    token_port: &'a dyn TokenPort,
    clock: Box<dyn Clock>,
}

impl<'a, U: UnitOfWork> Refresh<'a, U> {
    pub fn new(uow: U, token_port: &'a dyn TokenPort, clock: impl Clock + 'static) -> Self {
        Self {
            uow,
            token_port,
            clock: Box::new(clock),
        }
    }

    pub async fn execute(self, refresh_token: &str) -> Result<TokenPair, AppError> {
        let token_hash = hash_refresh_token(refresh_token);
        let session = self
            .uow
            .auth()
            .find_session_by_token_hash(&token_hash)
            .await?
            .ok_or(AppError::Unauthorized {
                reason: "Invalid refresh token",
            })?;

        // Check expiry
        if self.clock.now() > session.expires_at {
            self.uow.auth().delete_session(&session.id).await?;
            self.uow.commit().await?;
            return Err(AppError::Unauthorized {
                reason: "Refresh token expired",
            });
        }

        // Delete old session
        self.uow.auth().delete_session(&session.id).await?;

        let identity = self
            .uow
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
        let refresh_hash = hash_refresh_token(&new_refresh_token);
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + 7 * 24 * 3600;
        self.uow
            .auth()
            .create_session(&session_id, &identity.id, &refresh_hash, expires_at)
            .await?;

        self.uow.commit().await?;

        Ok(TokenPair {
            access_token,
            refresh_token: new_refresh_token,
        })
    }
}
