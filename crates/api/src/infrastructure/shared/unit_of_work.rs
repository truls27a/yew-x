use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Sqlite, SqlitePool, Transaction};
use tokio::sync::Mutex;

use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::infrastructure::auth::adapters::SqliteAuthRepository;
use crate::infrastructure::comments::adapters::SqliteCommentRepository;
use crate::infrastructure::notifications::adapters::SqliteNotificationRepository;
use crate::infrastructure::tweets::adapters::SqliteTweetRepository;
use crate::infrastructure::users::adapters::SqliteUserRepository;

pub type SharedTx = Arc<Mutex<Transaction<'static, Sqlite>>>;

#[derive(Clone)]
pub struct DatabaseClient {
    pool: SqlitePool,
}

impl DatabaseClient {
    pub async fn create() -> Result<Self, AppError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:data.db?mode=rwc")
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        sqlx::migrate!().run(&pool).await.map_err(|e| AppError::Internal {
            message: "Migration error".into(),
            source: Some(Box::new(e)),
        })?;
        Ok(Self { pool })
    }

    pub async fn get_session(&self) -> Result<Transaction<'static, Sqlite>, AppError> {
        self.pool.begin().await.map_err(|e| AppError::Internal {
            message: "Database error".into(),
            source: Some(Box::new(e)),
        })
    }
}

pub struct SqliteUnitOfWork {
    tx: SharedTx,
}

impl SqliteUnitOfWork {
    pub async fn new(db: &DatabaseClient) -> Result<Self, AppError> {
        let tx = db.get_session().await?;
        Ok(Self {
            tx: Arc::new(Mutex::new(tx)),
        })
    }
}

impl UnitOfWork for SqliteUnitOfWork {
    type UserRepo = SqliteUserRepository;
    type TweetRepo = SqliteTweetRepository;
    type NotificationRepo = SqliteNotificationRepository;
    type AuthRepo = SqliteAuthRepository;
    type CommentRepo = SqliteCommentRepository;

    fn users(&self) -> SqliteUserRepository {
        SqliteUserRepository::new(self.tx.clone())
    }

    fn tweets(&self) -> SqliteTweetRepository {
        SqliteTweetRepository::new(self.tx.clone())
    }

    fn notifications(&self) -> SqliteNotificationRepository {
        SqliteNotificationRepository::new(self.tx.clone())
    }

    fn auth(&self) -> SqliteAuthRepository {
        SqliteAuthRepository::new(self.tx.clone())
    }

    fn comments(&self) -> SqliteCommentRepository {
        SqliteCommentRepository::new(self.tx.clone())
    }

    async fn commit(self) -> Result<(), AppError> {
        let tx = Arc::try_unwrap(self.tx)
            .map_err(|_| AppError::Internal {
                message: "Transaction still referenced".into(),
                source: None,
            })?
            .into_inner();
        tx.commit().await.map_err(|e| AppError::Internal {
            message: "Database error".into(),
            source: Some(Box::new(e)),
        })?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), AppError> {
        let tx = Arc::try_unwrap(self.tx)
            .map_err(|_| AppError::Internal {
                message: "Transaction still referenced".into(),
                source: None,
            })?
            .into_inner();
        tx.rollback().await.map_err(|e| AppError::Internal {
            message: "Database error".into(),
            source: Some(Box::new(e)),
        })?;
        Ok(())
    }
}
