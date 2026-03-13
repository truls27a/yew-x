mod api;
mod application;
mod domain;
mod infrastructure;

use axum::routing::{get, post};
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{Any, CorsLayer};

use infrastructure::repositories::{
    notification_repository::SqliteNotificationRepository,
    tweet_repository::SqliteTweetRepository,
    user_repository::SqliteUserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: SqliteUserRepository,
    pub tweet_repo: SqliteTweetRepository,
    pub notification_repo: SqliteNotificationRepository,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:data.db?mode=rwc")
        .await?;

    infrastructure::database::init_db(&pool).await?;

    let state = AppState {
        user_repo: SqliteUserRepository::new(pool.clone()),
        tweet_repo: SqliteTweetRepository::new(pool.clone()),
        notification_repo: SqliteNotificationRepository::new(pool),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(api::routes::health::health))
        .route("/api/tweets", get(api::routes::tweets::list_tweets))
        .route("/api/tweets", post(api::routes::tweets::create))
        .route("/api/tweets/{id}", get(api::routes::tweets::get_single_tweet))
        .route("/api/tweets/{id}/like", post(api::routes::tweets::like))
        .route("/api/users/{id}", get(api::routes::users::get_single_user))
        .route(
            "/api/users/{id}/tweets",
            get(api::routes::users::get_user_tweets_handler),
        )
        .route(
            "/api/notifications",
            get(api::routes::notifications::list_notifications),
        )
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Serving on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
