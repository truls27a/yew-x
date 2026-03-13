use crate::application::auth::ports::AuthRepository;
use crate::domain::auth::entities::{Identity, Session};
use crate::domain::error::AppError;
use crate::infrastructure::shared::unit_of_work::SharedTx;

use super::models::{IdentityRow, SessionRow};

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
                .await?;

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
            .await?;
        Ok(())
    }

    async fn create_session(
        &self,
        id: &str,
        identity_id: &str,
        token_hash: &str,
        expires_at: &str,
    ) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("INSERT INTO sessions (id, identity_id, token_hash, expires_at) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(identity_id)
            .bind(token_hash)
            .bind(expires_at)
            .execute(&mut **tx)
            .await?;
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
                .await?;

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
            .await?;
        Ok(())
    }

    async fn find_identity_by_id(&self, id: &str) -> Result<Option<Identity>, AppError> {
        let mut tx = self.tx.lock().await;
        let row: Option<IdentityRow> =
            sqlx::query_as("SELECT id, user_id, email, password_hash FROM identities WHERE id = ?")
                .bind(id)
                .fetch_optional(&mut **tx)
                .await?;

        Ok(row.map(|r| Identity {
            id: r.id,
            user_id: r.user_id,
            email: r.email,
            password_hash: r.password_hash,
        }))
    }
}
