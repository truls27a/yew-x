use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Sqlite, SqlitePool, Transaction};
use tokio::sync::Mutex;

use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::infrastructure::auth::repositories::SqliteAuthRepository;
use crate::infrastructure::notifications::repositories::SqliteNotificationRepository;
use crate::infrastructure::tweets::repositories::SqliteTweetRepository;
use crate::infrastructure::users::repositories::SqliteUserRepository;

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
            .await?;
        init_db(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn get_session(&self) -> Result<Transaction<'static, Sqlite>, AppError> {
        Ok(self.pool.begin().await?)
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

    async fn commit(self) -> Result<(), AppError> {
        let tx = Arc::try_unwrap(self.tx)
            .map_err(|_| AppError::Internal {
                message: "Transaction still referenced".into(),
                source: None,
            })?
            .into_inner();
        tx.commit().await?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), AppError> {
        let tx = Arc::try_unwrap(self.tx)
            .map_err(|_| AppError::Internal {
                message: "Transaction still referenced".into(),
                source: None,
            })?
            .into_inner();
        tx.rollback().await?;
        Ok(())
    }
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            handle TEXT NOT NULL UNIQUE,
            avatar_url TEXT NOT NULL,
            bio TEXT NOT NULL DEFAULT '',
            followers INTEGER NOT NULL DEFAULT 0,
            following INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tweets (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id),
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tweet_likes (
            tweet_id TEXT NOT NULL REFERENCES tweets(id),
            user_id TEXT NOT NULL REFERENCES users(id),
            PRIMARY KEY (tweet_id, user_id)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notifications (
            id TEXT PRIMARY KEY,
            notification_type TEXT NOT NULL,
            actor_id TEXT NOT NULL REFERENCES users(id),
            target_user_id TEXT NOT NULL REFERENCES users(id),
            content TEXT,
            created_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id),
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            identity_id TEXT NOT NULL REFERENCES identities(id),
            token_hash TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
    )
    .execute(pool)
    .await?;

    // Seed data only if users table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        seed_data(pool).await?;
    }

    Ok(())
}

async fn seed_data(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Users
    sqlx::query(
        "INSERT INTO users (id, display_name, handle, avatar_url, bio, followers, following) VALUES
        ('alice', 'Alice Johnson', 'alice', 'https://i.pravatar.cc/150?u=alice', 'Software engineer. Rust enthusiast.', 1200, 340),
        ('bob', 'Bob Smith', 'bob', 'https://i.pravatar.cc/150?u=bob', 'Designer & frontend dev.', 850, 220),
        ('carol', 'Carol Chen', 'carol', 'https://i.pravatar.cc/150?u=carol', 'Open source contributor. Coffee addict.', 3400, 510)"
    )
    .execute(pool)
    .await?;

    // Tweets
    sqlx::query(
        "INSERT INTO tweets (id, user_id, content, created_at) VALUES
        ('t1', 'alice', 'Just shipped a new feature in Rust! The borrow checker is my best friend now.', unixepoch('now', '-2 hours')),
        ('t2', 'bob', 'Tailwind CSS makes prototyping so fast. Dark mode looks incredible with just a few utility classes.', unixepoch('now', '-4 hours')),
        ('t3', 'carol', 'WebAssembly is the future of the web. Change my mind.', unixepoch('now', '-6 hours')),
        ('t4', 'alice', 'Hot take: Yew is the best frontend framework. Type safety all the way down.', unixepoch('now', '-8 hours')),
        ('t5', 'carol', 'Just hit 1000 contributions on GitHub this year! Open source is incredibly rewarding.', unixepoch('now', '-12 hours'))"
    )
    .execute(pool)
    .await?;

    Ok(())
}
