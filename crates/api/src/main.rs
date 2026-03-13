mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use application::auth::ports::TokenPort;
use application::auth::use_cases as auth_uc;
use application::notifications::use_cases as notif_uc;
use application::tweets::use_cases as tweet_uc;
use application::users::use_cases as user_uc;
use infrastructure::auth::adapters::{Argon2Hasher, JwtEncoder, Sha256TokenHasher};
use infrastructure::shared::time::UtcClock;
use infrastructure::shared::unit_of_work::DatabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseClient,
    pub token_port: Arc<dyn TokenPort>,
    // Auth
    pub register: auth_uc::Register,
    pub login: auth_uc::Login,
    pub refresh: auth_uc::Refresh,
    // Tweets
    pub get_tweets: tweet_uc::GetTweets,
    pub get_tweet: tweet_uc::GetTweet,
    pub create_tweet: tweet_uc::CreateTweet,
    pub get_user_tweets: tweet_uc::GetUserTweets,
    pub toggle_like: tweet_uc::ToggleLike,
    // Users
    pub get_user: user_uc::GetUser,
    // Notifications
    pub get_notifications: notif_uc::GetNotifications,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db = DatabaseClient::create().await?;

    let jwt_secret =
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-do-not-use-in-prod".to_string());

    let password_hasher: Arc<dyn application::auth::ports::PasswordHashPort> =
        Arc::new(Argon2Hasher);
    let token_hasher: Arc<dyn application::auth::ports::TokenHashPort> =
        Arc::new(Sha256TokenHasher);
    let token_port: Arc<dyn TokenPort> = Arc::new(JwtEncoder::new(&jwt_secret));
    let clock: Arc<dyn application::shared::time::Clock> = Arc::new(UtcClock);

    let state = AppState {
        db,
        token_port: token_port.clone(),
        register: auth_uc::Register::new(
            password_hasher.clone(),
            token_hasher.clone(),
            token_port.clone(),
            clock.clone(),
        ),
        login: auth_uc::Login::new(
            password_hasher.clone(),
            token_hasher.clone(),
            token_port.clone(),
            clock.clone(),
        ),
        refresh: auth_uc::Refresh::new(token_hasher.clone(), token_port.clone(), clock.clone()),
        get_tweets: tweet_uc::GetTweets::new(),
        get_tweet: tweet_uc::GetTweet::new(),
        create_tweet: tweet_uc::CreateTweet::new(),
        get_user_tweets: tweet_uc::GetUserTweets::new(),
        toggle_like: tweet_uc::ToggleLike::new(),
        get_user: user_uc::GetUser::new(),
        get_notifications: notif_uc::GetNotifications::new(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(api::routes::health::health))
        .route("/api/tweets", get(api::routes::tweets::list_tweets))
        .route("/api/tweets", post(api::routes::tweets::create))
        .route(
            "/api/tweets/{id}",
            get(api::routes::tweets::get_single_tweet),
        )
        .route("/api/tweets/{id}/like", post(api::routes::tweets::like))
        .route(
            "/api/users/{id}",
            get(api::routes::users::get_single_user),
        )
        .route(
            "/api/users/{id}/tweets",
            get(api::routes::users::get_user_tweets_handler),
        )
        .route(
            "/api/notifications",
            get(api::routes::notifications::list_notifications),
        )
        .route(
            "/api/auth/register",
            post(api::routes::auth::register),
        )
        .route("/api/auth/login", post(api::routes::auth::login))
        .route(
            "/api/auth/refresh",
            post(api::routes::auth::refresh),
        )
        .route("/api/auth/me", get(api::routes::auth::me))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Serving on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
