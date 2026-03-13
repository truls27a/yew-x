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
    pub register_use_case: auth_uc::RegisterUseCase,
    pub login_use_case: auth_uc::LoginUseCase,
    pub refresh_use_case: auth_uc::RefreshUseCase,
    // Tweets
    pub get_tweets_use_case: tweet_uc::GetTweetsUseCase,
    pub get_tweet_use_case: tweet_uc::GetTweetUseCase,
    pub create_tweet_use_case: tweet_uc::CreateTweetUseCase,
    pub get_user_tweets_use_case: tweet_uc::GetUserTweetsUseCase,
    pub toggle_like_use_case: tweet_uc::ToggleLikeUseCase,
    // Users
    pub get_user_use_case: user_uc::GetUserUseCase,
    // Notifications
    pub get_notifications_use_case: notif_uc::GetNotificationsUseCase,
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
        register_use_case: auth_uc::RegisterUseCase::new(
            password_hasher.clone(),
            token_hasher.clone(),
            token_port.clone(),
            clock.clone(),
        ),
        login_use_case: auth_uc::LoginUseCase::new(
            password_hasher.clone(),
            token_hasher.clone(),
            token_port.clone(),
            clock.clone(),
        ),
        refresh_use_case: auth_uc::RefreshUseCase::new(token_hasher.clone(), token_port.clone(), clock.clone()),
        get_tweets_use_case: tweet_uc::GetTweetsUseCase::new(),
        get_tweet_use_case: tweet_uc::GetTweetUseCase::new(),
        create_tweet_use_case: tweet_uc::CreateTweetUseCase::new(),
        get_user_tweets_use_case: tweet_uc::GetUserTweetsUseCase::new(),
        toggle_like_use_case: tweet_uc::ToggleLikeUseCase::new(),
        get_user_use_case: user_uc::GetUserUseCase::new(),
        get_notifications_use_case: notif_uc::GetNotificationsUseCase::new(),
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
