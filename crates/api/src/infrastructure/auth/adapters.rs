use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use sha2::{Digest, Sha256};

use crate::application::auth::ports::{AuthRepository, PasswordHashPort, TokenHashPort, TokenPayload, TokenPort};
use crate::domain::auth::entities::{Identity, Session};
use crate::domain::error::AppError;
use crate::infrastructure::shared::unit_of_work::SharedTx;

use super::models::{IdentityRow, SessionRow};

pub struct Argon2Hasher;

impl PasswordHashPort for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal {
                message: format!("Password hashing failed: {e}"),
                source: None,
            })?
            .to_string();
        Ok(hash)
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::Internal {
            message: format!("Password hash parsing failed: {e}"),
            source: None,
        })?;
        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(AppError::Internal {
                message: format!("Password verification failed: {e}"),
                source: None,
            }),
        }
    }
}

pub struct Sha256TokenHasher;

impl TokenHashPort for Sha256TokenHasher {
    fn hash(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub struct JwtEncoder {
    jwt_secret: String,
}

impl JwtEncoder {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
        }
    }
}

impl TokenPort for JwtEncoder {
    fn encode(&self, sub: &str, iat: usize, exp: usize) -> Result<String, AppError> {
        let payload = TokenPayload {
            sub: sub.to_string(),
            exp,
            iat,
        };
        encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal {
            message: "JWT encoding failed".into(),
            source: Some(Box::new(e)),
        })
    }

    fn decode(&self, token: &str) -> Result<TokenPayload, AppError> {
        let token_data = decode::<TokenPayload>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Internal {
            message: "JWT decoding failed".into(),
            source: Some(Box::new(e)),
        })?;
        Ok(token_data.claims)
    }
}

#[derive(Clone)]
pub struct SqliteAuthRepository {
    tx: SharedTx,
}

impl SqliteAuthRepository {
    pub fn new(tx: SharedTx) -> Self {
        Self { tx }
    }
}

impl AuthRepository for SqliteAuthRepository {
    async fn find_identity_by_email(&self, email: &str) -> Result<Option<Identity>, AppError> {
        let mut tx = self.tx.lock().await;
        let row: Option<IdentityRow> =
            sqlx::query_as("SELECT id, user_id, email, password_hash FROM identities WHERE email = ?")
                .bind(email)
                .fetch_optional(&mut **tx)
                .await
                .map_err(|e| AppError::Internal {
                    message: "Database error".into(),
                    source: Some(Box::new(e)),
                })?;

        Ok(row.map(|r| Identity {
            id: r.id,
            user_id: r.user_id,
            email: r.email,
            password_hash: r.password_hash,
        }))
    }

    async fn create_identity(
        &self,
        id: &str,
        user_id: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("INSERT INTO identities (id, user_id, email, password_hash) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(email)
            .bind(password_hash)
            .execute(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(())
    }

    async fn create_session(
        &self,
        id: &str,
        identity_id: &str,
        token_hash: &str,
        expires_at: i64,
    ) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("INSERT INTO sessions (id, identity_id, token_hash, expires_at) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(identity_id)
            .bind(token_hash)
            .bind(expires_at)
            .execute(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(())
    }

    async fn find_session_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<Session>, AppError> {
        let mut tx = self.tx.lock().await;
        let row: Option<SessionRow> =
            sqlx::query_as("SELECT id, identity_id, token_hash, expires_at FROM sessions WHERE token_hash = ?")
                .bind(token_hash)
                .fetch_optional(&mut **tx)
                .await
                .map_err(|e| AppError::Internal {
                    message: "Database error".into(),
                    source: Some(Box::new(e)),
                })?;

        Ok(row.map(|r| Session {
            id: r.id,
            identity_id: r.identity_id,
            token_hash: r.token_hash,
            expires_at: r.expires_at,
        }))
    }

    async fn delete_session(&self, id: &str) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(id)
            .execute(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(())
    }

    async fn find_identity_by_id(&self, id: &str) -> Result<Option<Identity>, AppError> {
        let mut tx = self.tx.lock().await;
        let row: Option<IdentityRow> =
            sqlx::query_as("SELECT id, user_id, email, password_hash FROM identities WHERE id = ?")
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .map_err(|e| AppError::Internal {
                    message: "Database error".into(),
                    source: Some(Box::new(e)),
                })?;

        Ok(row.map(|r| Identity {
            id: r.id,
            user_id: r.user_id,
            email: r.email,
            password_hash: r.password_hash,
        }))
    }
}
